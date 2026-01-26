#![no_std]
#![feature(c_variadic)]
#![feature(str_as_str)]

#[cfg(feature = "engine-wasmtime")]
pub mod wasmtime;

#[cfg(feature = "engine-wasmtime")]
mod wasmtime_platform;

#[cfg(feature = "engine-tinywasm")]
pub mod tiny;

#[cfg(feature = "engine-wasmi")]
pub mod wasmi;

#[cfg(feature = "engine-wamr")]
pub mod wamr;
