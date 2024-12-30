use crate::mq_ui::*;

/// helper struct for building radio btns
#[derive(Debug, Clone)]
pub struct UiRadioParams<'a> {
  pub pos_size: UiRect,
  pub alignment: UiAlign,
  pub label: String,
  pub theme: Option<&'a UiTheme>,
}
impl Default for UiRadioParams<'_> {
  fn default() -> Self {
    Self {
      pos_size: UiRect::from_px(0.0, 0.0, 100.0, 30.0),
      alignment: UiAlign::TopLeft,
      label: String::new(),
      theme: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct UiRadio {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  abs_bounds: Rect,
  rel_bounds: UiRect,
  alignment: UiAlign,
  label: String,
  pub checked: bool,
  pub cir_color: Color,
  pub data: Option<UiMetaData>
}
impl UiRadio {
  pub fn new(id:u32, params: UiRadioParams) -> Self {
    let mut cir_color = GRAY;
    match params.theme {
      Some(tm) => {
        cir_color = tm.secondary[1];
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
      label: params.label,
      checked: false,
      cir_color,
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
    if self.event == UiAction::LRelease {
      self.checked = !self.checked;
    }
    // clone self into target
    if !action_available && target.is_none() {
      target.replace(UiElement::Radio(self.clone()));
    }
  }
  pub(crate) fn render(&self, theme: &UiTheme, parent_color: &Color) {
    // draw circle
    let cir_origin = vec2(
      self.abs_bounds.x + 12.0,
      self.abs_bounds.y + self.abs_bounds.h / 2.0
    );
    draw_poly(cir_origin.x, cir_origin.y, 24, 8.0, 0.0, self.cir_color);
    if self.checked {
      draw_poly(cir_origin.x, cir_origin.y, 24, 4.0, 0.0, contrast_color(&self.cir_color));
    }
    draw_poly_lines(cir_origin.x, cir_origin.y, 24, 8.0, 0.0, 1.0, BLACK);
    // draw text
    if !self.label.is_empty() {
      let txt_size = measure_text(&self.label, theme.font.as_ref(), theme.font_size, 1.0);
      let txt_x = self.abs_bounds.x + 24.0;
      let txt_y = self.abs_bounds.y - 2.0 + (txt_size.height + self.abs_bounds.h) / 2.0;
      draw_text_ex(&self.label, txt_x, txt_y, TextParams {
        font: theme.font.as_ref(),
        font_size: theme.font_size,
        color: contrast_color(parent_color),
        ..Default::default()
      });
    }
  }
}