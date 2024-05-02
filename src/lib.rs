#![deny(clippy::all)]

// use napi_derive::napi;

// #[napi]
// pub fn plus_100(input: u32) -> u32 {
//   input + 100
// }

/// import the preludes
// use napi::bindgen_prelude::*;
use napi_derive::napi;

/// module registration is done by the runtime, no need to explicitly do it now.
#[napi]
fn fib(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fib(n - 1) + fib(n - 2),
  }
}
