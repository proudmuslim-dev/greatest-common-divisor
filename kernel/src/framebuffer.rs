use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::{
    geometry::Point,
    pixelcolor::{Rgb888, RgbColor},
    Pixel,
};

pub struct Display<'f> {
    pub framebuffer: &'f mut FrameBuffer,
}

impl<'f> Display<'f> {
    pub fn new(framebuffer: &'f mut FrameBuffer) -> Self {
        Self { framebuffer }
    }

    pub fn draw_horizontal_line(&mut self, height: i32, start_at: i32, length: i32, color: Rgb888) {
        for x in start_at..=(start_at + length) {
            self.draw_pixel(Pixel(Point { x, y: height }, color));
        }
    }

    pub fn draw_vertical_line(&mut self, x: i32, start_at: i32, height: i32, color: Rgb888) {
        for y in start_at..=(start_at + height) {
            self.draw_pixel(Pixel(Point { x, y }, color));
        }
    }

    pub fn draw_square(&mut self, bottom_left_corner: Point, size: i32, color: Rgb888) {
        self.draw_rectangle(bottom_left_corner, size, size, color);
    }

    pub fn draw_rectangle(&mut self, bottom_left_corner: Point, width: i32, height: i32, color: Rgb888) {
        let Point {
            x: initial_x,
            y: initial_y,
        } = bottom_left_corner;

        for x in initial_x..=(initial_x + width) {
            for y in initial_y..=(initial_y + height) {
                self.draw_pixel(Pixel(Point { x, y }, color));
            }
        }
    }

    pub fn draw_pixel(&mut self, Pixel(coordinates, color): Pixel<Rgb888>) {
        let (width, height) = {
            let info = self.framebuffer.info();

            (info.width, info.height)
        };

        let (x, y) = {
            let c: (i32, i32) = coordinates.into();

            (c.0 as usize, height - c.1 as usize) // Invert Y value in order to make coordinates
                                                  // work as expected
        };

        if (0..width).contains(&x) && (0..height).contains(&y) {
            set_pixel_in(&mut self.framebuffer, (x, y), color)
        }
    }
}

fn set_pixel_in(framebuffer: &mut FrameBuffer, (x, y): (usize, usize), color: Rgb888) {
    let info = framebuffer.info();

    let pixel_offset = {
        let line_offset = y * info.stride; // get the start address of the line
        let pixel_offset = line_offset + x;
        pixel_offset * info.bytes_per_pixel
    };

    let pixel_buffer = &mut framebuffer.buffer_mut()[pixel_offset..];
    match info.pixel_format {
        PixelFormat::Rgb => {
            pixel_buffer[0] = color.r();
            pixel_buffer[1] = color.g();
            pixel_buffer[2] = color.b();
        }
        PixelFormat::Bgr => {
            pixel_buffer[0] = color.b();
            pixel_buffer[1] = color.g();
            pixel_buffer[2] = color.r();
        }
        PixelFormat::U8 => {
            let grayscale = color.r() / 3 + color.g() / 3 + color.b() / 3;

            pixel_buffer[0] = grayscale;
        }
        other => panic!("Unknown pixel format: {other:#?}"),
    }
}
