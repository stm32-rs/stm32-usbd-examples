//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32_usbd::UsbBus;
use stm32f3xx_hal::{prelude::*, stm32};
use usb_device::prelude::*;
use cdc_acm::{SerialPort, USB_CLASS_CDC};


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

    // Configure the on-board LED (LD10, south red)
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut led = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led.set_low(); // Turn off

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    // F3 Discovery board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    usb_dp.set_low();
    delay(clocks.sysclk().0 / 100);

    let usb_dm = gpioa.pa11.into_af14(&mut gpioa.moder, &mut gpioa.afrh);
    let usb_dp = usb_dp.into_af14(&mut gpioa.moder, &mut gpioa.afrh);

    configure_usb_clock();

    let usb_bus = UsbBus::new(dp.USB_FS, (usb_dm, usb_dp));

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_high(); // Turn on

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        },
                        _ => {},
                    }
                }
            }
            _ => {}
        }

        led.set_low(); // Turn off
    }
}
