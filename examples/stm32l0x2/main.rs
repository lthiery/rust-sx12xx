#![cfg_attr(not(test), no_std)]
#![no_main]

// To use example, press any key in serial terminal
// Packet will send and "Transmit Done!" will print when radio is done sending packet

extern crate nb;
extern crate panic_halt;

use core::fmt::Write;
use rtfm::app;
use stm32l0xx_hal::exti::{ExtiLine, GpioLine};
use stm32l0xx_hal::serial;
use stm32l0xx_hal::serial::USART2 as DebugUsart;
use stm32l0xx_hal::{exti::Exti, prelude::*, rcc, rng::Rng, syscfg};
use sx12xx;
use sx12xx::{ Sx12xx, LoRaBandwidth, LoRaSpreadingFactor, LoRaCodingRate };
mod bindings;
pub use bindings::initialize_irq as initialize_radio_irq;
pub use bindings::RadioIRQ;
pub use bindings::TcxoEn;

const OUI: u32 = 1;
const DEVICE_ID: u16 = 3;
const PRESHARED_KEY: [u8; 16] = [
    0x7B, 0x60, 0xC0, 0xF0, 0x77, 0x51, 0x50, 0xD3, 0x2, 0xCE, 0xAE, 0x50, 0xA0, 0xD2, 0x11, 0xC1,
];

#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        int: Exti,
        radio_irq: RadioIRQ,
        debug_uart: serial::Tx<DebugUsart>,
        uart_rx: serial::Rx<DebugUsart>,
        #[init([0;512])]
        buffer: [u8; 512],
        #[init(0)]
        count: u8,
        sx12xx: Sx12xx,
    }

    #[init(spawn = [send_ping], resources = [buffer])]
    fn init(ctx: init::Context) -> init::LateResources {
        let device = ctx.device;
        let mut rcc = device.RCC.freeze(rcc::Config::hsi16());
        let mut syscfg = syscfg::SYSCFG::new(device.SYSCFG, &mut rcc);

        let gpioa = device.GPIOA.split(&mut rcc);
        let gpiob = device.GPIOB.split(&mut rcc);
        let gpioc = device.GPIOC.split(&mut rcc);

        let (tx_pin, rx_pin, serial_peripheral) = (gpioa.pa2, gpioa.pa3, device.USART2);

        let mut serial = serial_peripheral
            .usart(tx_pin, rx_pin, serial::Config::default(), &mut rcc)
            .unwrap();

        // listen for incoming bytes which will trigger transmits
        serial.listen(serial::Event::Rxne);
        let (mut tx, rx) = serial.split();

        write!(tx, "LongFi Device Test\r\n").unwrap();

        let mut exti = Exti::new(device.EXTI);
        // constructor initializes 48 MHz clock that RNG requires
        // Initialize 48 MHz clock and RNG
        let hsi48 = rcc.enable_hsi48(&mut syscfg, device.CRS);
        let rng = Rng::new(device.RNG, &mut rcc, hsi48);
        let radio_irq = initialize_radio_irq(gpiob.pb4, &mut syscfg, &mut exti);

        let bindings = bindings::new(
            device.SPI1,
            &mut rcc,
            rng,
            gpiob.pb3,
            gpioa.pa6,
            gpioa.pa7,
            gpioa.pa15,
            gpioc.pc0,
            gpioa.pa1,
            gpioc.pc2,
            gpioc.pc1,
        );

        let sx12xx = Sx12xx::new(sx12xx::Radio::sx1276(), bindings);

        write!(tx, "Going to main loop\r\n").unwrap();

        // Return the initialised resources.
        init::LateResources {
            int: exti,
            radio_irq: radio_irq,
            debug_uart: tx,
            uart_rx: rx,
            sx12xx
        }
    }

    #[task(capacity = 4, priority = 2, resources = [debug_uart, buffer, sx12xx])]
    fn radio_event(ctx: radio_event::Context, event: sx12xx::Event) {
        let sx12xx = ctx.resources.sx12xx;
        let state = sx12xx.handle_event(event);
        let debug = ctx.resources.debug_uart;

        match state {
            sx12xx::State::Sx12xxState_TxDone => {
                write!(debug, "Transmit Done!\r\n").unwrap();
            }
            sx12xx::State::Sx12xxState_RxDone => {
                // get receive buffer
                // let rx_packet = longfi_radio.get_rx();
                // write!(debug, "Received packet\r\n").unwrap();
                // write!(debug, "  Length =  {}\r\n", rx_packet.len).unwrap();
                // write!(debug, "  Rssi   = {}\r\n", rx_packet.rssi).unwrap();
                // write!(debug, "  Snr    =  {}\r\n", rx_packet.snr).unwrap();
                // unsafe {
                //     for i in 0..rx_packet.len {
                //         write!(debug, "{:X} ", *rx_packet.buf.offset(i as isize)).unwrap();
                //     }
                //     write!(debug, "\r\n").unwrap();
                // }
                // // give buffer back to library
                // longfi_radio.set_buffer(ctx.resources.buffer);
            }
            sx12xx::State::Sx12xxState_Busy => {}
            sx12xx::State::Sx12xxState_RxError => {}
            sx12xx::State::Sx12xxState_RxTimeout => {}
            sx12xx::State::Sx12xxState_TxTimeout => {
                panic!("TxTimeout not handle");
            }
        }
    }

    #[task(capacity = 4, priority = 2, resources = [debug_uart, count, sx12xx])]
    fn send_ping(ctx: send_ping::Context) {
        write!(ctx.resources.debug_uart, "Sending Ping\r\n").unwrap();

        let packet: [u8; 128] = [0; 128];//[0xDE, 0xAD, 0xBE, 0xEF, *ctx.resources.count];
        *ctx.resources.count += 1;

        let sx12xx = ctx.resources.sx12xx;
        sx12xx.set_lora_tx_config(
            22,
            LoRaBandwidth::_125KHZ,
            LoRaSpreadingFactor::_10,
            LoRaCodingRate::_4_5
        );

        sx12xx.set_frequency(902700000);
        sx12xx.send(&packet);
    }

    #[task(binds = USART2, priority=1, resources = [uart_rx], spawn = [send_ping])]
    fn USART2(ctx: USART2::Context) {
        let rx = ctx.resources.uart_rx;
        rx.read().unwrap();
        ctx.spawn.send_ping().unwrap();
    }

    #[task(binds = EXTI4_15, priority = 1, resources = [radio_irq, int], spawn = [radio_event])]
    fn EXTI4_15(ctx: EXTI4_15::Context) {
        Exti::unpend(GpioLine::from_raw_line(ctx.resources.radio_irq.pin_number()).unwrap());
        ctx.spawn.radio_event(sx12xx::Event::Sx12xxEvent_DIO0).unwrap();
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART4_USART5();
    }
};
