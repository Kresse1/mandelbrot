use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use rayon::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    width: u32,
    height: u32,
    center_re: f64,
    center_im: f64,
    zoom: f64,
}
//-0.1015, 0.9563
impl Default for App<'_> {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            width: WIDTH,
            height: HEIGHT,
            center_re: -0.09564267564890372,
            center_im: 0.9590067481571154,
            zoom: 1.0,
        }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Mandelbrot")
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        self.width as f64,
                        self.height as f64,
                    )),
            )
            .unwrap();

        let window = Arc::new(window);

        let surface_texture = SurfaceTexture::new(self.width, self.height, Arc::clone(&window));
        let pixels = Pixels::new(self.width, self.height, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let pixels = self.pixels.as_mut().unwrap();
                draw(pixels.frame_mut(), self.width, self.height, self.center_re, self.center_im, self.zoom);
                self.zoom = self.zoom * 1.01;
                if pixels.render().is_err() {
                    event_loop.exit();
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn draw(frame: &mut [u8], width: u32, height: u32, center_re: f64, center_im: f64, zoom: f64) {
    let max_iter = 100;
    let span_re = 3.5 / zoom;
    let span_im = 3.0 / zoom;
    let x_min = center_re - span_re / 2.0;
    let x_max = center_re + span_re / 2.0;
    let y_min = center_im - span_im / 2.0;
    let y_max = center_im + span_im / 2.0;

    frame
        .par_chunks_mut((width * 4) as usize)  // eine Zeile pro Chunk
        .enumerate()
        .for_each(|(py, row)| {
            let im = y_min + (py as f64 / height as f64) * (y_max - y_min);
            
            for px in 0..width as usize {
                let re = x_min + (px as f64 / width as f64) * (x_max - x_min);
                let iterations = escape_time(re, im, max_iter);
                let color = (iterations * 255 / max_iter) as u8;

                let i = px * 4;
                row[i] = color;
                row[i + 1] = color;
                row[i + 2] = color;
                row[i + 3] = 255;
            }
        });
}


fn escape_time(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re:f64 = 0.0;
    let mut z_im:f64 = 0.0;

    let q = (c_re - 0.25)*(c_re - 0.25) + c_im*c_im;

    if q*(q+(c_re-0.25)) < c_im * c_im / 4.0{
        return max_iter;
    }
    if (c_re +1.0)*(c_re +1.0) + c_im * c_im < 1.0/16.0{
        return max_iter;
    }
    
    for i in 0..max_iter {
        // 1. Prüfen ob escaped: z_re² + z_im² > 4
        if z_re * z_re + z_im * z_im > 4.0 {
            return i;
        }
        // 2. Neues z berechnen (z = z² + c)
        let z_re_neu = z_re * z_re - z_im * z_im + c_re;
        let z_im_neu = 2.0 * z_re * z_im + c_im;

        z_re = z_re_neu;
        z_im = z_im_neu;
    }

    return max_iter; // nie escaped → in der Menge
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
