#![no_std]
#![no_main]

mod framebuffer;

use bootloader_api::BootInfo;
//  HACK: +10 for jank! Screw you, rust-analyzer.
#[cfg(not(test))]
use core::panic::PanicInfo;
use embedded_graphics::{geometry::Point, pixelcolor::Rgb888, prelude::RgbColor, Pixel};
use framebuffer::Display;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let Some(framebuffer) = boot_info.framebuffer.as_mut() else {
        loop {}
    };

    framebuffer.buffer_mut().iter_mut().for_each(|pixel| {
        *pixel = 0x00;
    });

    let mut display = Display::new(framebuffer);

    for n in 0..=20 {
        let step = n * 40;

        // Draw 800x800 px grid 
        display.draw_horizontal_line(step, 0, 800, Rgb888::GREEN);
        display.draw_vertical_line(step, 0, 800, Rgb888::GREEN);

        // Draw diagonals
        for x in 0..=step {
            for y in 0..=step {
                if x + y == step {
                    display.draw_pixel(Pixel(Point { x, y }, Rgb888::GREEN));
                }
            }
        }
    }

    // Answer line
    for n in 0..=800 {
        display.draw_pixel(Pixel(Point { x: n, y: n }, Rgb888::RED));
    }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // no one cares
}
