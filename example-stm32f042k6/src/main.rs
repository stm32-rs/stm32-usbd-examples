//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32_usbd::UsbBus;
use stm32f0xx_hal::{prelude::*, stm32};
use usb_device::prelude::*;

#[entry]
fn main() -> ! {
    let mut dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .configure()
        .hsi48()
        .enable_crs(dp.CRS)
        .sysclk(48.mhz())
        .pclk(24.mhz())
        .freeze(&mut dp.FLASH);

    let gpioa = dp.GPIOA.split(&mut rcc);

    let usb_dm = gpioa.pa11;
    let usb_dp = gpioa.pa12;

    let usb_bus = UsbBus::new(dp.USB, (usb_dm, usb_dp));

    let mut serial = cdc_acm::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(cdc_acm::USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                serial.write(&buf[0..count]).ok();
            }
            _ => {}
        }
    }
}
