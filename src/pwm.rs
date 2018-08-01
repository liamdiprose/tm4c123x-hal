//! Pulse Width Modulation
//! 
//! An initial implementation of embedded-hal's PwmPin trait 
#![feature(extern_prelude)]
use ::core::u32;
use hal;
use tm4c123x;
use sysctl;

use tm4c123x::{PWM1}; // TODO: PWM0
use gpio::gpiof::{PF1};
use gpio::{AlternateFunction, AF5, PushPull, OutputMode};  // FIXME: PushPull?


pub unsafe trait XPwmPin<Module,Generator,Channel> {}

pub struct Pwm<MODULE,GENERATOR,CHANNEL> {
    module: MODULE,
    generator: GENERATOR,
    channel: CHANNEL
}

pub enum Generator {
    _0,
    _1,
    _2
}

pub enum Channel {
    A,
    B
}

// FIXME:? Differenciate a PWM from module + generator + channel?
// FIXME: Come up with a better name for XPwmPin
unsafe impl<T> XPwmPin<PWM1, Generator::_1, Channel::B> for PF1<AlternateFunction<AF5, PushPull>> where T: OutputMode {}


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
    fn pwm<M, G, C>(
        ss: &mut sysctl::PowerControl,
    ) -> Pwm<M, G, C> {
       let p = unsafe {&(*tm4c123x::PWM1::ptr())};

        p._2_gena.write(|w| {
            w.actcmpad().bits(PwmTimerAction::DriveHigh);
            w.actzero.bits(PwmTimerAction::DriveLow);
        });

        // TODO: Calculate PWM timer period (aka frequency) using sysctl clock speed and pwm clock division
        unsafe {p._2_load.write(|w| w.bits(0xFFF));};

        // Set value of comparer A
        // TODO: Calculate comparer based on starting duty cycle
        unsafe {p._2_cmpa.bits(0x0F);};

        unsafe { p._2_ctl.write(|w| w.enable() )};

        Pwm {
            module: M,
            generator: G,
            channel: C
        }
    }
}



impl hal::PwmPin for Pwm<PF1> {
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