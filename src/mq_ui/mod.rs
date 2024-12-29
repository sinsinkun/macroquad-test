#![allow(unused)]

use std::ops::{Add, AddAssign, Sub, SubAssign};
use macroquad::prelude::*;
use macroquad::rand::rand;

mod ui_util;
pub use ui_util::*;
mod ui_theme;
pub use ui_theme::UiTheme;
mod ui_root;
pub use ui_root::UiRoot;
mod ui_box;
pub use ui_box::UiBoxParams;
pub use ui_box::UiBox;
mod ui_text;
pub use ui_text::UiTextParams;
pub use ui_text::UiText;
mod ui_button;
pub use ui_button::UiButtonParams;
pub use ui_button::UiButton;
mod ui_input;
pub use ui_input::UiInputParams;
pub use ui_input::UiInput;

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

// --- --- --- --- --- --- --- --- --- --- //
// --- -- -- SIZE & POSITIONING -- --- --- //
// --- --- --- --- --- --- --- --- --- --- //

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UiSize {
  Px(f32),
  Percent(f32),
}
impl UiSize {
  pub fn value(&self) -> f32 {
    match self {
      UiSize::Px(x) => *x,
      UiSize::Percent(x) => *x
    }
  }
  pub fn is_px(&self) -> bool {
    match self {
      UiSize::Px(_) => true,
      _ => false
    }
  }
  pub fn is_percent(&self) -> bool {
    match self {
      UiSize::Percent(_) => true,
      _ => false
    }
  }
}
impl Add<f32> for UiSize {
  type Output = UiSize;
  fn add(self, rhs: f32) -> Self::Output {
    let mut val: UiSize;
    match self {
      UiSize::Px(lhs) => {
        val = UiSize::Px(lhs + rhs);
      }
      UiSize::Percent(lhs) => {
        val = UiSize::Percent(lhs + rhs);
      }
    };
    val
  }
}
impl AddAssign<f32> for UiSize {
  fn add_assign(&mut self, rhs: f32) {
    *self = match self {
      UiSize::Px(lhs) => {
        let val = *lhs + rhs;
        UiSize::Px(val)
      }
      UiSize::Percent(lhs) => {
        let val = *lhs + rhs;
        UiSize::Percent(val)
      }
    };
  }
}
impl Sub<f32> for UiSize {
  type Output = UiSize;
  fn sub(self, rhs: f32) -> Self::Output {
    let mut val: UiSize;
    match self {
      UiSize::Px(lhs) => {
        val = UiSize::Px(lhs - rhs);
      }
      UiSize::Percent(lhs) => {
        val = UiSize::Percent(lhs - rhs);
      }
    };
    val
  }
}
impl SubAssign<f32> for UiSize {
  fn sub_assign(&mut self, rhs: f32) {
    *self = match self {
      UiSize::Px(lhs) => {
        let val = *lhs - rhs;
        UiSize::Px(val)
      }
      UiSize::Percent(lhs) => {
        let val = *lhs - rhs;
        UiSize::Percent(val)
      }
    };
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UiAlign {
  TopLeft,
  TopCenter,
  TopRight,
  CenterLeft,
  FullCenter,
  CenterRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

#[derive(Debug, Clone, Copy)]
pub struct UiRect {
  pub x: UiSize,
  pub y: UiSize,
  pub w: UiSize,
  pub h: UiSize,
}
impl UiRect {
  pub fn from_px(x: f32, y: f32, w: f32, h: f32) -> Self {
    Self {
      x: UiSize::Px(x),
      y: UiSize::Px(y),
      w: UiSize::Px(w),
      h: UiSize::Px(h),
    }
  }
  pub fn from_percent(x: f32, y: f32, w: f32, h: f32) -> Self {
    Self {
      x: UiSize::Percent(x),
      y: UiSize::Percent(y),
      w: UiSize::Percent(w),
      h: UiSize::Percent(h),
    }
  }
}