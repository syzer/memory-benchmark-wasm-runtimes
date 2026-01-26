#![no_std]
#![no_main]

extern crate alloc;

use defmt_rtt as _;
use embassy_nrf as _;
use embassy_time as _;

#[cfg(feature = "engine-wasmi")]
use memory_benchmark::wasmi;
#[cfg(feature = "engine-wasmtime")]
use memory_benchmark::wasmtime;
use panic_probe as _;

use core::{mem::MaybeUninit, ptr::addr_of_mut};

use embassy_executor::Spawner;

pub const HEAP_SIZE: usize = 200_000;

use embedded_alloc::Heap;
#[global_allocator]
static HEAP: Heap = Heap::empty();

/// Initializes the allocator
fn init_allocator() {
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    init_allocator();

    #[cfg(feature = "engine-tinywasm")]
    spawner
        .spawn(playground::tiny::wasm_task(rx))
        .expect("failed to spawn wasm task");
    #[cfg(feature = "engine-wasmi")]
    spawner
        .spawn(wasmi::wasm_task())
        .expect("failed to spawn wasm task");

    #[cfg(feature = "engine-wasmtime")]
    spawner
        .spawn(wasmtime::wasm_task())
        .expect("failed to spawn wasm task");

    #[cfg(feature = "engine-wamr")]
    spawner
        .spawn(playground::wamr::wasm_task(rx))
        .expect("failed to spawn wasm task");

    #[cfg(not(any(
        feature = "engine-tinywasm",
        feature = "engine-wasmi",
        feature = "engine-wasmtime",
        feature = "engine-wamr"
    )))]
    unimplemented!("one of the engines has to be active")
}
