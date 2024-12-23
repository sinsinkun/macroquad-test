use std::time::Duration;

use macroquad::prelude::*;
use macroquad::window;
use miniquad::conf::Platform;

// --- --- --- --- --- --- --- --- --- --- //
// --- --- --- - COMPONENTS -- --- --- --- //
// --- --- --- --- --- --- --- --- --- --- //
mod mq_util;
mod mq_ui2;
use mq_ui2::UiButton;
use mq_ui2::UiEvent;
use mq_ui2::UiTheme;
use mq_ui2::{UiRoot, UiBox};

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
    let box1 = UiBox::new(1, Rect::new(40.0, 40.0, 50.0, 50.0), false, true);
    let box2 = UiBox::new(2, Rect::new(40.0, 60.0, 100.0, 100.0), true, true).with_child(box1);
    let btn4 = UiButton::new(4, Rect::new(10.0, 10.0, 100.0, 30.0), "Button".to_owned());
    let box3 = UiBox::new(3, Rect::new(200.0, 200.0, 200.0, 80.0), true, false).with_child(btn4);
    let mut ui = UiRoot::new()
        .with_theme(UiTheme {
            font: Some(&font),
            font_size: 16,
        })
        .with_child(box2)
        .with_child(box3);
    let mut bg_color = Color::from_rgba(60, 60, 60, 255);

    loop {
        let win_size = (window::screen_width(), window::screen_height());
        update_bg_color(&mut bg_color, &win_size);
        if let Some((id, event)) = ui.update() {
            match event {
                UiEvent::LClick => { println!("Clicked node {id}") }
                UiEvent::LRelease => { println!("Released node {id}") }
                _ => ()
            };
        }

        // start render
        clear_background(bg_color);
        // draw circle
        draw_poly(win_size.0 / 2.0 + 3.0, win_size.1 / 2.0 + 2.0, 64, 106.0, 0.0, BLACK);
        draw_poly(win_size.0 / 2.0, win_size.1 / 2.0, 64, 100.0, 0.0, RED);
        // draw ui
        ui.render();
        fps_counter.update();

        // delay to next frame
        std::thread::sleep(Duration::from_micros(6000));
        next_frame().await
    }
}
