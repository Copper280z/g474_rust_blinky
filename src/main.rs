#![no_std]
#![no_main]

use core::borrow::BorrowMut;

use hal::pwm::PwmAdvExt;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use stm32g4xx_hal as hal;
use crate::hal::{stm32, 
    prelude::*, 
    // pwm,
    // time::Hertz,
    // gpio,
};

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};

// use crate::foc::math;
// pub mod foc;

#[entry]
fn main() -> ! {
    let mut dp = stm32::Peripherals::take().expect("cannot take device peripherals");
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");

    let pll_config = hal::rcc::PllConfig{
        mux: hal::rcc::PLLSrc::HSI,
        m: hal::rcc::PllMDiv::DIV_4,
        n: hal::rcc::PllNMul::MUL_85,
        r: Some(hal::rcc::PllRDiv::DIV_2),
        q: None,
        p: None,
    };
    let rcc_config = hal::rcc::Config::new(hal::rcc::SysClockSrc::PLL)
                                                        .pll_cfg(pll_config);

    let mut rcc = dp.RCC.constrain().freeze(rcc_config);
    rtt_init_print!();
    rprintln!("Hello, world!");
    rprintln!("Core Clock: {:?}",rcc.clocks.core_clk);

    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpioa = dp.GPIOA.split(&mut rcc);
    // let gpiob = dp.GPIOB.split(&mut rcc);

    let mut led = gpioa.pa5.into_push_pull_output();
    let pins = (
        gpioa.pa8.into_alternate(),
        gpioa.pa9.into_alternate(),
        gpioa.pa10.into_alternate(),
        gpioa.pa11.into_alternate(),
    );

    let (mut _control, (mut c0, _, _, _)) = dp.TIM1.pwm_advanced(
        pins,
        &mut rcc
    )
    .frequency(20.khz())
    .center_aligned()
    .finalize();

    rprintln!("Timer Configured");
    rprintln!("Max Duty: {}",c0.get_max_duty());

    c0.set_duty(100);
    c0.enable();
    
    dp.TIM1.borrow_mut()
            .cr1
            .modify(|_,w| w.cen().bit(false));

    loop {
            led.set_high().unwrap();
            delay.delay_ms(500);
            led.set_low().unwrap();
            delay.delay_ms(500);

            // rprintln!("Hello, world!");
            // let val = math::_sincos(12);
            // rprintln!("val: {}",val);
    }
}
