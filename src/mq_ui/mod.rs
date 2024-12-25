#![allow(unused)]

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UiAction{ None, Hover, Hold, LClickOuter, LClick, RClick, LRelease, RRelease }

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UiMouseAction{ None, Down, Hold, Release }

#[derive(Debug, Clone)]
pub enum UiElement {
  Box(UiBox),
  Text(UiText),
  Button(UiButton),
  Input(UiInput),
}

#[derive(Debug, Clone)]
pub enum UiMetaData {
  Integer(i32),
  Float(f32),
  Text(String),
  VecInt(Vec<i32>),
  VecFloat(Vec<f32>),
  VecText(Vec<String>),
}

mod ui_util;
pub use ui_util::*;
mod ui_theme;
pub use ui_theme::UiTheme;
mod ui_root;
pub use ui_root::UiRoot;
mod ui_box;
pub use ui_box::UiBox;
mod ui_text;
pub use ui_text::UiText;
mod ui_button;
pub use ui_button::UiButton;
mod ui_input;
pub use ui_input::UiInput;