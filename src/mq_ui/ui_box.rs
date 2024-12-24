use macroquad::prelude::*;
use crate::mq_ui::*;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct UiBox {
  pub id: u32,
  pub event: UiEvent,
  holding: bool,
  children: Vec<UiElement>,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  draggable: bool,
  pub show_hover: bool,
}
impl UiBox {
  pub fn new(id: u32, pos_size: Rect, draggable: bool, show_hover: bool) -> Self {
    Self {
      id,
      event: UiEvent::None,
      holding: false,
      children: Vec::new(),
      origin: (pos_size.x, pos_size.y),
      abs_origin: (pos_size.x, pos_size.y),
      size: (pos_size.w, pos_size.h),
      draggable,
      show_hover,
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
      self.draggable,
      self.holding,
    );
    // update children
    update_children(
      &mut self.children,
      target,
      &self.abs_origin,
      mouse_pos,
      mouse_delta,
      l_mouse,
      r_mouse,
      time_delta,
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
      target.replace(UiElement::Box(self.clone()));
    }
  }
  pub(crate) fn render(&self, theme: &UiTheme) {
    let active_color = match self.event {
      UiEvent::Hover | UiEvent::Hold | UiEvent::LClick | UiEvent::LRelease => theme.base_color_plus(20.0),
      _ => theme.base_color
    };
    draw_rectangle(
      self.abs_origin.0 - 1.0,
      self.abs_origin.1 - 1.0,
      self.size.0 + 4.0,
      self.size.1 + 6.0,
      theme.shadow_color,
    );
    draw_rectangle(
      self.abs_origin.0,
      self.abs_origin.1,
      self.size.0,
      self.size.1,
      active_color,
    );
    // render children
    render_children(&self.children, theme);
  }
  pub fn add_child(&mut self, elem: UiElement) {
    self.children.push(elem);
  }
}