use macroquad::prelude::*;
use crate::mq_ui::*;

/// helper struct for building boxes
pub struct UiBoxParams<'a> {
  pub id: u32,
  pub pos_size: UiRect,
  pub alignment: UiAlign,
  pub draggable: bool,
  pub show_hover: bool,
  pub theme: Option<&'a UiTheme>,
}
impl Default for UiBoxParams<'_> {
  fn default() -> Self {
    Self {
      id: rand(),
      pos_size: UiRect::from_px(0.0, 0.0, 200.0, 150.0),
      alignment: UiAlign::TopLeft,
      draggable: false,
      show_hover: false,
      theme: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct UiBox {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  pub(crate) children: Vec<UiElement>,
  abs_bounds: Rect,
  rel_bounds: UiRect,
  alignment: UiAlign,
  draggable: bool,
  pub show_hover: bool,
  pub color: Color,
  pub hover_color: Color,
  pub data: Option<UiMetaData>
}
impl UiBox {
  pub fn new(params: UiBoxParams) -> Self {
    let mut color = GRAY;
    let mut hover_color = LIGHTGRAY;
    match params.theme {
      Some(tm) => {
        color = tm.secondary[0];
        hover_color = tm.secondary[1];
      }
      None => ()
    }
    Self {
      id: params.id,
      event: UiAction::None,
      holding: false,
      children: Vec::new(),
      rel_bounds: params.pos_size,
      abs_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
      alignment: params.alignment,
      draggable: params.draggable,
      show_hover: params.show_hover,
      color,
      hover_color,
      data: None,
    }
  }
  pub fn with<F>(mut self, func: F) -> Self
  where F: Fn(&mut UiBox) {
    func(&mut self);
    self
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
      self.draggable,
      self.holding,
    );
    let size_delta = if self.abs_bounds.w != 0.0 && self.abs_bounds.h != 0.0 {
      (pos_update.0.w - self.abs_bounds.w, pos_update.0.h - self.abs_bounds.h)
    } else { (0.0, 0.0) };
    self.abs_bounds = pos_update.0;
    self.rel_bounds = pos_update.1;
    // update children
    update_children(
      &mut self.children,
      target,
      &self.abs_bounds,
      &size_delta,
      mouse_pos,
      mouse_delta,
      l_mouse,
      r_mouse,
      time_delta,
    );
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
      target.replace(UiElement::Box(self.clone()));
    }
  }
  pub(crate) fn render(&mut self, theme: &UiTheme) {
    let active_color = match self.event {
      UiAction::Hover | UiAction::Hold | UiAction::LClick | UiAction::LRelease => {
        if self.draggable { self.hover_color }
        else { self.color }
      }
      _ => self.color
    };
    draw_rectangle(
      self.abs_bounds.x - 1.0,
      self.abs_bounds.y - 1.0,
      self.abs_bounds.w + 4.0,
      self.abs_bounds.h + 6.0,
      theme.shadow_color,
    );
    draw_rectangle(
      self.abs_bounds.x,
      self.abs_bounds.y,
      self.abs_bounds.w,
      self.abs_bounds.h,
      active_color,
    );
    // render children
    render_children(&mut self.children, theme, &active_color);
  }
  pub fn add_child(&mut self, elem: UiElement) {
    self.children.push(elem);
  }
}