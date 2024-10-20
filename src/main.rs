#![no_std]
#![no_main]

use core::fmt::Write;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers();
    let mut serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    write!(serial, "\r\nHi there! {}:\r\n", 3).unwrap();

    led.set_high().unwrap();

    loop {
        write!(serial, "\r\nNext loop\r\n").unwrap();

        timer1.delay_ms(1000);
        led.toggle().unwrap();
        timer1.delay_ms(1000);
        led.toggle().unwrap();
        timer1.delay_ms(1000);
        led.toggle().unwrap();
        timer1.delay_ms(50000);
    }
}
