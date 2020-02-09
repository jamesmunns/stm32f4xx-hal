//! USB peripheral
//!
//! Requires the `synopsys-usb-otg` feature and one of the `usb_fs`/`usb_hs` features.
//! See https://github.com/stm32-rs/stm32f4xx-hal/tree/master/examples
//! for usage examples.

use crate::stm32;

#[cfg(feature = "usb_fs")]
use crate::gpio::{Alternate, AF10, gpioa::{PA11, PA12}};
#[cfg(feature = "usb_hs")]
use crate::gpio::{Alternate, AF12, gpiob::{PB14, PB15}};

use synopsys_usb_otg::UsbPeripheral;
pub use synopsys_usb_otg::UsbBus;

#[cfg(feature = "usb_fs")]
pub struct Peripheral {
    pub usb_global: stm32::OTG_FS_GLOBAL,
    pub usb_device: stm32::OTG_FS_DEVICE,
    pub usb_pwrclk: stm32::OTG_FS_PWRCLK,
    pub pin_dm: PA11<Alternate<AF10>>,
    pub pin_dp: PA12<Alternate<AF10>>,
}
#[cfg(feature = "usb_hs")]
pub struct Peripheral {
    pub usb_global: stm32::OTG_HS_GLOBAL,
    pub usb_device: stm32::OTG_HS_DEVICE,
    pub usb_pwrclk: stm32::OTG_HS_PWRCLK,
    pub pin_dm: PB14<Alternate<AF12>>,
    pub pin_dp: PB15<Alternate<AF12>>,
}

unsafe impl Sync for Peripheral {}

#[cfg(feature = "usb_fs")]
unsafe impl UsbPeripheral for Peripheral {
    //const REGISTERS: *const () = stm32::OTG_FS_GLOBAL::ptr() as *const ();
    const REGISTERS: *const () = 0x50000000 as *const ();

    const HIGH_SPEED: bool = false;
    const FIFO_DEPTH_WORDS: usize = 320;

    fn enable() {
        let rcc = unsafe { &*stm32::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.ahb2enr.modify(|_, w| w.otgfsen().set_bit());

            // Reset USB peripheral
            rcc.ahb2rstr.modify(|_, w| w.otgfsrst().set_bit());
            rcc.ahb2rstr.modify(|_, w| w.otgfsrst().clear_bit());
        });
    }
}
#[cfg(feature = "usb_hs")]
unsafe impl UsbPeripheral for Peripheral {
    //const REGISTERS: *const () = stm32::OTG_HS_GLOBAL::ptr() as *const ();
    const REGISTERS: *const () = 0x40040000 as *const ();

    const HIGH_SPEED: bool = true;
    const FIFO_DEPTH_WORDS: usize = 1024;

    fn enable() {
        let rcc = unsafe { &*stm32::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.ahb1enr.modify(|_, w| w.otghsen().set_bit());

            // Reset USB peripheral
            rcc.ahb1rstr.modify(|_, w| w.otghsrst().set_bit());
            rcc.ahb1rstr.modify(|_, w| w.otghsrst().clear_bit());
        });
    }
}

pub type UsbBusType = UsbBus<Peripheral>;
