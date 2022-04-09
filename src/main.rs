// filename: main.rs

// rtic demo - April 8, 2022
// uart on pin gpio0, gpio1 (need usb to uart)
//     uart ttl -> https://www.adafruit.com/product/954
//
// led on pin gpio25
// ws2812 pin on gpio10
// ws2812 pio is from -> https://github.com/raspberrypi/pico-examples/blob/master/pio/ws2812/ws2812.pio
//
// this is my rust tutorial for myself
//     separate file for setup (hardware setup on a separate file)
//     separate file for uart interrupt (task example on a separate file)
//     uses pio on rtic
//
// six files in this tutorial to start rust project
//     1. main.rs       - should be in the src directory
//     2. setup.rs      - should be in the src directory
//     3. rx_handler.rs - should be in the src directory
//     3. Cargo.toml    - should be in the root project directory
//     4. memory.x      - should be in the root project directory
//     5. config.toml   - should be in the .cargo directory 
//
// uart baud rate for testing is 115_200  

#![no_std]
#![no_main]

use panic_halt as _;
mod setup;       // this is where the pico hardware is setup
mod rx_handler;  // this is where interrupt uart receive handler

#[rtic::app(device = rp_pico::hal::pac, dispatchers = [XIP_IRQ])]
mod app {
    use crate::setup::setup;
    use crate::setup::LedPin;
    use crate::setup::UartType;
    use crate::setup::PioTx;
    use embedded_hal::digital::v2::ToggleableOutputPin;
    use rp2040_monotonic::*;
    
    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Monotonic = Rp2040Monotonic;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {led: LedPin, uart:UartType, pio_tx: PioTx}

    #[init(local = [])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (mono, led, uart, pio_tx) = setup(cx.device, cx.core);

        led_blinker::spawn().ok();

        ( Shared {},
          Local {led, uart, pio_tx},
          init::Monotonics(mono),)}

    // toggle led - shows we're still alive
    #[task(local = [led])]
    fn led_blinker(cx: led_blinker::Context) {
        cx.local.led.toggle().ok();
        led_blinker::spawn_after(500.millis()).ok();
    }
    
    use crate::rx_handler::do_it;
    extern "Rust" {
    #[task(binds = UART0_IRQ, priority = 2, local = [uart, pio_tx])]
    fn do_it(cx: do_it::Context);}} // do_it is in rx_handler.rs file
