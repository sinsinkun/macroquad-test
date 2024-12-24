use macroquad::prelude::*;
use macroquad::color::colors;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct UiTheme<'a> {
  pub font: Option<&'a Font>,
  pub font_size: u16,
  pub base_color: Color,
  pub contrast_color: Color,
  pub accent_color_1: Color,
  pub accent_color_2: Color,
  pub shadow_color: Color,
}
impl Default for UiTheme<'_> {
  fn default() -> Self {
    Self {
      font: None,
      font_size: 18,
      base_color: colors::GRAY,
      contrast_color: colors::BLACK,
      accent_color_1: colors::BLUE,
      accent_color_2: colors::LIME,
      shadow_color: Color::from_rgba(0, 0, 0, 120),
    }
  }
}
impl UiTheme<'_> {
  pub fn base_color_plus(&self, n: f32) -> Color {
    let val = n / 100.0;
    let mut clr = self.base_color;
    clr.r += val;
    clr.g += val;
    clr.b += val;
    clr
  }
  pub fn contrast_color_plus(&self, n: f32) -> Color {
    let val = n / 100.0;
    let mut clr = self.contrast_color;
    clr.r += val;
    clr.g += val;
    clr.b += val;
    clr
  }
}