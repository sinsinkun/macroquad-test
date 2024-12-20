use std::time::Duration;

use macroquad::prelude::*;
use miniquad::conf::Platform;

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

// window settings
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

    loop {
        // render
        clear_background(DARKGRAY);
        fps_counter.update();
        // delay to next frame
        std::thread::sleep(Duration::from_millis(6));
        next_frame().await
    }
}
