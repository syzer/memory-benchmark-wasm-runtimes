#![no_std]

#[cfg(feature = "engine-wasmtime")]
pub mod wasmtime;

#[cfg(feature = "engine-wasmtime")]
mod wasmtime_platform;

#[cfg(feature = "engine-tinywasm")]
pub mod tiny;

#[cfg(feature = "engine-wasmi")]
pub mod wasmi;
