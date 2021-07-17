#![no_std]
#![no_main]

// openocd server command
// openocd -s C:OpenOCD\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

use cortex_m::asm;
use cortex_m_rt::entry;
use hal::interrupt;
use hd44780_lcd::commands::Driver;
use hd44780_lcd::instructions;
use hd44780_lcd::WriteOnlyHD44780;
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use stm32f3xx_hal::{self as hal, pac, prelude::*};
use rotary_encoder_hal as rotary;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use hal::gpio::{gpioa, gpiob, gpiod, gpioe, Edge, Input, Output, PXx, PushPull};
// use embedded_hal::Direction;
use rotary_encoder_hal::{Direction, Rotary};
use core::borrow::{Borrow, BorrowMut};
use hd44780_lcd::instructions::{ShiftDirection, DataLength, NumberOfDisplayLines, CharacterFont};

// type LedPin = gpioe::PE9<Output<PushPull>>;
// static LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));
// 
// type ButtonPin = gpioa::PA0<Input>;
// static BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
// 
// type RotaryButtonPin = gpioe::PE0<Input>;
// static ROTARY_BUTTON: Mutex<RefCell<Option<RotaryButtonPin>>> = Mutex::new(RefCell::new(None));
// 
// type ClkInPin = gpiod::PD13<Input>;
// static CLK_IN: Mutex<RefCell<Option<ClkInPin>>> = Mutex::new(RefCell::new(None));
// 
// type DtInPin = gpiod::PD11<Input>;
// static DT_IN: Mutex<RefCell<Option<DtInPin>>> = Mutex::new(RefCell::new(None));

type LedPin = gpioe::PE9<Output<PushPull>>;
static LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

type RotaryButtonA = gpiob::PB0<Input>;
type RotaryButtonB = gpiod::PD0<Input>;
static ROTARY_ENCODER: Mutex<RefCell<Option<Rotary<RotaryButtonA, RotaryButtonB>>>> = Mutex::new(RefCell::new(None));

static POSITION: Mutex<RefCell<u8>> = Mutex::new(RefCell::new(0));

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut syscfg = dp.SYSCFG.constrain(&mut rcc.apb2);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // let mut user_button = gpioa.pa0;
    // user_button.make_interrupt_source(&mut syscfg);
    // user_button.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // user_button.enable_interrupt(&mut dp.EXTI);
    // let user_button_interrupt_line = user_button.nvic();
    // cortex_m::interrupt::free(|cs| *BUTTON.borrow(cs).borrow_mut() = Some(user_button));

    // let mut rotary_button = gpioe.pe0.into_pull_down_input(&mut gpioe.moder, &mut gpioe.pupdr);
    // rotary_button.make_interrupt_source(&mut syscfg);
    // rotary_button.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // rotary_button.enable_interrupt(&mut dp.EXTI);
    // let rotary_button_interrupt_line = rotary_button.nvic();
    // cortex_m::interrupt::free(|cs| *ROTARY_BUTTON.borrow(cs).borrow_mut() = Some(rotary_button));
    //
    // let mut clk_in = gpiod.pd13.into_pull_down_input(&mut gpiod.moder, &mut gpiod.pupdr);
    // clk_in.make_interrupt_source(&mut syscfg);
    // clk_in.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // clk_in.enable_interrupt(&mut dp.EXTI);
    // let clk_in_interrupt_line = clk_in.nvic();
    // cortex_m::interrupt::free(|cs| *CLK_IN.borrow(cs).borrow_mut() = Some(clk_in));
    //
    // let mut dt_in = gpiod.pd11.into_pull_down_input(&mut gpiod.moder, &mut gpiod.pupdr);
    // dt_in.make_interrupt_source(&mut syscfg);
    // dt_in.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // dt_in.enable_interrupt(&mut dp.EXTI);
    // let dt_in_interrupt_line = dt_in.nvic();
    // cortex_m::interrupt::free(|cs| *DT_IN.borrow(cs).borrow_mut() = Some(dt_in));

    // let rotary =

    // unsafe {
    //     NVIC::unmask(rotary_button_interrupt_line);
    //     NVIC::unmask(clk_in_interrupt_line);
    //     NVIC::unmask(dt_in_interrupt_line);
    // };

    let mut cw = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut ccw = gpioe
        .pe11
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut sw_out = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Move the ownership of the led to the global LED
    // cortex_m::interrupt::free(|cs| *LED.borrow(cs).borrow_mut() = Some(sw_out));

    let mut alive_timer = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let d0 = gpioa
        .pa0
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let d1 = gpioa
        .pa1
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let d2 = gpioa
        .pa2
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let d3 = gpioa
        .pa3
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let d4 = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let d5 = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let d6 = gpioa
        .pa6
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let d7 = gpioa
        .pa7
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let en = gpioc
        .pc2
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let rs = gpioc
        .pc1
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    let mut lcd = WriteOnlyHD44780::new_bus8(en, rs, d0, d1, d2, d3, d4, d5, d6, d7, delay);


    let str1 = "Scheduled  10:18";
    let str2 = "Expected   10:19";

    lcd.clear_display().unwrap();
    lcd.function_set(
        DataLength::EightBits,
        NumberOfDisplayLines::TwoLines,
        CharacterFont::FiveByEight,
    ).unwrap();
    lcd.set_display_control(
        instructions::ShowDisplay::On,
        instructions::ShowCursor::Off,
        instructions::Blink::Off,
    ).unwrap();
    lcd.set_entry_mode(
        instructions::IncrementDecrement::Increment,
        instructions::AccompaniesDisplayShift::NoShift,
    ).unwrap();
    lcd.set_position(0).unwrap();
    lcd.write_str("GV Rd : IB : 444").unwrap();





    let rot_btn = gpiod.pd9.into_pull_down_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let mut rot_pa = gpiob.pb0.into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let mut rot_pb = gpiod.pd0.into_pull_up_input(&mut gpiod.moder, &mut gpiod.pupdr);

    rot_pa.make_interrupt_source(&mut syscfg);
    rot_pa.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    rot_pa.enable_interrupt(&mut dp.EXTI);
    let rot_pa_interrupt_line = rot_pa.nvic();

    rot_pb.make_interrupt_source(&mut syscfg);
    rot_pb.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    rot_pb.enable_interrupt(&mut dp.EXTI);
    let rot_pb_interrupt_line = rot_pb.nvic();

    let mut enc = rotary::Rotary::new(rot_pa, rot_pb);
    cortex_m::interrupt::free(|cs| *ROTARY_ENCODER.borrow(cs).borrow_mut() = Some(enc));

    unsafe {
        NVIC::unmask(rot_pa_interrupt_line);
        NVIC::unmask(rot_pb_interrupt_line);
    };

    // let mut rot_enc = RotaryEncoder::from_pins(rot_pa, rot_pb, rot_btn);

    let mut counter = 0;

    loop {

        lcd.set_position(40).unwrap();

        if counter % 2 != 0 {
            lcd.write_str(str1);
        } else {
            lcd.write_str(str2);
        }

        counter += 1;

        alive_timer.toggle().unwrap();
        asm::delay(8_000_000);
    }
}

// Button Pressed interrupt.
// The exti# maps to the pin number that is being used as an external interrupt.
// See page 295 of the stm32f303 reference manual for proof:
// http://www.st.com/resource/en/reference_manual/dm00043574.pdf
//
// This may be called more than once per button press from the user since the button may not be debounced.
// todo! what if I don't want to use the button anywhere else? does it need to be in a refcell still?
#[interrupt]
fn EXTI0() {

    cortex_m::interrupt::free(|cs| {

        if let enc = ROTARY_ENCODER.borrow(cs).borrow_mut().as_mut().unwrap() {
            let mut pos = POSITION.borrow(cs).borrow_mut();

            match enc.update().unwrap() {
                Direction::Clockwise => {
                    if *pos < u8::MAX {
                        *pos += 1
                    }
                },
                Direction::CounterClockwise => {
                    if *pos > u8::MIN {
                        *pos -= 1
                    }
                },
                Direction::None => {}
            }

            // clear interrupt source
            if enc.pin_a().check_interrupt() {
                enc.pin_a().clear_interrupt_pending_bit();
            }
            if enc.pin_b().check_interrupt() {
                enc.pin_b().clear_interrupt_pending_bit();
            }
        }
    })


    // cortex_m::interrupt::free(|cs| {
    //     // if BUTTON.borrow(cs).borrow_mut().as_mut().unwrap().check_interrupt() {
    //     //
    //     //     // Toggle the LED
    //     //     LED.borrow(cs)
    //     //         .borrow_mut()
    //     //         .as_mut()
    //     //         .unwrap()
    //     //         .toggle()
    //     //         .unwrap();
    //     //
    //     //     // Clear the interrupt pending bit so we don't infinitely call this routine
    //     //     BUTTON
    //     //         .borrow(cs)
    //     //         .borrow_mut()
    //     //         .as_mut()
    //     //         .unwrap()
    //     //         .clear_interrupt_pending_bit();
    //
    //     // } else if
    //     if ROTARY_BUTTON
    //         .borrow(cs)
    //         .borrow_mut()
    //         .as_mut()
    //         .unwrap()
    //         .check_interrupt()
    //     {
    //         // Toggle the LED
    //         LED.borrow(cs)
    //             .borrow_mut()
    //             .as_mut()
    //             .unwrap()
    //             .toggle()
    //             .unwrap();
    //
    //         ROTARY_BUTTON
    //             .borrow(cs)
    //             .borrow_mut()
    //             .as_mut()
    //             .unwrap()
    //             .clear_interrupt_pending_bit();
    //     }
    // })
}

// #[interrupt]
// fn EXTI15() {
//
// }
