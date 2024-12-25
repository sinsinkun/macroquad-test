use macroquad::prelude::*;
use crate::mq_ui::*;

#[derive(Debug, Clone)]
pub struct UiBox {
  pub id: u32,
  pub event: UiAction,
  holding: bool,
  pub(crate) children: Vec<UiElement>,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  draggable: bool,
  pub show_hover: bool,
  pub color: Color,
  pub hover_color: Color,
  pub data: Option<UiMetaData>
}
impl UiBox {
  pub fn new(id: u32, pos_size: Rect, draggable: bool, show_hover: bool, theme: Option<&UiTheme>) -> Self {
    let mut color = GRAY;
    let mut hover_color = LIGHTGRAY;
    match theme {
      Some(tm) => {
        color = tm.palette_1;
        hover_color = tm.palette_2;
      }
      None => ()
    }
    Self {
      id,
      event: UiAction::None,
      holding: false,
      children: Vec::new(),
      origin: (pos_size.x, pos_size.y),
      abs_origin: (pos_size.x, pos_size.y),
      size: (pos_size.w, pos_size.h),
      draggable,
      show_hover,
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
  pub(crate) fn render(&mut self, theme: &UiTheme) {
    let active_color = match self.event {
      UiAction::Hover | UiAction::Hold | UiAction::LClick | UiAction::LRelease => {
        if self.draggable { self.hover_color }
        else { self.color }
      }
      _ => self.color
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
    render_children(&mut self.children, theme, &active_color);
  }
  pub fn add_child(&mut self, elem: UiElement) {
    self.children.push(elem);
  }
}