//! Pulse Width Modulation
//!
//! An initial implementation of embedded-hal's PwmPin trait
//#![feature(extern_prelude)]
#![allow(missing_docs)]
//use ::core::u32;
use hal;
use tm4c123x;
use sysctl;

use tm4c123x::{PWM0,PWM1};
use gpio::gpiof::{PF1};
use gpio::{AlternateFunction, AF5, PushPull};  // FIXME: PushPull?

use core::marker::PhantomData;
use core::mem;

/// Valid PWM output pins
pub trait OutputPin<Module, Generator, Channel> {}

/// A PWM Module
pub trait Module: Sized {
    const POWER_DOMAIN: sysctl::Domain;
    fn power_on(pc: &sysctl::PowerControl) {
        sysctl::control_power(pc,
                              Self::POWER_DOMAIN,
                              sysctl::RunMode::Run,
                              sysctl::PowerState::On);
        sysctl::reset(pc, Self::POWER_DOMAIN);
    }
    unsafe fn ptr() -> *const tm4c123x::pwm0::RegisterBlock;
}
impl Module for PWM0 {
    const POWER_DOMAIN: sysctl::Domain = sysctl::Domain::Pwm0;
    unsafe fn ptr() -> *const tm4c123x::pwm0::RegisterBlock {
        PWM0::ptr()
    }
}
impl Module for PWM1 {
    const POWER_DOMAIN: sysctl::Domain = sysctl::Domain::Pwm1;
    unsafe fn ptr() -> *const tm4c123x::pwm0::RegisterBlock {
        PWM1::ptr()
    }
}

/// A PWM hanldler
pub struct Pwm<M: Module, G: Generator, C: Channel> {
    module: PhantomData<M>,
    generator: PhantomData<G>,
    channel: PhantomData<C>
}

impl<M,G,C> Pwm<M, G, C> where C: Channel, G: Generator, M: Module {
    fn new () -> Self {
        unsafe { mem::uninitialized() }
    }
}

//struct M1PWM5;

pub enum Comparer {
    A,
    B
}

//pub type M1PWM5 = Pwm<PWM0, Generator1, ChannelB>;
/// A PWM Generator Block
pub trait Generator {
    fn enable(pwm: &tm4c123x::pwm0::RegisterBlock);
    fn set_action(pwm: &tm4c123x::pwm0::RegisterBlock, event: CountEvent, action: GeneratorAction);
    fn set_load(pwm: &tm4c123x::pwm0::RegisterBlock, value: u32);
    fn set_compare(pwm: &tm4c123x::pwm0::RegisterBlock, comparer: Comparer, value: u32);
}
pub struct Generator1;
impl Generator for Generator1 {
    fn enable (pwm: &tm4c123x::pwm0::RegisterBlock) {
        unsafe {pwm._1_ctl.write(|w| w.bits(0x01))}
    }

    fn set_action(pwm: &tm4c123x::pwm0::RegisterBlock, event: CountEvent, action: GeneratorAction) {
        let gen_register = &pwm._1_gena;
        match event {
            CountEvent::CompareA(direction) => {
                match direction {
                    CountDirection::Up => gen_register.write(|w: &mut tm4c123x::pwm0::_1_gena::W| w.actcmpau().bits(action as u8)),
                    CountDirection::Down => gen_register.write(|w: &mut tm4c123x::pwm0::_1_gena::W| w.actcmpad().bits(action as u8))
                }
            },
            CountEvent::CompareB(direction) => {
                match direction {
                    CountDirection::Up => gen_register.write(|w: &mut tm4c123x::pwm0::_1_gena::W| w.actcmpbu().bits(action as u8)),
                    CountDirection::Down => gen_register.write(|w:&mut tm4c123x::pwm0::_1_gena::W| w.actcmpbd().bits(action as u8))
                }
            },
            CountEvent::Load => gen_register.write(|w| w.actload().bits(action as u8)),
            CountEvent::Zero => gen_register.write(|w| w.actzero().bits(action as u8))
        }
    }
    fn set_load(pwm: &tm4c123x::pwm0::RegisterBlock, value: u32) {
        unsafe {pwm._1_load.write(|w| w.bits(value))}
    }

    fn set_compare(pwm: &tm4c123x::pwm0::RegisterBlock, comparer: Comparer, value: u32) {
        match comparer {
            Comparer::A => unsafe {pwm._1_cmpa.write(|w| w.bits(value))}
            Comparer::B => unsafe {pwm._1_cmpb.write(|w| w.bits(value))}

        }
    }
}

/// A PMM Output Channel
pub trait Channel {}
pub struct ChannelB;
impl Channel for ChannelB {}

// TODO: When trait alias's are a thing:
//trait M1PWM5 = OutputPin<PWM1, Generator1, ChannelB>
impl OutputPin<PWM1, Generator1, ChannelB> for PF1<AlternateFunction<AF5, PushPull>>{}


pub enum CountDirection {
    Up,
    Down
}

pub enum CountEvent {
    CompareA(CountDirection),
    CompareB(CountDirection),
    Load,
    Zero
}

pub enum GeneratorAction {
    DoNothing = 0x00,
    DriveHigh = 0x01,
    DriveLow = 0x02,
    Invert = 0x04
}
/// Create PwmExt trait
pub trait PwmExt {
    /// Create a PWM Pin
    fn pwm<PIN, GEN, CHAN>(
        pc: &sysctl::PowerControl,
        _pin: PIN
    ) -> Pwm<Self, GEN, CHAN> where PIN: OutputPin<Self, GEN, CHAN>, GEN: Generator, CHAN: Channel, Self: Module {

        Self::power_on(pc);
        let pwm = unsafe { &(*Self::ptr()) };
        GEN::set_action(pwm, CountEvent::CompareA(CountDirection::Up), GeneratorAction::DriveHigh);
        GEN::set_action(pwm, CountEvent::Zero, GeneratorAction::DriveLow);
        GEN::set_load(pwm, 0xFFF);
        GEN::set_compare(pwm, Comparer::A, 0xFF);
        GEN::enable(pwm);

        Pwm::new()
    }
}

impl PwmExt for PWM0 { }

impl PwmExt for PWM1 { }

impl<M, G, C> hal::PwmPin for Pwm<M, G, C> where M: Module, G: Generator, C: Channel {
    type Duty = u16;

    fn disable(&mut self) {
//        C::disable()
    }

    fn enable(&mut self) {
//        C::enable()
    }

    fn get_duty(&self) -> Self::Duty {
//        G::get_duty()
        unimplemented!()
    }

    fn get_max_duty(&self) -> Self::Duty {
//        G::get_max_duty()
        unimplemented!()
    }

    fn set_duty(&mut self, _duty: Self::Duty) {
//        G::set_duty(duty)
        unimplemented!()
    }
}
