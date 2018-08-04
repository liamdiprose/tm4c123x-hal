//! Pulse Width Modulation
//!
//! An initial implementation of embedded-hal's PwmPin trait
//#![feature(extern_prelude)]
#![allow(missing_docs)]
//use ::core::u32;
//use hal;
//use tm4c123x;
use sysctl;

use tm4c123x::{PWM0,PWM1};
use gpio::gpiof::{PF1};
use gpio::{AlternateFunction, AF5, PushPull};  // FIXME: PushPull?

use core::marker::PhantomData;

/// Valid PWM output pins
pub trait OutputPin<Module, Generator, Channel> {}

/// A PWM Module
pub trait Module: Sized {}
impl Module for PWM0 {}
impl Module for PWM1 {}

/// A PWM hanldler
pub struct Pwm<M: Module, G: Generator, C: Channel> {
    module: PhantomData<M>,
    generator: PhantomData<G>,
    channel: PhantomData<C>
}
// FIXME: Use struct instead?


/// Create PwmExt trait
pub trait PwmExt {
    /// Create a PWM Pin
    fn pwm<PIN, GEN, CHAN>(
        pc: &sysctl::PowerControl,
        pin: PIN
    ) -> Pwm<Self, GEN, CHAN> where PIN: OutputPin<Self, GEN, CHAN>, GEN: Generator, CHAN: Channel, Self: Module;
}

//struct M1PWM5;

//pub type M1PWM5 = Pwm<PWM0, Generator1, ChannelB>;
/// A PWM Generator Block
pub trait Generator {}
pub struct Generator1;
impl Generator for Generator1 {}

/// A PMM Output Channel
pub trait Channel {}
pub struct ChannelB;
impl Channel for ChannelB {}

// TODO: When trait alias's are a thing:

//trait M1PWM5 = Pwm<Module1,
//trait M1PWM5 = PF1<AlternateFunction<AF5, PushPull>>;
impl OutputPin<PWM1, Generator1, ChannelB> for PF1<AlternateFunction<AF5, PushPull>>{}


impl PwmExt for PWM0 {
    fn pwm<PIN, GEN, CHAN>(_pc: &sysctl::PowerControl, _pin: PIN) -> Pwm<PWM0, GEN, CHAN>
        where GEN: Generator, CHAN: Channel, PIN: OutputPin<PWM0, GEN, CHAN> {
        // Turn on PWM0

        GEN::turn_on();

        // Start GEN
        // Do GEN things
        unimplemented!()
    }
}


//hal! {
//    // Format:-
//    // Output (PWM Module, Generator, Channel)
//    M1PWM5: (1, 2, B)
//}

