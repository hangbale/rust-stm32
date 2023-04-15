#![deny(unsafe_code)]
#![no_std]
#![no_main]
// S接PB12 -接负极即可
use panic_halt as _;
use nb::block;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpiob = dp.GPIOB.split();
    // pb12口 控制蜂鸣器
    let mut buzzer = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(5000.Hz()).unwrap();
    buzzer.set_high();
    loop {
        block!(timer.wait()).unwrap();
        buzzer.set_high();
        block!(timer.wait()).unwrap();
        buzzer.set_low();
    }
}
