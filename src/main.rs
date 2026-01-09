use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use std::sync::Arc;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    width: u32,
    height: u32,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            width: WIDTH,
            height: HEIGHT,
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

        let surface_texture =
            SurfaceTexture::new(self.width, self.height, Arc::clone(&window));
        let pixels =
            Pixels::new(self.width, self.height, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let pixels = self.pixels.as_mut().unwrap();
                draw(pixels.frame_mut(), self.width, self.height);
                if pixels.render().is_err() {
                    event_loop.exit();
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn draw(frame: &mut [u8], width: u32, height: u32) {
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.5;
    let y_max = 1.5;
    let max_iter = 100;

    for py in 0..height {
        for px in 0..width {
            let re = x_min + (px as f64 / WIDTH as f64) * (x_max - x_min);
            let im = y_min + (py as f64 / WIDTH as f64) * (y_max - y_min);
            
            let iterations = escape_time(re, im, max_iter);
            

            let color = (iterations * 255 / max_iter) as u8;
            
            let i = ((py * width + px) * 4) as usize;
            frame[i] = color;
            frame[i + 1] = color;
            frame[i + 2] = color;
            frame[i + 3] = 255;
        }
    }
}

fn escape_time(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;

    for i in 0..max_iter {
        // 1. Prüfen ob escaped: z_re² + z_im² > 4
        if z_re * z_re + z_im * z_im > 4.0{
            return i;
        }
        
        // 2. Neues z berechnen (z = z² + c)
        let z_re_neu = z_re * z_re - z_im * z_im + c_re;
        let z_im_neu = 2.0 * z_re * z_im + c_im;

        z_re = z_re_neu;
        z_im = z_im_neu;

    }

    return max_iter;  // nie escaped → in der Menge
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
