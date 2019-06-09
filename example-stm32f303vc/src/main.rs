//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

extern crate panic_semihosting;

use core::mem;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32_usbd::UsbBus;
use stm32f3xx_hal::{prelude::*, stm32};
use stm32f3xx_hal::gpio::{AF14, gpioa::{PA11, PA12}};
use usb_device::prelude::*;

fn configure_usb_gpio<DM, DP>(usb_dm: PA11<DM>, usb_dp: PA12<DP>) -> (PA11<AF14>, PA12<AF14>) {
    let moder = unsafe { &(*stm32::GPIOA::ptr()).moder };
    let afrh = unsafe { &(*stm32::GPIOA::ptr()).afrh };

    let mode = 0b10; // alternate function mode
    moder.modify(|r, w| unsafe {
        let offset11 = 2 * 11;
        let offset12 = 2 * 12;
        let mut v = r.bits();
        v = (v & !(0b11 << offset11)) | (mode << offset11);
        v = (v & !(0b11 << offset12)) | (mode << offset12);
        w.bits(v)
    });
    let af = 14;
    afrh.modify(|r, w| unsafe {
        let offset11 = 4 * (11 % 8);
        let offset12 = 4 * (12 % 8);
        let mut v = r.bits();
        v = (v & !(0b1111 << offset11)) | (af << offset11);
        v = (v & !(0b1111 << offset12)) | (af << offset12);
        w.bits(v)
    });

    unsafe {
        (
            mem::transmute(usb_dm),
            mem::transmute(usb_dp),
        )
    }
}

fn configure_usb_clock() {
    let rcc = unsafe { &*stm32::RCC::ptr() };
    rcc.cfgr.modify(|_, w| w.usbpres().set_bit());
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .pclk2(24.mhz())
        .freeze(&mut flash.acr);

    // assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    // F3 Discovery board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    usb_dp.set_low();
    delay(clocks.sysclk().0 / 100);

    // TODO: fix this
    let usb_dm = gpioa.pa11;
    let (usb_dm, usb_dp) = configure_usb_gpio(usb_dm, usb_dp);

    configure_usb_clock();

    let usb_bus = UsbBus::new(dp.USB_FS, (usb_dm, usb_dp));

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
