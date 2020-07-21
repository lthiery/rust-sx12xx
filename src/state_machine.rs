use super::{Sx12xx, Event, Response, Error};
use core::marker::PhantomData;
use lorawan_device::radio::{Event as LoraEvent, Response as LoraResponse, Error as LoraError};

#[derive(Copy, Clone)]
pub enum State
{
    Idle(Idle),
    Txing(Txing),
    Rxing(Rxing),
}

use core::convert::From;

// #[derive(Debug)]
// pub enum Event<'a>
// {
//     TxRequest(TxConfig, &'a mut Vec<u8, U256>),
//     RxRequest(RfConfig),
//     CancelRx,
//     PhyEvent(super::Event),
// }

pub struct StateWrapper
{
    sx12xx: Sx12xx,
    radio_state: State,
}

use core::default::Default;

impl StateWrapper
{
    fn new(sx12xx: Sx12xx) -> Self {
        Self {
            sx12xx,
            radio_state: State::Idle(Idle::default()),
        }
    }
}

macro_rules! default_transition {
    ($from:tt,$to:tt) => {
        impl From<$from> for $to {
            fn from(val: $from) -> $to {
                $to {}
            }
        }
    };
}

macro_rules! state {
    (
        $name:tt; [ $( $y:tt ),* ]
       ) => {
        pub struct $name;

        $(default_transition![
          $name, $y
        ];)*

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self {}
            }
        }

        impl Copy for $name {}
    };
}

state![Idle; [Txing, Rxing]];

impl Default for Idle {
    fn default() -> Self {
        Idle {}
    }
}

impl Idle
{
    fn handle_event(self, sx12xx: &mut Sx12xx, event: LoraEvent<StateWrapper>)
        -> (State, Result<LoraResponse<StateWrapper>, Error>) {
        match event {
            LoraEvent::TxRequest(tx_config, buf) => {
                sx12xx.set_frequency(tx_config.rf.frequency);
                sx12xx.configure_lora_tx(
                    tx_config.pw,
                    tx_config.rf.bandwidth.into(),
                    tx_config.rf.spreading_factor.into(),
                    tx_config.rf.coding_rate.into(),
                );
                let len = buf.len();
                sx12xx.send(buf[..len].as_mut());
                (State::Txing(self.into()), Ok(LoraResponse::Txing))
            }
            LoraEvent::RxRequest(config) => {
                sx12xx.set_frequency(config.frequency);
                sx12xx.configure_lora_rx(
                    config.bandwidth.into(),
                    config.spreading_factor.into(),
                    config.coding_rate.into()
                );
                sx12xx.set_rx();
                (State::Rxing(self.into()), Ok(LoraResponse::Rxing))
            }
            LoraEvent::PhyEvent(phyevent) => (State::Idle(self), Ok(LoraResponse::Idle)),
            LoraEvent::CancelRx => (State::Idle(self), Err(Error::CancelRxWhileIdle.into())),
        }
    }
}

state![Txing; [Idle]];
impl Txing
{
    fn handle_event(self, sx12xx: &mut Sx12xx, event: LoraEvent<StateWrapper>)
        -> (State, Result<LoraResponse<StateWrapper>, Error>) {
        match event {
            LoraEvent::PhyEvent(phyevent) => {
                if let Some(Response::PhyResponse(Response::TxDone(timestamp_ms))) = sx12xx.handle_event(phyevent) {
                    (
                        State::Idle(self.into()),
                        Ok(Response::TxComplete(timestamp_ms)),
                    )
                } else {
                    (State::Txing(self), Ok(LoraResponse::Txing))
                }
            }
            LoraEvent::TxRequest(_, _) => (State::Txing(self), Err(Error::TxRequestDuringTx)),
            LoraEvent::RxRequest(_) => (State::Txing(self), Err(Error::RxRequestDuringTx)),
            LoraEvent::CancelRx => {
                (State::Txing(self), Err(Error::CancelRxDuringTx))
            },
        }
    }
}

state![Rxing; [Idle]];
impl Rxing
{
    fn handle_event(self, sx12xx: &mut Sx12xx, event: LoraEvent<StateWrapper>)
        -> (State, Result<LoraResponse<StateWrapper>, Error>) {
        match event {
            LoraEvent::PhyEvent(phyevent) => {
                // if let radio::Response::RxDone(quality)) = radio.handle_phy_event(phyevent) {
                //     (State::Idle(self.into()), Ok(Response::Rx(quality)))
                // } else {
                //     (State::Rxing(self), Ok(Response::Rxing))
                // }
            }
            LoraEvent::CancelRx => {
                (State::Idle(self.into()), Ok(Response::Idle))
            }
            LoraEvent::TxRequest(_, _) => (State::Rxing(self), Err(Error::TxRequestDuringTx)),
            LoraEvent::RxRequest(_) => (State::Rxing(self), Err(Error::RxRequestDuringRx)),
        }
    }
}

use lorawan_device::radio::{TxConfig, RfConfig};
use heapless::{Vec, consts::*};

impl lorawan_device::radio::PhyRxTx for StateWrapper {
    type PhyEvent = super::Event;
    type PhyResponse = super::Response;
    type PhyError = super::Error;

    fn get_received_packet(&mut self) -> &mut Vec<u8, U256> {
        &mut self.rx_buffer
    }

    fn get_mut_radio(&mut self) -> &mut Self {
        &mut self
    }


    fn handle_event(&mut self, event: LoraEvent<Self>) -> Result<LoraResponse<Self>, LoraError<Self>>
    {
        let (new_state, response) = match &self.radio_state {
            State::Idle(state) => state.handle_event(&mut self.radio, event),
            State::Txing(state) => state.handle_event(&mut self.radio, event),
            State::Rxing(state) => state.handle_event(&mut self.radio, event),
        };
        self.radio_state = new_state;
        Ok(response)
    }
}


impl lorawan_device::Timings for super::Sx12xx {
    fn get_rx_window_offset_ms(&mut self) -> i32 {
        -750
    }
    fn get_rx_window_duration_ms(&mut self) -> u32 {
        1000
    }
}


use lorawan_device::radio::{Bandwidth, SpreadingFactor, CodingRate};

impl Into<super::LoRaBandwidth> for Bandwidth {
    fn into(self: Bandwidth) -> super::LoRaBandwidth {
        match self {
            Bandwidth::_125KHZ => super::LoRaBandwidth::_125KHZ,
            Bandwidth::_250KHZ => super::LoRaBandwidth::_250KHZ,
            Bandwidth::_500KHZ => super::LoRaBandwidth::_500KHZ,
        }
    }
}

impl Into<super::LoRaSpreadingFactor> for SpreadingFactor {
    fn into(self: SpreadingFactor) -> super::LoRaSpreadingFactor {
        match self {
            SpreadingFactor::_7 => super::LoRaSpreadingFactor::_7,
            SpreadingFactor::_8 => super::LoRaSpreadingFactor::_8,
            SpreadingFactor::_9 => super::LoRaSpreadingFactor::_9,
            SpreadingFactor::_10 => super::LoRaSpreadingFactor::_10,
            SpreadingFactor::_11 => super::LoRaSpreadingFactor::_11,
            SpreadingFactor::_12 => super::LoRaSpreadingFactor::_12,
        }
    }
}

impl Into<super::LoRaCodingRate> for CodingRate {
    fn into(self: CodingRate) -> super::LoRaCodingRate {
        match self {
            CodingRate::_4_5 => super::LoRaCodingRate::_4_5,
            CodingRate::_4_6 => super::LoRaCodingRate::_4_6,
            CodingRate::_4_7 => super::LoRaCodingRate::_4_7,
            CodingRate::_4_8 => super::LoRaCodingRate::_4_8,
        }
    }
}
