use crate::mq_ui::*;

/// helper struct for building text
#[derive(Debug, Clone)]
pub struct UiTextParams<'a> {
  pub pos_size: UiRect,
  pub alignment: UiAlign,
  pub text: String,
  pub font_size: u16,
  pub draggable: bool,
  pub theme: Option<&'a UiTheme>,
}
impl Default for UiTextParams<'_> {
  fn default() -> Self {
    Self {
      pos_size: UiRect::from_px(0.0, 0.0, 10.0, 10.0),
      alignment: UiAlign::TopLeft,
      text: "[Display Text]".to_owned(),
      font_size: 18,
      draggable: false,
      theme: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct UiText {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  abs_bounds: Rect,
  rel_bounds: UiRect,
  alignment: UiAlign,
  draggable: bool,
  pub text: String,
  pub font_size: u16,
  pub data: Option<UiMetaData>
}
impl UiText {
  pub fn new(id: u32, params: UiTextParams) -> Self {
    let mut font_size = params.font_size;
    match params.theme {
      Some(tm) => {
        font_size = tm.font_size;
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
      draggable: params.draggable,
      text: params.text,
      font_size,
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
  ) {
    let pos_update = update_position_adv(
      &self.abs_bounds,
      &self.rel_bounds,
      parent_rect,
      parent_delta,
      &self.alignment,
      mouse_delta,
      self.draggable,
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
      target.replace(UiElement::Text(self.clone()));
    }
  }
  pub(crate) fn render(&self, theme: &UiTheme, parent_color: &Color) {
    let txt_size = measure_text(&self.text, theme.font.as_ref(), theme.font_size, 1.0);
    let txt_y = self.abs_bounds.y + txt_size.height / 2.0;
    draw_text_ex(&self.text, self.abs_bounds.x, txt_y, TextParams {
      font: theme.font.as_ref(),
      font_size: self.font_size,
      color: contrast_color(parent_color),
      ..Default::default()
    });
  }
}