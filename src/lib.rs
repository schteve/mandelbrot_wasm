mod timer;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[repr(C, packed)]
#[derive(Clone)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
pub struct Mandelbrot {
    view_left: f32, // Left side of the view in complex space
    view_top: f32, // Top side of the view in complex space
    view_width: f32, // Width of the view in complex space (left + width = right)
    view_height: f32, // Height of the view in complex space (top + height = bottom)
    max_iterations: u8,
    plot_width: u32,
    plot_height: u32,
    plot: Vec<u8>, // The number of iterations before the value escaped (or max_iterations if it never escaped). Must be sized at least (width * height) bytes.
    plot_rgba: Vec<RGBA>, // The plot data converted to RGBA. Must be sized at least (width * height * 4) bytes.
}

/// Private methods, not visible to JavaScript
impl Mandelbrot {
    // Get the index into plot that corresponds to the given row (y) and col (x)
    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.plot_width + x) as usize
    }

    fn iterate(&self, x: u32, y: u32) -> Option<u8> {
        let cx = self.view_left + (x as f32) / ((self.plot_width - 1) as f32) * self.view_width;
        let cy = self.view_top + (y as f32) / ((self.plot_height - 1) as f32) * self.view_height;

        let mut zx = 0.0;
        let mut zy = 0.0;
        
        for i in 0..self.max_iterations {
            let xi = zx * zx - zy * zy + cx;
            let yi = 2.0 * zx * zy + cy;

            if (xi * xi + yi * yi) > 4.0 { // Guaranteed to escape
                return Some(i);
            }

            zx = xi;
            zy = yi;
        }

        None
    }

    fn rgba(&self, value: u8) -> RGBA {
        if value >= self.max_iterations {
            RGBA {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        } else {
            RGBA {
                r: ((value & 0x03) >> 0) * 64,
                g: ((value & 0x0C) >> 2) * 64,
                b: ((value & 0x30) >> 4) * 64,
                a: 255,
            }
        }
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Mandelbrot {
    pub fn new(plot_width: u32, plot_height: u32) -> Self {
        utils::set_panic_hook();

        Self {
            view_left: -2.0,
            view_top: -2.0,
            view_width: 4.0,
            view_height: 4.0,
            max_iterations: 16,
            plot_width: plot_width,
            plot_height: plot_height,
            plot: vec![0; (plot_width * plot_height) as usize],
            plot_rgba: vec![RGBA { r: 0, g: 0, b: 0, a: 0 }; (plot_width * plot_height) as usize],
        }
    }

    pub fn view_center(&mut self, x: f32, y: f32) {
        // x is in the range 0..plot_width
        // y is in the range 0..plot_height
        self.view_left += ((x / self.plot_width as f32) - 0.5) * self.view_width;
        self.view_top += ((y / self.plot_height as f32) - 0.5) * self.view_height;
    }

    pub fn view_zoom(&mut self, factor: f32) {
        // Value greater than one means to zoom out, less than one means to zoom in
        self.view_left += (self.view_width / 2.0) * (1.0 - factor);
        self.view_top += (self.view_height / 2.0) * (1.0 - factor);
        self.view_width *= factor;
        self.view_height *= factor;
    }

    pub fn max_iterations_set(&mut self, max_iterations: u8) {
        self.max_iterations = max_iterations;
    }

    pub fn plot_width(&self) -> u32 {
        self.plot_width
    }

    pub fn plot_height(&self) -> u32 {
        self.plot_height
    }

    pub fn plot_generate(&mut self) {
        let _timer = timer::Timer::new("Mandelbrot::plot_generate()");

        for x in 0..self.plot_width {
            for y in 0..self.plot_height {
                let iterations = self.iterate(x, y);
                let idx = self.get_index(x, y);
                self.plot[idx] = iterations.unwrap_or(self.max_iterations);
                self.plot_rgba[idx] = self.rgba(self.plot[idx]);
            }
        }
    }

    pub fn plot_data(&self) -> *const u8 {
        self.plot.as_ptr()
    }

    pub fn plot_rgba(&self) -> *const u8 {
        self.plot_rgba.as_ptr() as *const u8
    }
}
