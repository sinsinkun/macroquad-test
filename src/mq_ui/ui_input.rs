use std::collections::HashSet;
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
  blink_counter: f32,
  show_blink: bool,
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
      blink_counter: 0.0,
      show_blink: false,
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
    time_delta: &f32,
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
      let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
      let pressed: HashSet<KeyCode> = get_keys_pressed();
      for key_code in pressed.iter() {
        if key_code == &KeyCode::Backspace {
          self.input.pop();
          continue;
        }
        let cc = key_code_to_char(key_code);
        let c = if shift { cc.1 } else { cc.0 };
        self.input += c;
      }
      // update blinker state
      self.blink_counter += time_delta;
      if self.blink_counter > 0.5 {
        self.show_blink = !self.show_blink;
        self.blink_counter = 0.0;
      }
    } else {
      self.blink_counter = 0.0;
      self.show_blink = false;
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
    // draw text
    let txt_size = measure_text(&self.input, theme.font, theme.font_size, 1.0);
    let txt_x = self.abs_origin.0 + 3.0;
    let txt_y = self.abs_origin.1 + self.size.1 - 10.0;
    if self.is_active || !self.input.is_empty() {
      draw_text_ex(&self.input, txt_x, txt_y, TextParams {
        font: theme.font,
        font_size: theme.font_size,
        color: theme.contrast_color,
        ..Default::default()
      });
    } else if !self.placeholder.is_empty() {
      draw_text_ex(&self.placeholder, txt_x, txt_y, TextParams {
        font: theme.font,
        font_size: theme.font_size,
        color: theme.contrast_color_plus(30.0),
        ..Default::default()
      });
    }
    // draw blinker
    if self.is_active && self.show_blink {
      let blinker_x = self.abs_origin.0 + txt_size.width + 3.0;
      let blinker_y = self.abs_origin.1 + 2.0;
      draw_line(blinker_x, blinker_y, blinker_x, blinker_y + self.size.1 - 4.0, 2.0, BLACK);
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