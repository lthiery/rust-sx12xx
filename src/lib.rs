#![cfg_attr(not(test), no_std)]
use sx12xx_sys::*;

extern crate libm;

pub use sx12xx_sys::AntPinsMode_t as AntPinsMode;
pub use sx12xx_sys::BoardBindings_t as BoardBindings;
pub use sx12xx_sys::Sx12xxEvent_t as Event;
use sx12xx_sys::Sx12xxState_t as Sx12xxState;
use sx12xx_sys::Sx12xxRxMetadata_t as RxMetadata;

pub struct RxQuality {
    rssi: i16,
    snr: i8,
}

impl RxQuality {
    pub fn get_rssi(&self)-> i16 {
        self.rssi
    }

    pub fn get_snr(&self)-> i8 {
        self.snr
    }
}

pub enum State {
    Busy,
    TxDone,
    RxDone(RxQuality),
    TxTimeout,
    RxTimeout,
    RxError,
}

use heapless::consts::*;
use heapless::Vec;

pub struct Radio {
    c_handle: Radio_t,
}

enum HopPeriod {
    Disabled,
    Enabled(u8),
}

struct IqInverted {
    rx: bool,
    tx: bool,
}

struct Settings {
    iq_inverted: IqInverted,
    hop_period: HopPeriod,
    crc_on: bool,
    fix_len: bool,
    preamble_len: u16,
    timeout: u16, // timeout in FSK bytes or LoRa symboles,
    continuous_rx: bool,
}

pub struct Sx12xx {
    settings: Settings,
    radio: Radio,
    buffer: Vec<u8, U256>,
}

impl Radio {
    pub fn sx1262() -> Radio {
        Radio {
            c_handle: unsafe { SX126xRadioNew() },
        }
    }
    pub fn sx1276() -> Radio {
        Radio {
            c_handle: unsafe { SX1276RadioNew() },
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NoRadioPointer,
}

pub enum LoRaBandwidth {
    _125KHZ = 0,
    _250KHZ = 1,
    _500KHZ = 2,
    _RESERVED = 3,
}

pub enum LoRaSpreadingFactor {
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
}

pub enum LoRaCodingRate {
    _4_5 = 1,
    _4_6 = 2,
    _4_7 = 3,
    _4_8 = 4,
}

impl Sx12xx {
    
    pub fn new(mut radio: Radio, bindings: BoardBindings) -> Sx12xx {
        unsafe {
            sx12xx_init(&mut radio.c_handle, bindings);
        };

        Sx12xx {
            radio,
            settings: Settings {
                iq_inverted: IqInverted {
                    tx: false,
                    rx: true,
                },
                hop_period: HopPeriod::Disabled,
                crc_on: true,
                fix_len: false,
                preamble_len: 8,
                timeout: 5,
                continuous_rx: true
            },
            buffer: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> State {
        let sx12xx_state = unsafe { sx12xx_handle_event(event) };

        match sx12xx_state {
            Sx12xxState::Sx12xxState_Busy => State::Busy,
            Sx12xxState::Sx12xxState_TxDone =>  {
                self.buffer.clear();
                State::TxDone
            },
            Sx12xxState::Sx12xxState_RxDone => {
                let metadata = unsafe { sx12xx_get_rx_metadata() };
                self.buffer.resize(metadata.rx_len as usize, 0);
                State::RxDone(RxQuality{
                    snr: metadata.snr,
                    rssi: metadata.rssi
                })
            },
            Sx12xxState::Sx12xxState_TxTimeout => State::TxTimeout,
            Sx12xxState::Sx12xxState_RxTimeout => State::RxTimeout,
            Sx12xxState::Sx12xxState_RxError => State::RxError,
        }
    }

    pub fn get_buffer(&mut self) -> &Vec<u8, U256> {
        &mut self.buffer
    }

    pub fn get_mut_buffer(&mut self) -> &mut Vec<u8, U256> {
        &mut self.buffer
    }

    pub fn send_buffer(&mut self) {
        unsafe {
            if let Some(send) = self.radio.c_handle.Send {
                send(self.buffer.as_mut_ptr(), self.buffer.len() as u8);
            }
        };
    }
    pub fn send(&mut self, buffer: &[u8]) {
        self.buffer.extend(buffer);
        self.send_buffer();
    }

    pub fn configure_fsk_tx(&mut self, power: i8, fdev: u32, datarate: u32) {
        unsafe {
            if let Some(set_tx_config) = self.radio.c_handle.SetTxConfig {
                set_tx_config(
                    RadioModems_t_MODEM_FSK,
                    power,
                    fdev,
                    0,
                    datarate,
                    0,
                    self.settings.preamble_len,
                    self.settings.fix_len,
                    self.settings.crc_on,
                    false,
                    0,
                    false,
                    0,
                )
            }
        };
    }

    pub fn configure_lora_tx(
        &mut self,
        power: i8,
        bandwidth: LoRaBandwidth,
        datarate: LoRaSpreadingFactor,
        coderate: LoRaCodingRate,
    ) {
        let (freq_hop_on, hop_period) = if let HopPeriod::Enabled(period) = self.settings.hop_period
        {
            (true, period)
        } else {
            (false, 0)
        };

        unsafe {
            if let Some(set_tx_config) = self.radio.c_handle.SetTxConfig {
                set_tx_config(
                    RadioModems_t_MODEM_LORA,   // modem
                    power,                      // power
                    0,                          // fdev (is always 0 for LoRa)
                    bandwidth as u32,           // bandwidth
                    datarate as u32,            // datarate
                    coderate as u8,             // coding rate
                    self.settings.preamble_len, // preamble len
                    self.settings.fix_len,      // fix length packet
                    self.settings.crc_on,       // crc setting
                    freq_hop_on,                // frequency hop setting
                    hop_period,                 // number of symbols before hop
                    self.settings.iq_inverted.tx,  // inverted iq
                    3000,                       // transmission timeout
                );
            }
        };
    }


    pub fn configure_lora_rx(
        &mut self,
        bandwidth: LoRaBandwidth,
        datarate: LoRaSpreadingFactor,
        coderate: LoRaCodingRate,
    ) {
        let (freq_hop_on, hop_period) = if let HopPeriod::Enabled(period) = self.settings.hop_period
        {
            (true, period)
        } else {
            (false, 0)
        };

        unsafe {
            if let Some(set_rx_config) = self.radio.c_handle.SetRxConfig {
                set_rx_config(
                    RadioModems_t_MODEM_LORA,       // modem
                    bandwidth as u32,               // bandwidth
                    datarate as u32,                // datarate
                    coderate as u8,                 // coding rate
                    0, // bandwidth afc (FSK only)
                    self.settings.preamble_len,     // preamble len
                    self.settings.timeout,
                    self.settings.fix_len,          // fix length packet,
                    0, // packet length if fixed
                    self.settings.crc_on,           // crc setting
                    freq_hop_on,                    // frequency hop setting
                    hop_period,                     // number of symbols before hop
                    self.settings.iq_inverted.rx,      // inverted iq
                    self.settings.continuous_rx,    // transmission timeout
                );
            }
        };
    }

    pub fn set_rx(
        &mut self,
    ) {

        // we resize the buffer to max size allowing the C library to write to it
        unsafe {
            self.buffer.resize(255, 0);
            sx12xx_set_rx_buffer(self.buffer.as_mut_ptr(), 255);
        };

        unsafe {
            if let Some(rx) = self.radio.c_handle.Rx {
                rx(0)
            }
        };
    }


    pub fn set_frequency(&mut self, frequency_mhz: u32) {
        unsafe {
            if let Some(set_channel) = self.radio.c_handle.SetChannel {
                set_channel(frequency_mhz);
            }
        };
    }

    pub fn set_public_network(&mut self, enable: bool) {
        unsafe {
            if let Some(set_public_network) = self.radio.c_handle.SetPublicNetwork {
                set_public_network(enable);
            }
        };
    }

    pub fn enable_hop_period(&mut self, period: u8) {
        self.settings.hop_period = HopPeriod::Enabled(period);
    }

    pub fn disable_hop_period(&mut self) {
        self.settings.hop_period = HopPeriod::Disabled;
    }

    pub fn set_iq_invert_tx(&mut self, set: bool) {
        self.settings.iq_inverted.tx = set;
    }

    pub fn set_iq_invert_rx(&mut self, set: bool) {
        self.settings.iq_inverted.rx = set;
    }

    // timeout in FSK bytes or LoRa symboles
    pub fn set_timeout(&mut self, timeout: u16) {
        self.settings.timeout = timeout;
    }
}

#[no_mangle]
pub extern "C" fn ceil(expr: f64) -> f64 {
    libm::ceil(expr)
}

#[no_mangle]
pub extern "C" fn round(expr: f64) -> f64 {
    libm::round(expr)
}

#[no_mangle]
pub extern "C" fn floor(expr: f64) -> f64 {
    libm::floor(expr)
}
