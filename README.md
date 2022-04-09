# rtic_template_module
pico rust rtic template modularized (separate task file, separate setup file), tutorial

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

