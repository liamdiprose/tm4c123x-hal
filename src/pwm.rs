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

pub trait OutputPin {}

pub trait Pwm<Module, Generator, Channel> {}
// FIXME: Use struct instead?


// Create PwmExt trait
pub trait PwmExt {
    fn pwm<PIN: OutputPin, GEN, CHAN>(
        pc: &PowerControl,
        pin: PIN<Self, GEN, CHAN>
    ) -> impl Pwm<Self, GEN, CHAN>;
}

//struct M1PWM5;

//pub type M1PWM5 = Pwm<PWM0, Generator1, ChannelB>;

trait Generator {}
struct Generator1;
impl Generator for Generator1 {}

trait Channel {}
struct ChannelB;
impl Channel for ChannelB {}

//type M1PWM5 = Pwm<PWM1, Generator1, ChannelB>;
//trait M1PWM5 = PF1<AlternateFunction<AF5, PushPull>>;
impl Pwm<PWM1, Generator1, ChannelB> for PF1<AlternateFunction<AF5, PushPull>>{}


impl PwmExt for PWM0 {
    fn pwm<PIN: OutputPin, GEN, CHAN>(pc: &sysctl::PowerControl, pin: Pin<PWM0, GEN, CHAN>) -> impl Pwm<Self, GEN, CHAN> where GEN: Generator, CHAN: Channnel {
        // Turn on PWM0

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

