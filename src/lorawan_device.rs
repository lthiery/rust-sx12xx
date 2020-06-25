use lorawan_device::radio::{TxConfig, RfConfig};
use heapless::{Vec, consts::*};

impl lorawan_device::radio::PhyRxTx for super::Sx12xx {
    type PhyEvent = super::Event;

    fn send(&mut self, buffer: &mut [u8]) {
        self.send(buffer);
    }

    fn get_received_packet(&mut self) -> &mut Vec<u8, U256> {
        &mut self.rx_buffer
    }

    fn configure_tx(&mut self, tx_config: TxConfig) {
        self.set_frequency(tx_config.rf.frequency);
        self.configure_lora_tx(
            tx_config.pw,
            tx_config.rf.bandwidth.into(),
            tx_config.rf.spreading_factor.into(),
            tx_config.rf.coding_rate.into(),
        );
    }

    fn configure_rx(&mut self, config: RfConfig) {
        self.set_frequency(config.frequency);
        self.configure_lora_rx(
            config.bandwidth.into(),
            config.spreading_factor.into(),
            config.coding_rate.into()
        );
    }

    fn set_rx(&mut self) {
        self.set_rx();
    }

    fn handle_phy_event(&mut self, event: super::Event) -> Option<lorawan_device::radio::PhyResponse> {
        Some(match self.handle_event(event) {
            super::State::Busy => lorawan_device::radio::PhyResponse::Busy,
            super::State::TxDone(t) => lorawan_device::radio::PhyResponse::TxDone(t),
            super::State::RxDone(t, q) => lorawan_device::radio::PhyResponse::RxDone(
                lorawan_device::radio::RxQuality::new(
                    q.rssi,
                    q.snr
                )
            ),
            super::State::TxTimeout => lorawan_device::radio::PhyResponse::TxError,
            super::State::RxTimeout => lorawan_device::radio::PhyResponse::RxError,
            super::State::RxError => lorawan_device::radio::PhyResponse::RxError,
        })

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
