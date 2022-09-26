use std::ops::{Deref, DerefMut};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window as WinitWindow, WindowBuilder};

pub struct Window {
    window: WinitWindow,
    aspect_ratio: f32,
}

impl Window {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        let window_inner_size = PhysicalSize {
            width: 1024.0,
            height: 768.0,
        };
        let window = WindowBuilder::new()
            .with_inner_size(window_inner_size)
            .build(event_loop)
            .unwrap();
        let aspect_ratio = window_inner_size.width / window_inner_size.height;
        Self {
            window,
            aspect_ratio,
        }
    }

    pub fn on_resized(&mut self) {
        let window_inner_size = self.window.inner_size();
        let aspect_ratio = window_inner_size.width as f32 / window_inner_size.height as f32;
        self.aspect_ratio = aspect_ratio;
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}

impl Deref for Window {
    type Target = WinitWindow;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}
