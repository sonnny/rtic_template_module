// filename: rx_handler.rs

use crate::app::{do_it};
use crate::setup::{UartType, PioTx};

pub(crate) fn do_it(mut cx: do_it::Context){
   let mut data = [0u8; 1];
   let uart: &UartType = &cx.local.uart;
   let pio_tx: &mut PioTx = &mut cx.local.pio_tx;

        match uart.read_full_blocking(&mut data){
          Err(_e) => {}
          Ok(_count) => {
          match data[0] { // change color of ws2812
              b'g' | b'G' => {pio_tx.write(0x000080);}, // green
              b'r' | b'R' => {pio_tx.write(0x008000);}, // red
              b'b' | b'B' => {pio_tx.write(0x080000);}, // blue
              _ =>           {pio_tx.write(0x000000);}, }}}} // off