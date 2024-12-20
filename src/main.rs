use macroquad::prelude::*;

// window settings
fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Test".to_owned(),
        window_height: 600,
        window_width: 800,
        window_resizable: true,
        icon: None,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // load assets
    set_pc_assets_folder("assets");
    let font = load_ttf_font("Helvetica.ttf").await.unwrap();

    // state vars
    let mut time_since_last_fps_update = 0.0;
    let mut fps_str: String = "FPS: ".to_owned();

    loop {
        // update
        time_since_last_fps_update += get_frame_time();
        if time_since_last_fps_update > 0.5 {
            time_since_last_fps_update = 0.0;
            let fps = get_fps();
            fps_str = "FPS: ".to_owned() + &fps.to_string();
        }

        // render
        clear_background(DARKGRAY);
        draw_text_ex(&fps_str, 5.0, 20.0, TextParams {
            font: Some(&font),
            font_size: 18,
            color: GREEN,
            ..Default::default()
        });
        next_frame().await
    }
}
