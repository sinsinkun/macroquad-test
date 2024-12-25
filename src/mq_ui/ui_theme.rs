use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct UiTheme {
  pub font: Option<Font>,
  pub font_size: u16,
  pub primary: Color, // 60%
  pub secondary: [Color; 5], // 30%
  pub accent: [Color; 2], // 10%
  pub shadow_color: Color,
}
impl Default for UiTheme {
  fn default() -> Self {
    let primary     = Color::from_hex(0xBEC3C5);
    let secondary_1 = Color::from_hex(0x94A6AD);
    let secondary_2 = Color::from_hex(0x669099);
    let secondary_3 = Color::from_hex(0x457378);
    let secondary_4 = Color::from_hex(0x285152);
    let secondary_5 = Color::from_hex(0x112826);
    let accent_1    = Color::from_hex(0x4A44C5);
    let accent_2    = Color::from_hex(0xDC1E70);
    Self {
      font: None,
      font_size: 18,
      primary,
      secondary: [secondary_1, secondary_2, secondary_3, secondary_4, secondary_5],
      accent: [accent_1, accent_2],
      shadow_color: Color::from_rgba(0, 0, 0, 120),
    }
  }
}
