use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct UiTheme<'a> {
  pub font: Option<&'a Font>,
  pub font_size: u16,
  pub palette_1: Color,
  pub palette_2: Color,
  pub palette_3: Color,
  pub palette_4: Color,
  pub palette_5: Color,
  pub shadow_color: Color,
}
impl Default for UiTheme<'_> {
  fn default() -> Self {
    Self {
      font: None,
      font_size: 18,
      palette_1: Color::from_hex(0xe1e5ee),
      palette_2: Color::from_hex(0xc7ccdb),
      palette_3: Color::from_hex(0x767b91),
      palette_4: Color::from_hex(0x2a324b),
      palette_5: Color::from_hex(0xf7c59f),
      shadow_color: Color::from_rgba(0, 0, 0, 120),
    }
  }
}
