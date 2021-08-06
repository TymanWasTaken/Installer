#![windows_subsystem = "windows"]

extern crate fltk;
extern crate rust_embed;
use fltk::{
    app::*,
    prelude::*,
    window::Window,
    frame::Frame,
    button::Button

};
use rust_embed::RustEmbed;
use fltk::image::PngImage;
use std::process::exit;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn main() {
    const WINDOW_WIDTH: i32 = 400;
    const WINDOW_HEIGHT: i32 = 200;

    let app = App::default();

    let mut window: Window = Window::new(
        100,
        100,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        "Bundle installer"
    );
    Frame::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT - 100, "Bundle installer");
    let mut button: Button = Button::new(150, 120, 100, 40, "Install bundle");
    configure_window(&mut window);
    window.end();
    window.show();

    button.set_callback(move |_| install());

    app.run().unwrap();
}

fn install() {
    const BUTTON_WIDTH: i32 = 50;
    const BUTTON_HEIGHT: i32 = 30;
    const WINDOW_WIDTH: i32 = 200;
    const WINDOW_HEIGHT: i32 = 100;

    // TODO: Add actual code that installs bundle

    let mut window: Window = Window::new(200, 200, WINDOW_WIDTH, WINDOW_HEIGHT, "Bundle installer");
    Frame::new(0, 0, 200, 50, "Finished installing bundle.");
    let mut button: Button = Button::new(
        WINDOW_WIDTH / 2 - BUTTON_WIDTH / 2,
        60,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        "Close"
    );
    configure_window(&mut window);
    window.end();
    window.show();

    // Remember: Callbacks after initializing the interface
    button.set_callback(move |_| exit(0));
}

fn configure_window(window: &mut Window) {
    let icon = Asset::get("bundle.png").unwrap();
    let icon_image = PngImage::from_data(&icon.data);

    match icon_image {
        Ok(img) => {
            window.set_icon(Some(img));
        }
        Err(_) => {}
    }

    window.make_resizable(true);
}