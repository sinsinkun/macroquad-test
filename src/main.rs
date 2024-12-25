use std::time::Duration;

use macroquad::prelude::*;
use macroquad::window;
use miniquad::conf::Platform;

// --- --- --- --- --- --- --- --- --- --- //
// --- --- --- - COMPONENTS -- --- --- --- //
// --- --- --- --- --- --- --- --- --- --- //
mod mq_ui;
use mq_ui::*;

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
		let y = window::screen_height() - 5.0;
		// render display
		draw_text_ex(&self.display, 5.0, y, TextParams {
			font: self.font,
			font_size: 18,
			color: GREEN,
			..Default::default()
		});
	}
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
	let mut ui = UiRoot::new().with(|root| {
		// add theme
		root.theme = UiTheme {
			font: Some(font.clone()),
			font_size: 18,
			..Default::default()
		};

		// dialog box
		let dialog = UiBox::new(
			4, Rect::new(200.0, 200.0, 300.0, 25.0), true, true, Some(&root.theme)
		).with(|dialog| {
			// modify colors directly
			dialog.color = root.theme.secondary[2];
			dialog.hover_color = root.theme.secondary[3];

			let dialog_body = UiBox::new(
				4, Rect::new(0.0, 25.0, 300.0, 100.0), false, false, Some(&root.theme)
			).with(|body| {
				let dialog_txt = UiText::new(5, Rect::new(10.0, 10.0, 10.0, 10.0), "Drag me".to_owned(), false);
				let dialog_btn = UiButton::new(6, Rect::new(10.0, 60.0, 100.0, 30.0), "Button".to_owned());
				body.add_child(UiElement::Text(dialog_txt));
				body.add_child(UiElement::Button(dialog_btn));
			});
			dialog.add_child(UiElement::Box(dialog_body));
		});
		root.add_child(UiElement::Box(dialog));
	});

	let mut nav = UiBox::new(1, Rect::new(0.0, 0.0, 800.0, 50.0), false, false, Some(&ui.theme));
	let search_input = UiInput::new(2, Rect::new(170.0, 10.0, 320.0, 30.0), "Search".to_owned());
	let search_btn = UiButton::new(3, Rect::new(510.0, 10.0, 100.0, 30.0), "Search".to_owned());
	nav.add_child(UiElement::Input(search_input));
	nav.add_child(UiElement::Button(search_btn));
	ui.add_child(UiElement::Box(nav));

	let bg_color = ui.theme.primary;

	loop {
		let win_size = (window::screen_width(), window::screen_height());
		if let Some(elem) = ui.update() {
			match elem {
				UiElement::Button(e) => {
					if e.event == UiAction::LClick {
						println!("Clicked btn {}", e.id);
					}
					if e.event == UiAction::LRelease {
						println!("Released btn {}", e.id);
					}
				}
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
