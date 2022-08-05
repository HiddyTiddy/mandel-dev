use std::sync::Arc;

use eframe::{
    egui::{self, style::Margin, Frame, Ui},
    emath::Vec2,
    epaint::{mutex::Mutex, Color32, ColorImage},
};
use nalgebra::Complex;
use rayon::prelude::*;

use crate::{Imager, Mandelbrot, HEIGHT, WIDTH};

pub fn gui() {
    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(Vec2::new(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    eframe::run_native("title", options, Box::new(|_| Box::new(App::new())));
}

struct App {
    scale: f64,
    // texture: AtomicCell<Option<egui::TextureHandle>>,
    texture: Arc<Mutex<Option<egui::TextureHandle>>>,
}

// this function has to change for there to be
// scrolling, scaling, etc
// for this the image has to be repainted
fn to_coordinate(x: usize, y: usize, scale: f64) -> Complex<f64> {
    Complex::new(
        (((x as isize) - (WIDTH / 2) as isize) as f64) / (WIDTH as f64 / 4.0) * scale - 1.0,
        (((y as isize) - (HEIGHT / 2) as isize) as f64) / (HEIGHT as f64 / 3.0) * scale,
    )
}

fn make_buffer() -> Vec<u8> {
    let mandelbroter = Mandelbrot;
    (0..WIDTH * HEIGHT)
        .into_par_iter()
        .map(|i| to_coordinate(i % WIDTH, i / WIDTH, 1.0))
        .flat_map(|coordinate| {
            let pixel = mandelbroter.color_at(coordinate);
            pixel.to_rgba()
        })
        .collect()
}

impl App {
    fn new() -> Self {
        Self {
            scale: 1.0,
            texture: Arc::new(Mutex::new(None)),
        }
    }

    // caching in rust = <3
    fn ui_content(&mut self, ui: &mut Ui) {
        let texture = self.texture.lock();

        if let Some(texture) = &*texture {
            ui.label("got some");
            Frame::none()
                .fill(Color32::from_rgb(0, 100, 200))
                .inner_margin(Margin::same(10.0))
                .show(ui, |ui| {
                    ui.add(egui::Image::new(texture, texture.size_vec2()));
                });
        } else {
            let texture = Arc::clone(&self.texture);
            let ctx = ui.ctx().clone(); // cheap according to docs
            std::thread::spawn(move || {
                let buffer = make_buffer();
                let buffer = ctx.load_texture(
                    "mandelbrot",
                    ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &buffer),
                );
                let mut texture = texture.lock();
                *texture = Some(buffer);
            });
            ui.label("loading...");
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.scale, 0.1..=10.0).text("scale"));
            ui.label(format!("using scale {}", self.scale));
            self.ui_content(ui);
        });
    }
}
