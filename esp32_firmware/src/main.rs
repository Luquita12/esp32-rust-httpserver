#![no_std]
#![no_main]

use esp_alloc::EspHeap;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, system::SystemControl,
};

extern crate alloc;
use core::mem::MaybeUninit;

#[global_allocator]
static ALLOCATOR: EspHeap = EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024; /* Set the heap size */
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit(); /* Create a empty vector to the heap*/

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE); /* Alocate de heap size*/
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take(); /* Get peripherals*/
    let system = SystemControl::new(peripherals.SYSTEM); /* Get objects to config peripherals*/

    let clocks = ClockControl::max(system.clock_control).freeze(); /* Configure the system clock*/
    let delay = Delay::new(&clocks); /*Get object to handle delays*/
    init_heap(); /* Init heapp memorry resources*/

    esp_println::logger::init_logger_from_env(); /* Set log level(DEFAULT: Info)*/

    /*  Crate a timer instance, without any timer interuption*/
    let timer = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1, &clocks, None).timer0;

    /* Init wifi connection */
    let _init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timer,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    /* Main loop*/
    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
