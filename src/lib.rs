mod timer;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Mandelbrot {
    view_left: f32, // Left side of the view in complex space
    view_top: f32, // Top side of the view in complex space
    view_width: f32, // Width of the view in complex space (left + width = right)
    view_height: f32, // Height of the view in complex space (top + height = bottom)
    max_iterations: u8,
    plot_width: u32,
    plot_height: u32,
    plot: Vec<u8>, // The plot data. Must be sized at least (width * height) elements.
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

        let mut zx = cx;
        let mut zy = cy;
        
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
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Mandelbrot {
    pub fn new(plot_width: u32, plot_height: u32) -> Self {
        utils::set_panic_hook();

        let plot = vec![0; (plot_width * plot_height) as usize];

        Self {
            view_left: -2.5,
            view_top: -2.0,
            view_width: 4.0,
            view_height: 4.0,
            max_iterations: 6,
            plot_width: plot_width,
            plot_height: plot_height,
            plot: plot,
        }
    }

    pub fn max_iterations(&self) -> u8 {
        self.max_iterations
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
            }
        }
    }

    pub fn plot_data(&self) -> *const u8 {
        self.plot.as_ptr()
    }
}
