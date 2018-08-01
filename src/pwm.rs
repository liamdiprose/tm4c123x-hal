//! Pulse Width Modulation
//!
//! An initial implementation of embedded-hal's PwmPin trait
#![feature(extern_prelude)]
use ::core::u32;
use hal;
use tm4c123x;
use sysctl;

use tm4c123x::{PWM0,PWM1};
use gpio::gpiof::{PF1};
use gpio::{AlternateFunction, AF5, PushPull, OutputMode};  // FIXME: PushPull?


pub unsafe trait XPwmPin<Module,Generator,Channel> {}

pub struct Pwm<MODULE,GENERATOR,CHANNEL> {
    module: MODULE,
    generator: GENERATOR,
    channel: CHANNEL
}

pub trait Module {
    type RegisterBlock;
}
impl Module for PWM0 {
    type RegisterBlock = pwm0::RegisterBlock;
}
impl Module for PWM1 {
    type RegisterBlock = pwm1::RegisterBlock;
}

pub mod generator {
    pub struct G0;
    pub struct G1;
    pub struct G2;
    pub struct G3;

}

// FIXME:? Move generators into PWM module impl's?
/// The different stages that can trigger an action in the PWM module
pub enum Comparer {
    A(CountDirection),
    B(CountDirection),
    Zero,
    Load
}

/// Count direction for a generator action
pub enum CountDirection {
    Up,
    Down
}

/// A PWM Generator
pub trait Generator<Module> {
    fn turn_on(p: &Module::RegisterBlock);
    fn set_action(p: &Module::RegisterBlock, comparer: Comparer, action: PwmTimerAction);
    fn enable_output(p: &Module::RegisterBlock);
}

impl Generator for Module {
    fn turn_on(p: &Module::RegisterBlock) {
    }

    fn set_action(p: &Module::RegisterBlock, comparer: Comparer, action: PwmTimerAction) {
        match comparer {
            Comparer::A(direction) => match direction {
                CountDirection::Up => { p._0_actcmpau.bits(action as u8)},
                CountDirection::Down => { p._0_actcmpad.bits(action as u8)},
            },
            Comparer::B(direction) => match direction {
                CountDirection::Up => { p._0_actcmpau.bits(action as u8)},
                CountDirection::Down => { p._0_actcmpad.bits(action as u8)},
            },
            Comparer::Zero => { p._0_actzero.bits(action as u8)},
            Comparer::Load => { p._0_actload.bits(action as u8)},
        }
    }

    fn enable_output(p: &Module::RegisterBlock) {
        const ENABLE: u8 = 0x01;
        p._0_ctl.bits(ENABLE);
    }
}
//impl Generator for generator::_1 {}
//impl Generator for generator::_2 {}
//impl Generator for generator::_3 {}

// FIXME:? Move channels into generator impls?
pub mod channel {
    pub struct A;
    pub struct B;
}

pub trait Channel {}
impl Channel for channel::A {}
impl Channel for channel::B {}

// FIXME: Come up with a better name for XPwmPin
unsafe impl<T> XPwmPin<PWM1, generator::_1, channel::B> for PF1<AlternateFunction<AF5, PushPull>> where T: OutputMode {}


#[repr(u8)]
enum PwmTimerAction {
    /// Do nothing
    Nothing = 0x00,
    /// Invert PWM signal
    Invert = 0x01,
    /// Drive PWM signal low
    DriveLow = 0x02,
    /// Drive PWM signal high
    DriveHigh = 0x04
}

pub trait PwmExt {
//    General Case --commented--
    fn pwm<P, M, G, C>(
        pin: P,
        module: M,
        generator: G,
        channel: C,
        ss: &mut sysctl::PowerControl,
    ) -> Pwm<M, G, C> where P: XPwmPin<M, G, C>, M: Module, G: Generator, C: Channel {

       let p = unsafe {&(*M::ptr())};

        G::set_action(p, Comparer::A(CountDirection::Down),PwmTimerAction::DriveHigh);
        G::set_action(p, Comparer::Zero, PwmTimerAction::DriveHigh);

//        p._2_gena.write(|w| {
//            w.actcmpad().bits(PwmTimerAction::DriveHigh);
//            w.actzero.bits(PwmTimerAction::DriveLow);
//        });

        G::set_load(p, 0xFFF);
        // TODO: Calculate PWM timer period (aka frequency) using sysctl clock speed and pwm clock division
        unsafe {p._2_load.write(|w| w.bits(0xFFF));};

        G::set_comparer_value(p, Comparer::A, 0x0F);
        // TODO: Calculate comparer based on starting duty cycle
//        unsafe {p._2_cmpa.bits(0x0F);};

        unsafe { p._2_ctl.write(|w| w.enable() )};

        Pwm {module, generator, channel}
    }
}



impl hal::PwmPin for Pwm<PWM1, generator::_1, channel::B> {
    type Duty = u32;

    fn enable(&self) {
        // Enable Module?
        // Enalbe Generator?
        // Enable GPIO map?


        // All of the above need to be enabled, but at what point?
        // PwmPin implies actions should only effect the single pin,
        // implying only the GPIO mapping should be enabled/disabled
    }

    fn disable(&self) {
        unimplemented!()
    }

    fn set_duty(&self, duty: u32) {
        unimplemented!()
    }

    fn get_duty(&self) -> Self::Duty {
        unimplemented!()
    }

    fn get_max_duty(&self) -> Self::Duty {
        core::u32::MAX
    }
}