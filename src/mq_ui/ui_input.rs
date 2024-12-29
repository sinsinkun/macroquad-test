use std::collections::HashSet;
use macroquad::prelude::*;
use crate::mq_ui::*;

/// helper struct for building inputs
#[derive(Debug, Clone)]
pub struct UiInputParams<'a> {
  pub pos_size: UiRect,
  pub alignment: UiAlign,
  pub placeholder: String,
  pub theme: Option<&'a UiTheme>,
}
impl Default for UiInputParams<'_> {
  fn default() -> Self {
    Self {
      pos_size: UiRect::from_px(0.0, 0.0, 200.0, 30.0),
      alignment: UiAlign::TopLeft,
      placeholder: "Input".to_owned(),
      theme: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct UiInput {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  abs_bounds: Rect,
  rel_bounds: UiRect,
  alignment: UiAlign,
  pub is_active: bool,
  pub input: String,
  pub placeholder: String,
  blink_counter: f32,
  show_blink: bool,
  bksp_cooldown: f32,
  target: RenderTarget,
  pub data: Option<UiMetaData>
}
impl UiInput {
  pub fn new(id: u32, params: UiInputParams) -> Self {
    let target = render_target_msaa(2000, 1000, 4);
    Self {
      id,
      event: UiAction::None,
      holding: false,
      rel_bounds: params.pos_size,
      abs_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
      alignment: params.alignment,
      is_active: false,
      input: String::new(),
      placeholder: params.placeholder,
      blink_counter: 0.0,
      show_blink: false,
      bksp_cooldown: 0.0,
      target,
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
    parent_rect: &Rect,
    parent_delta: &(f32, f32),
    mouse_pos: &(f32, f32),
    mouse_delta: &(f32, f32),
    l_mouse: &UiMouseAction,
    r_mouse: &UiMouseAction,
    time_delta: &f32,
  ) {
    let pos_update = update_position_adv(
      &self.abs_bounds,
      &self.rel_bounds,
      parent_rect,
      parent_delta,
      &self.alignment,
      mouse_delta,
      false,
      self.holding,
    );
    self.abs_bounds = pos_update.0;
    self.rel_bounds = pos_update.1;
    // update self
    let inbounds = point_in_rect(mouse_pos, &self.abs_bounds);
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
      UiAction::LRelease => {
        self.is_active = !self.is_active;
      }
      UiAction::LClickOuter => {
        self.is_active = false;
      }
      _ => ()
    };
    // take input
    if self.is_active {
      // register key inputs
      let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
      if is_key_pressed(KeyCode::Backspace) && !self.input.is_empty() {
        // give higher cooldown on first press
        self.input.pop();
        self.bksp_cooldown = 0.5;
      } else if is_key_down(KeyCode::Backspace) && !self.input.is_empty() {
        if self.bksp_cooldown > 0.0 {
          self.bksp_cooldown -= time_delta;
        }
        if self.bksp_cooldown <= 0.0 {
          self.input.pop();
          self.bksp_cooldown = 0.06;
        }
      }
      let pressed: HashSet<KeyCode> = get_keys_pressed();
      for key_code in pressed.iter() {
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
  pub(crate) fn render(&mut self, theme: &UiTheme) {
    let mut active_color = match self.event {
      UiAction::Hover | UiAction::LClick => theme.secondary[2],
      UiAction::Hold | UiAction::LRelease => theme.accent[0],
      _ => theme.secondary[1]
    };
    if self.is_active { active_color = theme.secondary[3] };
    let txt_size = measure_text(&self.input, theme.font.as_ref(), theme.font_size, 1.0);
    self.draw_to_target(theme, &(txt_size.width, txt_size.height), active_color);
    // draw target
    draw_texture_ex(&self.target.texture, self.abs_bounds.x, self.abs_bounds.y, WHITE, DrawTextureParams {
      source: Some(Rect::new(0.0, 0.0, self.abs_bounds.w, self.abs_bounds.h)),
      ..Default::default()
    });
    // draw blinker
    if self.is_active && self.show_blink {
      let mut blinker_x = self.abs_bounds.x + txt_size.width + 3.0;
      if txt_size.width > self.abs_bounds.w {
        // scroll text so its right aligned
        blinker_x = self.abs_bounds.x + self.abs_bounds.w - 3.0;
      }
      let blinker_y = self.abs_bounds.y + 2.0;
      draw_line(blinker_x, blinker_y, blinker_x, blinker_y + self.abs_bounds.h - 4.0, 2.0, contrast_color(&active_color));
    }
    // draw border
    draw_rectangle_lines(self.abs_bounds.x, self.abs_bounds.y, self.abs_bounds.w, self.abs_bounds.h, 1.5, BLACK);
  }
  fn draw_to_target(&mut self, theme: &UiTheme, txt_size: &(f32, f32), active_color: Color) {
    // draw to target
    set_camera(&Camera2D {
      zoom: vec2(0.001, 0.002),
      target: vec2(1000.0, 500.0),
      render_target: Some(self.target.clone()),
      ..Default::default()
    });
    clear_background(active_color);
    // draw text
    let mut txt_x = 3.0;
    let txt_y = self.abs_bounds.h - 10.0;
    let text_color = contrast_color(&active_color);
    if txt_size.0 > self.abs_bounds.w {
      // scroll text so its right aligned
      txt_x = self.abs_bounds.w - txt_size.0 - 3.0;
    }
    if self.is_active || !self.input.is_empty() {
      draw_text_ex(&self.input, txt_x, txt_y, TextParams {
        font: theme.font.as_ref(),
        font_size: theme.font_size,
        color: contrast_color(&active_color),
        ..Default::default()
      });
    } else if !self.placeholder.is_empty() {
      draw_text_ex(&self.placeholder, txt_x, txt_y, TextParams {
        font: theme.font.as_ref(),
        font_size: theme.font_size,
        color: adjust_alpha(&text_color, 0.6),
        ..Default::default()
      });
    }

    // stop drawing to target
    set_default_camera();
  }
  pub fn clear(&mut self) {
    self.input.clear();
  }
}