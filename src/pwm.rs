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
//        sysctl::reset(pc, Self::POWER_DOMAIN);
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

pub type M1PWM5 = Pwm<PWM1, Generator2, ChannelB>;
//struct M1PWM5;

// FIXME: Rename to 'comparitor'
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
    fn set_compare(pwm: &tm4c123x::pwm0::RegisterBlock, comparer: Comparer, value: u16);
}
pub struct Generator2;
impl Generator for Generator2 {
    fn enable (pwm: &tm4c123x::pwm0::RegisterBlock) {
        pwm._2_ctl.write(|w| w.enable().set_bit());
        pwm.enable.write(|w| w.pwm5en().set_bit());
    }

    fn set_action(pwm: &tm4c123x::pwm0::RegisterBlock, event: CountEvent, action: GeneratorAction) {
        // FIXME: Move to Channel trait
        let gena_register = &pwm._2_genb;
        let genb_register = &pwm._2_genb;
        unsafe {pwm._2_ctl.write(|w| w.bits(0) );};
        match event {
            CountEvent::CompareA(direction) => {
                match direction {
                    CountDirection::Up => gena_register.modify(|_, w| w.actcmpau().bits(action as u8)),
                    CountDirection::Down => gena_register.modify(|_, w| w.actcmpad().bits(action as u8))
                }
            },
            CountEvent::CompareB(direction) => {
                match direction {
                    CountDirection::Up => genb_register.modify(|_, w| w.actcmpbu().bits(action as u8)),
                    CountDirection::Down => genb_register.modify(|_, w| w.actcmpbd().bits(action as u8))
                }
            },
            CountEvent::Load => gena_register.modify(|_, w| w.actload().bits(action as u8)),
            CountEvent::Zero => gena_register.modify(|_, w| w.actzero().bits(action as u8))
        }
    }
    fn set_load(pwm: &tm4c123x::pwm0::RegisterBlock, value: u32) {
        unsafe {pwm._2_load.write(|w| w.bits(value))}
    }

    fn set_compare(pwm: &tm4c123x::pwm0::RegisterBlock, comparer: Comparer, value: u16) {
        match comparer {
            Comparer::A => unsafe {pwm._2_cmpa.write(|w| w.compa().bits(value) )}
            Comparer::B => unsafe {pwm._2_cmpb.write(|w| w.compb().bits(value))}
        }
    }
}

/// A PMM Output Channel
pub trait Channel {}
pub struct ChannelA;
pub struct ChannelB;
impl Channel for ChannelA {}
impl Channel for ChannelB {}

// TODO: When trait alias's are a thing:
//trait M1PWM5 = OutputPin<PWM1, Generator1, ChannelB>
impl OutputPin<PWM1, Generator2, ChannelB> for PF1<AlternateFunction<AF5, PushPull>>{}


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

#[repr(u8)]
pub enum GeneratorAction {
    DoNothing = 0x00,
    Invert = 0x01,
    DriveLow = 0x2,
    DriveHigh = 0x03,
}
/// Create PwmExt trait
pub trait PwmExt {
    /// Create a PWM Pin
    fn pwm<PIN, GEN, CHAN>(
        self,
        pc: &sysctl::PowerControl,
        _pin: PIN
    ) -> Pwm<Self, GEN, CHAN> where PIN: OutputPin<Self, GEN, CHAN>, GEN: Generator, CHAN: Channel, Self: Module {

        Self::power_on(pc);
        let pwm = unsafe { &*Self::ptr()};

        // TODO: move to sysctl
        unsafe {
            let ss = &(*tm4c123x::SYSCTL::ptr());
            ::bb::change_bit(&ss.rcc.write(|w| {
                w.usepwmdiv().set_bit();
                w.pwmdiv().bits(64)
            }), 0, true);
        }

        GEN::set_load(&pwm, 0xFFF);
        GEN::set_action(&pwm, CountEvent::Load, GeneratorAction::DriveLow);
        GEN::set_action(&pwm, CountEvent::CompareA(CountDirection::Down), GeneratorAction::DriveHigh);

        GEN::set_compare(&pwm, Comparer::A, 0xF);


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
        G::enable(unsafe {&*M::ptr()});
    }

    fn get_duty(&self) -> Self::Duty {
//        G::get_duty()
        unimplemented!()
    }

    fn get_max_duty(&self) -> Self::Duty {
//        G::get_max_duty()
        unimplemented!()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
//        G::set_duty(duty)
        G::set_compare(unsafe {&*M::ptr()}, Comparer::A, duty);
    }
}
