// Draw a 1 bit per pixel black and white image. On a 128x64 SSD1306 display over I2C.
//
// Image was created with ImageMagick:
//
// ```bash
// convert rust.png -depth 1 gray:rust.raw
// ```
//
// This example is for the STM32F103 "Blue Pill" board using I2C1.
//
// Wiring connections are as follows for a CRIUS-branded display:
//
// ```
//      Display -> Blue Pill
// (black)  GND -> GND
// (red)    +5V -> VCC
// (yellow) SDA -> PB9
// (green)  SCL -> PB8
// ```
//

#![no_std]
#![no_main]
// GND 电源地
// VCC 电源正（3～5.5V）
// SCL OLED的D0脚，在IIC通信中为时钟管脚
// SDA OLED的D1脚，在IIC通信中为数据管脚
use panic_halt as _;
use nb::block;
use cortex_m_rt::{
    entry,
    exception,
    ExceptionFrame,
};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use stm32f1xx_hal::{
    pac,
    prelude::*,
    timer::Timer,
    i2c::{BlockingI2c, DutyCycle, Mode},
    stm32
};


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./a.raw"), 128);
    let raw2: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./b.raw"), 128);

    let im = Image::new(&raw, Point::new(0, 0));
    let im2 = Image::new(&raw2, Point::new(0, 0));

    
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(5.Hz()).unwrap();

    loop {
        im.draw(&mut display).unwrap();
        display.flush().unwrap();
        block!(timer.wait()).unwrap();
        display.flush().unwrap();
        im2.draw(&mut display).unwrap();
        display.flush().unwrap();
    }
}

