#![no_main]
#![no_std]

// matti@miya:/etc/udev/rules.d$ cat 99-usb-test.rules
// SUBSYSTEMS=="usb", ATTR{idVendor}=="16c0", ATTR{idProduct}=="27d8", MODE:="0666"

extern crate panic_semihosting;

use cortex_m::asm::delay;
use rtfm::app;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{
    prelude::*,
    usb::{Peripheral, UsbBus, UsbBusType},
    gpio::{gpioc::*, Output, PushPull},
};
use usb_device::bus;
use usb_device::prelude::*;
use usbd_webusb::WebUsb;

mod blinky {
    use core::marker::PhantomData;
    use embedded_hal::digital::v2::OutputPin;
    use usb_device::class_prelude::*;

    pub struct BlinkyClass<B: UsbBus, LED> {
        spooky: core::marker::PhantomData<B>,
        led: LED,
    }

    impl<B: UsbBus, LED: OutputPin> BlinkyClass<B, LED> {
        pub fn new(_alloc: &UsbBusAllocator<B>, led: LED) -> Self {
            Self {
                spooky: PhantomData,
                led,
            }
        }
    }

    impl<B: UsbBus, LED: OutputPin> UsbClass<B> for BlinkyClass<B, LED> {
        fn control_out(&mut self, xfer: ControlOut<B>) {
            let req = xfer.request();

            if req.request_type == control::RequestType::Vendor
                && req.recipient == control::Recipient::Device
                && req.request == 1
            {
                if req.value > 0 {
                    self.led.set_low().ok();
                } else {
                    self.led.set_high().ok();
                }
            }
        }
    }
}

#[app(device = stm32f1xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        usb_dev: UsbDevice<'static, UsbBusType>,
        blinky: blinky::BlinkyClass<UsbBusType, PC13<Output<PushPull>>>,
        webusb: WebUsb<UsbBusType>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        static mut USB_BUS: Option<bus::UsbBusAllocator<UsbBusType>> = None;

        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(48.mhz())
            .pclk1(24.mhz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        let mut gpioc = cx.device.GPIOC.split(&mut rcc.apb2);

        let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        led.set_high().ok();

        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);

        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        usb_dp.set_low().unwrap();
        delay(clocks.sysclk().0 / 100);

        let usb_dm = gpioa.pa11;
        let usb_dp = usb_dp.into_floating_input(&mut gpioa.crh);

        let usb = Peripheral {
            usb: cx.device.USB,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        *USB_BUS = Some(UsbBus::new(usb));

        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27d8))
            .manufacturer("Fake Company")
            .product("Web Blinky")
            .serial_number("TEST")
            .build();

        init::LateResources {
            usb_dev,
            blinky: blinky::BlinkyClass::new(USB_BUS.as_ref().unwrap(), led),
            webusb: WebUsb::new(
                USB_BUS.as_ref().unwrap(),
                usbd_webusb::url_scheme::HTTPS,
                "virkkunen.net/b/blinky.html"),
        }
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [usb_dev, webusb, blinky])]
    fn usb_lp(cx: usb_lp::Context) {
        cx.resources.usb_dev.poll(&mut [
            cx.resources.webusb,
            cx.resources.blinky
        ]);
    }
};
