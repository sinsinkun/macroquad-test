use macroquad::prelude::*;
use crate::mq_ui::*;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct UiInput {
  pub id: u32,
  pub event: UiEvent,
  holding: bool,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  pub is_active: bool,
  pub input: String,
  pub placeholder: String,
}
impl UiInput {
  pub fn new(id: u32, pos_size: Rect, placeholder: String) -> Self {
    Self {
      id,
      event: UiEvent::None,
      holding: false,
      origin: (pos_size.x, pos_size.y),
      abs_origin: (pos_size.x, pos_size.y),
      size: (pos_size.w, pos_size.h),
      is_active: false,
      input: String::new(),
      placeholder,
    }
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
    // toggle active state
    match self.event {
      UiEvent::LRelease => {
        self.is_active = !self.is_active;
      }
      UiEvent::LClickOuter => {
        self.is_active = false;
      }
      _ => ()
    };
    // take input
    if self.is_active {
      // register key inputs
    }
    // clone self into target
    if !action_available && target.is_none() {
      target.replace(UiElement::Input(self.clone()));
    }
  }
  pub(crate) fn render(&self, theme: &UiTheme) {
    let mut active_color = match self.event {
      UiEvent::Hover | UiEvent::LClick => theme.base_color_plus(20.0),
      UiEvent::Hold | UiEvent::LRelease => theme.base_color_plus(30.0),
      _ => theme.base_color_plus(10.0)
    };
    if self.is_active { active_color = theme.base_color_plus(30.0) };
    draw_rectangle(
      self.abs_origin.0,
      self.abs_origin.1,
      self.size.0,
      self.size.1,
      active_color,
    );
    if self.is_active || !self.input.is_empty() {
      let txt_size = measure_text(&self.input, theme.font, theme.font_size, 1.0);
      let txt_x = self.abs_origin.0 + 5.0;
      let txt_y = self.abs_origin.1 + 5.0 + txt_size.height;
      draw_text_ex(&self.input, txt_x, txt_y, TextParams {
        font: theme.font,
        font_size: theme.font_size,
        color: theme.contrast_color,
        ..Default::default()
      });
    } else if !self.placeholder.is_empty() {
      let txt_size = measure_text(&self.placeholder, theme.font, theme.font_size, 1.0);
      let txt_x = self.abs_origin.0 + 5.0;
      let txt_y = self.abs_origin.1 + 5.0 + txt_size.height;
      draw_text_ex(&self.placeholder, txt_x, txt_y, TextParams {
        font: theme.font,
        font_size: theme.font_size,
        color: theme.contrast_color_plus(30.0),
        ..Default::default()
      });
    }
    // draw border
    draw_rectangle_lines(
      self.abs_origin.0,
      self.abs_origin.1,
      self.size.0,
      self.size.1,
      1.5,
      BLACK,
    );
  }
}