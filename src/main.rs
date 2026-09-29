use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("game of life")
                        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
                )
                .unwrap(),
        );

        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window.clone());
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut() {
                    let frame = pixels.frame_mut();

                    for pixel in frame.chunks_exact_mut(4) {
                        pixel[0] = 255; // R
                        pixel[1] = 105; // G
                        pixel[2] = 180; // B
                        pixel[3] = 255; // A
                    }

                    pixels.render().unwrap();
                }
            }

            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
