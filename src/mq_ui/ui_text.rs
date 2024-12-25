use macroquad::prelude::*;
use crate::mq_ui::*;

#[derive(Debug, Clone)]
pub struct UiText {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  draggable: bool,
  pub text: String,
  pub data: Option<UiMetaData>
}
impl UiText {
  pub fn new(id: u32, pos_size: Rect, text: String, draggable: bool) -> Self {
    Self {
      id,
      event: UiAction::None,
      holding: false,
      origin: (pos_size.x, pos_size.y),
      abs_origin: (pos_size.x, pos_size.y),
      size: (pos_size.w, pos_size.h),
      draggable,
      text,
      data: None,
    }
  }
  pub fn with_meta_data(mut self, meta_data: UiMetaData) -> Self {
    self.data = Some(meta_data);
    self
  }
  pub(crate) fn update(
    &mut self,
    target: &mut Option<UiElement>,
    parent_origin: &(f32, f32),
    mouse_pos: &(f32, f32),
    mouse_delta: &(f32, f32),
    l_mouse: &UiMouseAction,
    r_mouse: &UiMouseAction,
  ) {
    update_position(
      &mut self.abs_origin,
      &mut self.origin,
      parent_origin,
      mouse_delta,
      self.draggable,
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
      target.replace(UiElement::Text(self.clone()));
    }
  }
  pub(crate) fn render(&self, theme: &UiTheme, parent_color: &Color) {
    let txt_size = measure_text(&self.text, theme.font, theme.font_size, 1.0);
    let txt_y = self.abs_origin.1 + txt_size.height / 2.0;
    draw_text_ex(&self.text, self.abs_origin.0, txt_y, TextParams {
      font: theme.font,
      font_size: theme.font_size,
      color: contrast_color(parent_color),
      ..Default::default()
    });
  }
}