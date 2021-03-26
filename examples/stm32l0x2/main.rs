#![cfg_attr(not(test), no_std)]
#![no_main]

// To use example, press any key in serial terminal
// Packet will send and "Transmit Done!" will print when radio is done sending packet

extern crate nb;
extern crate panic_ramdump;

use core::fmt::Write;
use lorawan_crypto::LorawanCrypto as Crypto;
use lorawan_device::{
    radio, Device as LorawanDevice, Error as LorawanError, Event as LorawanEvent,
    Response as LorawanResponse, region, region::RegionHandler
};
use rtic::app;
use stm32l0xx_hal::{
    exti::Exti,
    exti::{ExtiLine, GpioLine},
    pac,
    pac::Interrupt,
    prelude::*,
    rcc, rng,
    rng::Rng,
    serial,
    serial::Serial1Ext,
    syscfg,
    timer::Timer,
    time::Bps
};
use sx12xx::{self, LorawanRadio, Sx12xx};
mod bindings;
pub use bindings::initialize_irq as initialize_radio_irq;
pub use bindings::RadioIRQ;
pub use bindings::TcxoEn;

// catena-4610
use stm32l0xx_hal::serial::USART1 as DebugUsart;

// lrwan1-disco
//use stm32l0xx_hal::serial::USART1 as DebugUsart;

static mut RNG: Option<rng::Rng> = None;
fn get_random_u32() -> u32 {
    unsafe {
        if let Some(rng) = &mut RNG {

            // enable starts the ADC conversions that generate the random number
            rng.enable();
            // wait until the flag flips; interrupt driven is possible but no implemented
            rng.wait();
            // reading the result clears the ready flag
            let val = rng.take_result();
            // can save some power by disabling until next random number needed
            rng.disable();
            val
        } else {
            panic!("No Rng exists!");
        }
    }
}

pub struct TimerContext {
    pub target: u16,
    pub count: u16,
    pub enable: bool,
    pub armed: bool,
}

#[derive(Debug)]
enum Error {
    UsartRx
}

#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        int: Exti,
        radio_irq: RadioIRQ,
        debug_uart: serial::Tx<DebugUsart>,
        uart_rx: serial::Rx<DebugUsart>,
        timer: Timer<pac::TIM2>,
        #[init([0;512])]
        buffer: [u8; 512],
        #[init(false)]
        ready_to_send: bool,
        lorawan: Option<LorawanDevice<LorawanRadio, Crypto>>,
        #[init(TimerContext {
        target: 0,
        count: 0,
        enable: false,
        armed: false,
        })]
        timer_context: TimerContext,
    }

    #[init(spawn = [send_ping, lorawan_event], resources = [buffer])]
    fn init(ctx: init::Context) -> init::LateResources {
        let device = ctx.device;
        let mut rcc = device.RCC.freeze(rcc::Config::hsi16());
        let mut syscfg = syscfg::SYSCFG::new(device.SYSCFG, &mut rcc);
        let gpioa = device.GPIOA.split(&mut rcc);
        let gpiob = device.GPIOB.split(&mut rcc);
        let gpioc = device.GPIOC.split(&mut rcc);

        // catena-4610
        let (tx_pin, rx_pin, serial_peripheral) = (gpioa.pa9, gpioa.pa10, device.USART1);

        // lrwan1-disco
        //let (tx_pin, rx_pin, serial_peripheral) = (gpioa.pa2, gpioa.pa3, device.USART2);
        let serial_config = serial::Config::default().baudrate(Bps(115200));
        let mut serial = serial_peripheral
            .usart(tx_pin, rx_pin, serial_config , &mut rcc)
            .unwrap();

        // listen for incoming bytes which will trigger transmits
        serial.listen(serial::Event::Rxne);
        let (mut tx, rx) = serial.split();

        write!(tx, "LongFi Device Test\r\n").unwrap();

        let mut exti = Exti::new(device.EXTI);

        // constructor initializes 48 MHz clock that RNG requires
        // Initialize 48 MHz clock and RNG
        let hsi48 = rcc.enable_hsi48(&mut syscfg, device.CRS);
        unsafe { RNG = Some(Rng::new(device.RNG, &mut rcc, hsi48)) };
        let radio_irq = initialize_radio_irq(gpiob.pb4, &mut syscfg, &mut exti);

        // Configure the timer.
        let timer = device.TIM2.timer(1.khz(), &mut rcc);

        let bindings = bindings::new(
            device.SPI1,
            &mut rcc,
            gpiob.pb3,
            gpioa.pa6,
            gpioa.pa7,
            gpioa.pa15,
            gpioc.pc0,
            gpioa.pa1,
            gpioc.pc2,
            gpioc.pc1,
            Some(gpioa.pa8), // None //use pa8 for catena
        );

        let mut sx12xx = Sx12xx::new(sx12xx::Radio::sx1276(), bindings);
        sx12xx.set_public_network(true);

        let mut region = region::US915::default();
        region.set_subband(2);
        let lorawan = LorawanDevice::new(
            region.into(),
            LorawanRadio::new(sx12xx),
            [0x65, 0xFD, 0x86, 0x1A, 0xE7, 0x44, 0x89, 0xC0],
            [0x00, 0xE4, 0x56, 0x87, 0x9A, 0xB9, 0x3E, 0x76],
            [
                0xFB, 0x4B, 0x19, 0xE9, 0xF8, 0xD4, 0xB1, 0x50, 0x35, 0x76, 0xCE, 0x9B, 0xD8, 0x79, 0x9C, 0xD3
            ],
            get_random_u32,
        );

        ctx.spawn
            .lorawan_event(LorawanEvent::NewSessionRequest)
            .unwrap();

        write!(tx, "Going to main loop\r\n").unwrap();

        // Return the initialised resources.
        init::LateResources {
            int: exti,
            radio_irq,
            debug_uart: tx,
            uart_rx: rx,
            lorawan: Some(lorawan),
            timer,
        }
    }

    #[task(capacity = 4, priority = 2, resources = [debug_uart, buffer, lorawan])]
    fn print_error(ctx: print_error::Context, event: Error) {
        let debug = ctx.resources.debug_uart;
        write!(debug, "ERROR {:?}\r\n", event).unwrap();

    }

    #[task(capacity = 4, priority = 2, resources = [debug_uart, buffer, lorawan], spawn  = [lorawan_response])]
    fn lorawan_event(ctx: lorawan_event::Context, event: LorawanEvent<'static, LorawanRadio>) {
        let debug = ctx.resources.debug_uart;

        // The LoraWAN stack is a giant state machine which needs to mutate internally
        // We let that happen within RTIC's framework for shared statics
        // by using an Option cell that we can take() from
        if let Some(lorawan) = ctx.resources.lorawan.take() {
            // debug statements for the event
            match &event {
                LorawanEvent::NewSessionRequest => {
                    write!(debug, "New Session Request \r\n").unwrap();
                }
                LorawanEvent::RadioEvent(e) => match e {
                    radio::Event::TxRequest(_, _) => (),
                    radio::Event::RxRequest(_) => (),
                    radio::Event::CancelRx => (),
                    radio::Event::PhyEvent(phy) => {
                        let event = phy as &sx12xx::Event;
                        match event {
                            // The timing of DIO0 marks:
                            //          * when a packet was done being sent
                            //          * or when a packet was received
                            // Since every RxWindow is some amount of time after end of tx,
                            // we reset the timer to 0 on every transmit
                            sx12xx::Event::DIO0(t) => {
                                write!(debug, "Radio Rx/Tx (DIO0) Interrupt at {} ms\r\n", t)
                                    .unwrap()
                            }
                            _ => write!(debug, "\r\n").unwrap(),
                        }
                    }
                },
                LorawanEvent::TimeoutFired => (),
                LorawanEvent::SendDataRequest(_e) => {
                    write!(debug, "SendData \r\n").unwrap();
                }
            }
            let (new_state, response) = lorawan.handle_event(event);
            ctx.spawn.lorawan_response(response).unwrap();

            // placing back into the Option cell after taking is critical
            *ctx.resources.lorawan = Some(new_state);
        }
    }

    #[task(capacity = 4, priority = 2, resources = [debug_uart, timer_context, lorawan], spawn = [lorawan_event, send_ping])]
    fn lorawan_response(
        mut ctx: lorawan_response::Context,
        response: Result<LorawanResponse, LorawanError<LorawanRadio>>,
    ) {
        let debug = ctx.resources.debug_uart;
        match response {
            Ok(response) => match response {
                LorawanResponse::TimeoutRequest(ms) => {
                    ctx.resources.timer_context.lock(|context| {
                        context.target = ms as u16;
                        context.armed = true;
                    });
                    write!(debug, "TimeoutRequest: {:?}\r\n", ms).unwrap();
                }
                LorawanResponse::JoinSuccess => {
                    if let Some(lorawan) = ctx.resources.lorawan.take() {
                        write!(
                            debug,
                            "Join Success: {:?}\r\n",
                            lorawan.get_session_keys().unwrap()
                        )
                        .unwrap();

                        *ctx.resources.lorawan = Some(lorawan);
                    }
                    ctx.resources.timer_context.lock(|context| {
                        context.enable = false;
                    });

                    ctx.spawn.send_ping().unwrap();

                }
                LorawanResponse::ReadyToSend => {
                    write!(
                        debug,
                        "RxWindow expired but no ACK expected. Ready to Send\r\n"
                    )
                    .unwrap();
                    ctx.resources.timer_context.lock(|context| {
                        context.enable = false;
                    });

                    ctx.spawn.send_ping().unwrap();

                }
                LorawanResponse::DownlinkReceived(fcnt_down) => {
                    ctx.resources.timer_context.lock(|context| {
                        context.enable = false;
                    });

                    if let Some(mut lorawan) = ctx.resources.lorawan.take() {
                        if let Some(downlink) = lorawan.take_data_downlink() {
                            let fhdr = downlink.fhdr();
                            let fopts = fhdr.fopts();
                            use lorawan_encoding::parser::{DataHeader, FRMPayload};

                            if let Ok(FRMPayload::Data(data)) = downlink.frm_payload() {
                                write!(
                                    debug,
                                    "Downlink received \t\t(FCntDown={}\tFRM: {:?})",
                                    fcnt_down, data,
                                )
                                .unwrap();
                            } else {
                                write!(debug, "Downlink received \t\t(FcntDown={})", fcnt_down)
                                    .unwrap();
                            }

                            let mut mac_commands_len = 0;
                            for mac_command in fopts {
                                if mac_commands_len == 0 {
                                    write!(debug, "\tFOpts: {:?}", mac_command).unwrap();
                                } else {
                                    write!(debug, ",{:?}", mac_command).unwrap();
                                }
                                mac_commands_len += 1;
                            }

                            write!(debug, "\r\n").unwrap();

                        }
                        ctx.spawn.send_ping().unwrap();

                        // placing back into the Option cell after taking is critical
                        *ctx.resources.lorawan = Some(lorawan);
                    }
                }
                LorawanResponse::NoAck => {
                    write!(
                        debug,
                        "RxWindow expired, expected ACK to confirmed uplink not received\r\n"
                    )
                    .unwrap();
                    ctx.resources.timer_context.lock(|context| {
                        context.enable = false;
                    });
                    ctx.spawn.send_ping().unwrap();

                }
                LorawanResponse::NoJoinAccept => {
                    write!(debug, "No Join Accept Received\r\n").unwrap();
                    ctx.spawn
                        .lorawan_event(LorawanEvent::NewSessionRequest)
                        .unwrap();
                    ctx.resources.timer_context.lock(|context| {
                        context.enable = false;
                    });
                }
                LorawanResponse::SessionExpired => {
                    write!(debug, "SessionExpired. Created new Session\r\n").unwrap();
                    ctx.spawn
                        .lorawan_event(LorawanEvent::NewSessionRequest)
                        .unwrap();
                    ctx.resources.timer_context.lock(|context| {
                        context.enable = false;
                    });
                }
                LorawanResponse::NoUpdate => (),
                LorawanResponse::UplinkSending(fcnt_up) => {
                    write!(debug, "Uplink with FCnt {}\r\n", fcnt_up).unwrap();
                }
                LorawanResponse::JoinRequestSending => {
                    write!(debug, "Join Request Sending\r\n").unwrap();
                }
            },
            Err(err) => match err {
                LorawanError::Radio(_) => write!(debug, "Radio \r\n").unwrap(),
                LorawanError::Session(e) => write!(debug, "Session {:?}\r\n", e).unwrap(),
                LorawanError::NoSession(_) => write!(debug, "NoSession\r\n").unwrap(),
            },
        }
    }

    #[task(capacity = 4, priority = 2, resources = [debug_uart, lorawan], spawn = [lorawan_response])]
    fn send_ping(ctx: send_ping::Context) {
        // The LoraWAN stack is a giant state machine which needs to mutate internally
        // We let that happen within RTIC's framework for shared statics
        // by using an Option cell that we can take() from
        if let Some(lorawan) = ctx.resources.lorawan.take() {
            let debug = ctx.resources.debug_uart;

            let ready_to_send = lorawan.ready_to_send_data();

            // placing back into the Option cell after take() is critical
            *ctx.resources.lorawan = Some(if ready_to_send {
                let fcnt_up = if let Some(fcnt) = lorawan.get_fcnt_up() {
                    fcnt
                } else {
                    0
                };
                let data: [u8; 5] = [0xDE, 0xAD, 0xBE, 0xEF, fcnt_up as u8];

                // requested confirmed packet every 4 packets
                let confirmed = true;// if fcnt_up % 4 == 0 {
                //     write!(debug, "Requesting Confirmed Uplink\r\n").unwrap();
                //     true
                // } else {
                //     write!(debug, "Requesting Unconfirmed Uplink\r\n").unwrap();
                //     true
                // };
                let (new_state, response) = lorawan.send(&data, 1, confirmed);
                ctx.spawn.lorawan_response(response).unwrap();
                new_state
            } else {
                write!(debug, "Suppressing Send Request\r\n").unwrap();
                lorawan
            });
        }
    }

    // catena-4610
    #[task(binds = USART1, priority=1, resources = [uart_rx], spawn = [send_ping, print_error])]
    fn USART1(ctx: USART1::Context) {
        let rx = ctx.resources.uart_rx;
        if let Err(_e) = rx.read() {
            ctx.spawn.print_error(Error::UsartRx).unwrap();
        }
        ctx.spawn.send_ping().unwrap();
    }

    // lrwan1-disco
    //#[task(binds = USART1, priority=1, resources = [uart_rx], spawn = [send_ping])]
    // fn USART2(ctx: USART2::Context) {
    //     let rx = ctx.resources.uart_rx;
    //     rx.read().unwrap();
    //     ctx.spawn.send_ping().unwrap();
    // }

    #[task(binds = EXTI4_15, priority = 3, resources = [radio_irq, int, timer_context], spawn = [lorawan_event])]
    fn EXTI4_15(ctx: EXTI4_15::Context) {
        Exti::unpend(GpioLine::from_raw_line(ctx.resources.radio_irq.pin_number()).unwrap());
        let context = ctx.resources.timer_context;
        let mut count = 0;

        // grab a lock on timer and start new
        if context.enable {
            count = context.count as u32;
        } else {
            context.target = 0xFFFF as u16;
            context.count = 0;
            context.enable = true;
        }

        rtic::pend(Interrupt::TIM2);

        ctx.spawn
            .lorawan_event(lorawan_device::Event::RadioEvent(
                lorawan_device::radio::Event::PhyEvent(sx12xx::Event::DIO0(count)),
            ))
            .unwrap();
    }

    // This is a pretty not scalable timeout implementation
    // but we can switch to RTICtimer queues later maybe
    #[task(binds = TIM2, priority = 3, resources = [timer, timer_context], spawn = [lorawan_event])]
    fn TIM2(ctx: TIM2::Context) {
        let context = ctx.resources.timer_context;
        let timer = ctx.resources.timer;
        let spawn = ctx.spawn;
        timer.clear_irq();

        // If timer has been disabled, another task
        // is asking to disarm the timer
        if !context.enable {
            context.target = 0;
            context.count = 0;
            context.armed = true;
            // Disable this interrupt from firing on ticks
            timer.unlisten();
        } else {
            // If count is 0, this vector fired because it
            // was spawned by another task
            // This means someone wants to start the timer
            if context.count == 0 {
                // Resets the underlying hardware timer
                // to avoid fractional ticks underway
                timer.reset();
                // Enables this interrupt to fire on ticks
                timer.listen();
            }
            context.count += 1;

            // if we have a match and timer is still armed to fire
            if context.count >= context.target && context.armed {
                spawn.lorawan_event(LorawanEvent::TimeoutFired).unwrap();
                context.armed = false;
            }
        }
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART4_USART5();
    }
};
