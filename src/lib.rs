#![cfg_attr(not(test), no_std)]
use sx12xx_sys::*;

use Sx12xx_t;
extern crate libm;

pub use sx12xx_sys::AntPinsMode_t as AntPinsMode;
pub use sx12xx_sys::BoardBindings_t as BoardBindings;
pub use sx12xx_sys::Sx12xxEvent_t as Event;
pub use sx12xx_sys::Sx12xxState_t as State;

use heapless::Vec;
use heapless::consts::*;

pub struct Radio {
    c_handle: Radio_t,
}

enum HopPeriod {
    Disabled,
    Enabled(u8),
}

struct Settings {
    iq_inverted: bool,
    hop_period: HopPeriod,
    crc_on: bool,
    fix_len: bool,
    preamble_len: u16
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
                iq_inverted: false,
                hop_period: HopPeriod::Disabled,
                crc_on: true,
                fix_len: false,
                preamble_len: 8
            },
            buffer: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> State {
        unsafe { sx12xx_handle_event(event) }
    }

    pub fn send(&mut self, buffer: &[u8]) {
        self.buffer.clear();
        self.buffer.extend(buffer);
        unsafe { 
            if let Some(send) = self.radio.c_handle.Send {
                send(self.buffer.as_mut_ptr(), 32);
            }
        };
    }

    pub fn set_fsk_tx_config(
        &mut self,
        power: i8,
        fdev: u32,
        datarate: u32,
        preamble_len: u16,
    ) {
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

    pub fn set_lora_tx_config(
        &mut self,
        power: i8,
        bandwidth: LoRaBandwidth,
        datarate: LoRaSpreadingFactor,
        coderate: LoRaCodingRate,
    ) {
        let (freq_hop_on, hop_period) = if let HopPeriod::Enabled(period) = self.settings.hop_period {
            (true, period)
        } else {
            (false, 0)
        };

        unsafe {
            if let Some(set_tx_config) = self.radio.c_handle.SetTxConfig {
                set_tx_config(
                    RadioModems_t_MODEM_LORA, // modem
                    power, // power
                    0, // fdev (is always 0 for LoRa)
                    bandwidth as u32, // bandwidth
                    datarate as u32, // datarate
                    coderate as u8, // coding rate
                    self.settings.preamble_len, // preamble len
                    self.settings.fix_len, // fix length packet
                    self.settings.crc_on, // crc setting
                    freq_hop_on, // frequency hop setting
                    hop_period, // number of symbols before hop
                    self.settings.iq_inverted, // inverted iq
                    3000, // transmission timeout
                );
            }
        };
    }

    pub fn set_frequency(
        &mut self,
        frequency: u32
    ) {
        unsafe { 
            if let Some(set_channel) = self.radio.c_handle.SetChannel {
                set_channel(frequency);
            }
        };
    }

    pub fn enable_hop_period(&mut self, period: u8) {
        self.settings.hop_period = HopPeriod::Enabled(period);
    }

    pub fn disable_hop_period(&mut self) {
        self.settings.hop_period = HopPeriod::Disabled;
    }

    pub fn set_invert_iq(&mut self, set: bool) {
        self.settings.iq_inverted = set;
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
