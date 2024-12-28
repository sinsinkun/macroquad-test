use macroquad::prelude::*;
use crate::mq_ui::*;

#[derive(Debug, Clone)]
pub struct UiButton {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  text: String,
  pub color: Color,
  pub hover_color: Color,
  pub hold_color: Color,
  pub data: Option<UiMetaData>
}
impl UiButton {
  pub fn new(id: u32, pos_size: Rect, text: String, theme: Option<&UiTheme>) -> Self {
    let mut color = GRAY;
    let mut hover_color = LIGHTGRAY;
    let mut hold_color = BLUE;
    match theme {
      Some(tm) => {
        color = tm.secondary[0];
        hover_color = tm.secondary[1];
        hold_color = tm.secondary[2];
      }
      None => ()
    };
    Self {
      id,
      event: UiAction::None,
      holding: false,
      origin: (pos_size.x, pos_size.y),
      abs_origin: (pos_size.x, pos_size.y),
      size: (pos_size.w, pos_size.h),
      text,
      color,
      hover_color,
      hold_color,
      data: None
    }
  }
  pub fn with_meta_data(mut self, meta_data: UiMetaData) -> Self {
    self.data = Some(meta_data);
    self
  }
  pub(crate) fn update(
    &mut self,
    target: &mut Option<UiElement>,
    parent_rect: &Rect,
    mouse_pos: &(f32, f32),
    mouse_delta: &(f32, f32),
    l_mouse: &UiMouseAction,
    r_mouse: &UiMouseAction,
  ) {
    update_position(
      &mut self.abs_origin,
      &mut self.origin,
      &(parent_rect.x, parent_rect.y),
      mouse_delta,
      false,
      self.holding,
    );
    // update self
    let bounds = Rect {
      x: self.abs_origin.0, y: self.abs_origin.1, w: self.size.0, h: self.size.1
    };
    let inbounds = point_in_rect(mouse_pos, &bounds);
    let mut action_available = target.is_none();
    self.event = update_event(
      &mut action_available,
      inbounds,
      &mut self.holding,
      &self.event,
      l_mouse,
      r_mouse,
    );
    // clone self into target
    if !action_available && target.is_none() {
      target.replace(UiElement::Button(self.clone()));
    }
  }
  pub(crate) fn render(&self, theme: &UiTheme) {
    let active_color = match self.event {
      UiAction::Hover | UiAction::LClick => self.hover_color,
      UiAction::Hold | UiAction::LRelease => self.hold_color,
      _ => self.color
    };
    // draw pill
    draw_poly(
      self.abs_origin.0 + self.size.1 / 2.0,
      self.abs_origin.1 + self.size.1 / 2.0,
      36,
      self.size.1 / 2.0,
      0.0,
      active_color
    );
    draw_poly(
      self.abs_origin.0 + self.size.0 - self.size.1 / 2.0,
      self.abs_origin.1 + self.size.1 / 2.0,
      36,
      self.size.1 / 2.0,
      0.0,
      active_color
    );
    draw_poly_lines(
      self.abs_origin.0 + self.size.1 / 2.0,
      self.abs_origin.1 + self.size.1 / 2.0,
      36,
      self.size.1 / 2.0,
      0.0,
      1.0,
      BLACK
    );
    draw_poly_lines(
      self.abs_origin.0 + self.size.0 - self.size.1 / 2.0,
      self.abs_origin.1 + self.size.1 / 2.0,
      36,
      self.size.1 / 2.0,
      0.0,
      1.0,
      BLACK,
    );
    draw_rectangle(
      self.abs_origin.0 + self.size.1 / 2.0,
      self.abs_origin.1,
      self.size.0 - self.size.1,
      self.size.1,
      active_color,
    );
    draw_line(
      self.abs_origin.0 + self.size.1 / 2.0,
      self.abs_origin.1 - 0.5,
      self.abs_origin.0 + self.size.0 - self.size.1 / 2.0,
      self.abs_origin.1 - 0.5,
      1.0,
      BLACK,
    );
    draw_line(
      self.abs_origin.0 + self.size.1 / 2.0,
      self.abs_origin.1 + self.size.1 + 0.5,
      self.abs_origin.0 + self.size.0 - self.size.1 / 2.0,
      self.abs_origin.1 + self.size.1 + 0.5,
      1.0,
      BLACK,
    );
    // calculate text pos
    let txt_size = measure_text(&self.text, theme.font.as_ref(), theme.font_size, 1.0);
    let txt_x = self.abs_origin.0 + (self.size.0 - txt_size.width) / 2.0;
    let txt_y = self.abs_origin.1 + txt_size.height + (self.size.1 - txt_size.height) / 2.0;
    draw_text_ex(&self.text, txt_x, txt_y, TextParams {
      font: theme.font.as_ref(),
      font_size: theme.font_size,
      color: contrast_color(&active_color),
      ..Default::default()
    });
  }
}