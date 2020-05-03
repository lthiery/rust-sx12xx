#![cfg_attr(not(test), no_std)]
use sx12xx_sys::*;

use Sx12xx_t;
extern crate libm;

pub use sx12xx_sys::Sx12xxEvent_t as Event;
pub use sx12xx_sys::Sx12xxState_t as State;
pub use sx12xx_sys::AntPinsMode_t as AntPinsMode;
pub use sx12xx_sys::BoardBindings_t as BoardBindings;

pub struct Radio {
    c_handle: Radio_t,
}

pub struct Sx12xx {
    handle: Sx12xx_t,
    radio: Radio,
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

impl Sx12xx {
    pub fn new(
        mut radio: Radio,
        bindings: BoardBindings,
    ) -> Sx12xx {

        let mut handle;
        unsafe {
            handle = sx12xx_new_handle();
            sx12xx_init(&mut handle, &mut radio.c_handle, bindings);
        }

        Sx12xx {
            handle,
            radio,
        }
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
