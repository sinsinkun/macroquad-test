use crate::mq_ui::*;

/// helper struct for building buttons
#[derive(Debug, Clone)]
pub struct UiButtonParams<'a> {
  pub pos_size: UiRect,
  pub alignment: UiAlign,
  pub text: String,
  pub theme: Option<&'a UiTheme>,
}
impl Default for UiButtonParams<'_> {
  fn default() -> Self {
    Self {
      pos_size: UiRect::from_px(0.0, 0.0, 100.0, 30.0),
      alignment: UiAlign::TopLeft,
      text: "Button".to_owned(),
      theme: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct UiButton {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  abs_bounds: Rect,
  rel_bounds: UiRect,
  alignment: UiAlign,
  text: String,
  pub color: Color,
  pub hover_color: Color,
  pub hold_color: Color,
  pub data: Option<UiMetaData>
}
impl UiButton {
  pub fn new(id:u32, params: UiButtonParams) -> Self {
    let mut color = GRAY;
    let mut hover_color = LIGHTGRAY;
    let mut hold_color = BLUE;
    match params.theme {
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
      rel_bounds: params.pos_size,
      abs_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
      alignment: params.alignment,
      text: params.text,
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
    parent_delta: &(f32, f32),
    mouse_pos: &(f32, f32),
    mouse_delta: &(f32, f32),
    l_mouse: &UiMouseAction,
    r_mouse: &UiMouseAction,
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
      self.abs_bounds.x + self.abs_bounds.h / 2.0,
      self.abs_bounds.y + self.abs_bounds.h / 2.0,
      36,
      self.abs_bounds.h / 2.0,
      0.0,
      active_color
    );
    draw_poly(
      self.abs_bounds.x + self.abs_bounds.w - self.abs_bounds.h / 2.0,
      self.abs_bounds.y + self.abs_bounds.h / 2.0,
      36,
      self.abs_bounds.h / 2.0,
      0.0,
      active_color
    );
    draw_poly_lines(
      self.abs_bounds.x + self.abs_bounds.h / 2.0,
      self.abs_bounds.y + self.abs_bounds.h / 2.0,
      36,
      self.abs_bounds.h / 2.0,
      0.0,
      1.0,
      BLACK
    );
    draw_poly_lines(
      self.abs_bounds.x + self.abs_bounds.w - self.abs_bounds.h / 2.0,
      self.abs_bounds.y + self.abs_bounds.h / 2.0,
      36,
      self.abs_bounds.h / 2.0,
      0.0,
      1.0,
      BLACK,
    );
    draw_rectangle(
      self.abs_bounds.x + self.abs_bounds.h / 2.0,
      self.abs_bounds.y,
      self.abs_bounds.w - self.abs_bounds.h,
      self.abs_bounds.h,
      active_color,
    );
    draw_line(
      self.abs_bounds.x + self.abs_bounds.h / 2.0,
      self.abs_bounds.y - 0.5,
      self.abs_bounds.x + self.abs_bounds.w - self.abs_bounds.h / 2.0,
      self.abs_bounds.y - 0.5,
      1.0,
      BLACK,
    );
    draw_line(
      self.abs_bounds.x + self.abs_bounds.h / 2.0,
      self.abs_bounds.y + self.abs_bounds.h + 0.5,
      self.abs_bounds.x + self.abs_bounds.w - self.abs_bounds.h / 2.0,
      self.abs_bounds.y + self.abs_bounds.h + 0.5,
      1.0,
      BLACK,
    );
    // calculate text pos
    let txt_size = measure_text(&self.text, theme.font.as_ref(), theme.font_size, 1.0);
    let txt_x = self.abs_bounds.x + (self.abs_bounds.w - txt_size.width) / 2.0;
    let txt_y = self.abs_bounds.y + txt_size.height + (self.abs_bounds.h - txt_size.height) / 2.0;
    draw_text_ex(&self.text, txt_x, txt_y, TextParams {
      font: theme.font.as_ref(),
      font_size: theme.font_size,
      color: contrast_color(&active_color),
      ..Default::default()
    });
  }
}