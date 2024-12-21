use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use macroquad::prelude::*;
use macroquad::window;
use miniquad::conf::Platform;

// --- --- --- --- --- --- --- --- --- --- //
// --- --- --- - COMPONENTS -- --- --- --- //
// --- --- --- --- --- --- --- --- --- --- //
mod c_ui;
mod c_util;
use c_ui::{UiBox, UiGlobal};

#[derive(Debug)]
struct FpsCounter<'a> {
    time_since_last_update: f32,
    display: String,
    font: Option<&'a Font>
}
impl<'a> FpsCounter<'a> {
    pub fn new(font: Option<&'a Font>) -> Self {
        FpsCounter {
            time_since_last_update: 0.0,
            display: "FPS: ".to_owned(),
            font: font
        }
    }
    pub fn update(&mut self) {
        // update display
        self.time_since_last_update += get_frame_time();
        if self.time_since_last_update > 0.5 {
            self.time_since_last_update = 0.0;
            let fps = get_fps();
            self.display = "FPS: ".to_owned() + &fps.to_string();
        }
        // render display
        draw_text_ex(&self.display, 5.0, 20.0, TextParams {
            font: self.font,
            font_size: 18,
            color: GREEN,
            ..Default::default()
        });
    }
}

fn update_bg_color(bg_color: &mut Color, win_size: &(f32, f32)) {
    // calculate x/y percentage of mouse on screen
    let mouse_pos = mouse_position();
    let pct_x = 0.2 + 0.5 * mouse_pos.0 / win_size.0;
    let pct_y = 0.2 + 0.5 * mouse_pos.1 / win_size.1;
    bg_color.r = pct_x;
    bg_color.b = pct_y;
}

// --- --- --- --- --- --- --- --- --- --- //
// --- --- --- -- MAIN LOOP -- --- --- --- //
// --- --- --- --- --- --- --- --- --- --- //
fn window_conf() -> Conf {
    // note: swap_interval determines Vsync
    // -1: adaptive vsync
    // 0: no vsync
    // 1: vsync
    let platform = Platform {
        swap_interval: Some(0),
        ..Default::default()
    };
    Conf {
        window_title: "Macroquad Test".to_owned(),
        window_height: 600,
        window_width: 800,
        window_resizable: true,
        fullscreen: false,
        icon: None,
        sample_count: 4,
        high_dpi: false,
        platform: platform
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // load assets
    set_pc_assets_folder("assets");
    let font = load_ttf_font("Helvetica.ttf").await.unwrap();

    // states
    let mut fps_counter = FpsCounter::new(Some(&font));
    let ui_global = UiGlobal::new(Some(&font));
    let ui_glb = Rc::new(RefCell::new(ui_global));
    let mut box1 = UiBox::new(
        Rc::clone(&ui_glb),
        Rect { x: 10.0, y: 100.0, w: 200.0, h: 100.0 },
        true, false
    );
    let mut box2 = UiBox::new(
        Rc::clone(&ui_glb),
        Rect { x: 40.0, y: 120.0, w: 100.0, h: 250.0 },
        true, true
    );
    let mut bg_color = Color::from_rgba(60, 60, 60, 255);

    loop {
        let win_size = (window::screen_width(), window::screen_height());
        update_bg_color(&mut bg_color, &win_size);
        ui_glb.borrow_mut().update();
        box2.update();
        box1.update();
        ui_glb.borrow().update_cursor();

        // start render
        clear_background(bg_color);
        box1.render();
        box2.render();
        // draw circle
        draw_poly(win_size.0 / 2.0 + 3.0, win_size.1 / 2.0 + 2.0, 64, 106.0, 0.0, BLACK);
        draw_poly(win_size.0 / 2.0, win_size.1 / 2.0, 64, 100.0, 0.0, RED);
        fps_counter.update();

        // delay to next frame
        std::thread::sleep(Duration::from_micros(6000));
        next_frame().await
    }
}
