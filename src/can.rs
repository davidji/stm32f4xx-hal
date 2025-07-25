//! # Controller Area Network (CAN) Interface
//!

use crate::gpio;
use crate::pac::{self, RCC};
use crate::rcc;

pub trait Instance: crate::Sealed + rcc::Enable + rcc::Reset + gpio::alt::CanCommon {}

macro_rules! can {
    ($CAN:ty: $Can:ident) => {
        pub type $Can = Can<$CAN>;

        impl Instance for $CAN {}
    };
}

// Implemented by all SPI instances
can! { pac::CAN1: Can1 }
#[cfg(feature = "can2")]
can! { pac::CAN2: Can2 }
#[cfg(feature = "can3")]
can! { pac::CAN3: Can3 }

/// Pins and definitions for models with a third CAN peripheral
#[cfg(feature = "can3")]
mod can3 {
    use super::*;

    unsafe impl bxcan::Instance for Can<pac::CAN3> {
        const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN3::ptr() as *mut _;
    }

    unsafe impl bxcan::FilterOwner for Can<pac::CAN3> {
        const NUM_FILTER_BANKS: u8 = 14;
    }
}

pub trait CanExt: Sized + Instance {
    fn can(self, pins: (impl Into<Self::Tx>, impl Into<Self::Rx>), rcc: &mut RCC) -> Can<Self>;

    fn tx(self, tx_pin: impl Into<Self::Tx>, rcc: &mut RCC) -> Can<Self>;

    fn rx(self, rx_pin: impl Into<Self::Rx>, rcc: &mut RCC) -> Can<Self>;
}

impl<CAN: Instance> CanExt for CAN {
    fn can(self, pins: (impl Into<Self::Tx>, impl Into<Self::Rx>), rcc: &mut RCC) -> Can<Self> {
        Can::new(self, pins, rcc)
    }

    fn tx(self, tx_pin: impl Into<Self::Tx>, rcc: &mut RCC) -> Can<Self> {
        Can::tx(self, tx_pin, rcc)
    }

    fn rx(self, rx_pin: impl Into<Self::Rx>, rcc: &mut RCC) -> Can<Self> {
        Can::rx(self, rx_pin, rcc)
    }
}

/// Interface to the CAN peripheral.
pub struct Can<CAN: Instance> {
    can: CAN,
    pins: (Option<CAN::Tx>, Option<CAN::Rx>),
}

impl<CAN: Instance> Can<CAN> {
    /// Creates a CAN interface.
    pub fn new(can: CAN, pins: (impl Into<CAN::Tx>, impl Into<CAN::Rx>), rcc: &mut RCC) -> Self {
        Self::_new(can, (Some(pins.0.into()), Some(pins.1.into())), rcc)
    }
    fn _new(can: CAN, pins: (Option<CAN::Tx>, Option<CAN::Rx>), rcc: &mut RCC) -> Self {
        CAN::enable(rcc);
        CAN::reset(rcc);

        Can { can, pins }
    }

    pub fn release(self) -> (CAN, (Option<CAN::Tx>, Option<CAN::Rx>)) {
        (self.can, self.pins)
    }
}

impl<CAN: Instance> Can<CAN> {
    pub fn tx(usart: CAN, tx_pin: impl Into<CAN::Tx>, rcc: &mut RCC) -> Self {
        Self::_new(usart, (Some(tx_pin.into()), None), rcc)
    }

    pub fn rx(usart: CAN, rx_pin: impl Into<CAN::Rx>, rcc: &mut RCC) -> Self {
        Self::_new(usart, (None, Some(rx_pin.into())), rcc)
    }
}

unsafe impl bxcan::Instance for Can<pac::CAN1> {
    const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN1::ptr() as *mut _;
}

#[cfg(feature = "can2")]
unsafe impl bxcan::Instance for Can<pac::CAN2> {
    const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN2::ptr() as *mut _;
}

unsafe impl bxcan::FilterOwner for Can<pac::CAN1> {
    const NUM_FILTER_BANKS: u8 = 28;
}

unsafe impl bxcan::MasterInstance for Can<pac::CAN1> {}
