#![allow(dead_code)]

use macroquad::prelude::*;
use macroquad::window;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;

use crate::mq_util::point_in_rect;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UiEvent{ None, Hover, Hold, LClickOuter, LClick, RClick, LRelease, RRelease }

#[derive(Debug, Clone)]
pub struct UiTheme<'a> {
  pub font: Option<&'a Font>,
  pub font_size: u16,
}
impl Default for UiTheme<'_> {
  fn default() -> Self {
    Self {
      font: None,
      font_size: 18,
    }
  }
}

#[derive(Debug, Clone)]
pub struct UiNodeParams {
  event: UiEvent,
  holding: bool,
  id_read_only: u32,
  draggable_read_only: bool,
  show_hover_read_only: bool,
  rel_pos_size: Rect,
  abs_pos_size: Rect,
}

pub trait UiNode {
  fn get_children(&self) -> Option<&Vec<Box<dyn UiNode>>>;
  fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn UiNode>>>;
  fn is_mouse_in_bounds(&self, mouse_pos: &(f32, f32)) -> bool;
  fn update(&mut self);
  fn render(&self, theme: &UiTheme);
  /// fetch existing state for ui component
  fn node_fetch_prev(&self) -> UiNodeParams;
  /// pass back updated state for ui component
  fn node_set(&mut self, update: UiNodeParams);
  /// recursively calls update on all components - DO NOT REIMPLEMENT
  fn node_update(
    &mut self,
    can_act: &mut bool,
    action: &mut Option<(u32, UiEvent)>,
    cursor_icon: &mut CursorIcon,
    mouse_pos: &(f32, f32),
    mouse_delta: &(f32, f32),
    parent_rect: &Rect,
  ) {
    // handle dragging
    let prev_state = self.node_fetch_prev();
    let mut new_state = prev_state.clone();
    new_state.event = UiEvent::None;
    if prev_state.draggable_read_only && prev_state.holding {
      // update positions by being dragged
      new_state.abs_pos_size.x += mouse_delta.0;
      new_state.abs_pos_size.y += mouse_delta.1;
      new_state.rel_pos_size.x += mouse_delta.0;
      new_state.rel_pos_size.y += mouse_delta.1;
    } else {
      // maintain relative position from parent
      new_state.abs_pos_size.x = parent_rect.x + prev_state.rel_pos_size.x;
      new_state.abs_pos_size.y = parent_rect.y + prev_state.rel_pos_size.y;
    }
    // handle children, in reverse order
    match self.get_children_mut() {
      Some(children) => {
        for i in (0..children.len()).rev() {
          children[i].node_update(can_act, action, cursor_icon, mouse_pos, mouse_delta, &new_state.abs_pos_size);
        }
      }
      None => ()
    };
    let inbounds = self.is_mouse_in_bounds(mouse_pos);
    // handle click actions
    if *can_act && inbounds {
      *can_act = false;
      match prev_state.event {
        UiEvent::None | UiEvent::Hover | UiEvent::LRelease | UiEvent::RRelease => {
          new_state.event = UiEvent::Hover;
          if prev_state.show_hover_read_only {
            *cursor_icon = CursorIcon::Pointer;
          }
        }
        _ => ()
      };
      if is_mouse_button_pressed(MouseButton::Left) {
        new_state.event = UiEvent::LClick;
        new_state.holding = true;
        if prev_state.show_hover_read_only {
          *cursor_icon = CursorIcon::Pointer;
        }
      } else if is_mouse_button_released(MouseButton::Left) {
        if prev_state.holding {
          new_state.event = UiEvent::LRelease;
        }
        new_state.holding = false;
        if prev_state.show_hover_read_only {
          *cursor_icon = CursorIcon::Pointer;
        }
      } else if is_mouse_button_down(MouseButton::Left) {
        new_state.event = UiEvent::Hold;
        if prev_state.show_hover_read_only {
          *cursor_icon = CursorIcon::Pointer;
        }
      } else if is_mouse_button_pressed(MouseButton::Right) {
        new_state.event = UiEvent::RClick;
      } else if is_mouse_button_released(MouseButton::Right) {
        new_state.event = UiEvent::RRelease;
      }
      if *action == None {
        *action = Some((prev_state.id_read_only, new_state.event.clone()));
      }
    }
    // handle external action
    if !inbounds && is_mouse_button_pressed(MouseButton::Left) {
      new_state.event = UiEvent::LClickOuter;
    }
    self.node_set(new_state);
    self.update();
  }
  /// recursively calls render on all components - DO NOT REIMPLEMENT
  fn call_render(&self, theme: &UiTheme) {
    self.render(theme);
    match self.get_children() {
      Some(children) => {
        for i in 0..children.len() {
          children[i].render(theme);
        }
      }
      None => ()
    };
  }
}
impl std::fmt::Debug for dyn UiNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let stats = self.node_fetch_prev();
    write!(f, "UiNode{{{:?}}}", stats)
  }
}

#[derive(Debug)]
pub struct UiRoot<'a> {
  children: Vec<Box<dyn UiNode>>,
  prev_mouse_pos: (f32, f32),
  prev_cursor: CursorIcon,
  theme: UiTheme<'a>,
  id_counter: u32,
}
impl<'a> UiRoot<'a> {
  pub fn new() -> Self {
    Self {
      children: Vec::new(),
      prev_mouse_pos: (0.0, 0.0),
      prev_cursor: CursorIcon::Default,
      theme: UiTheme::default(),
      id_counter: 1,
    }
  }
  /// auto-incremented id generation, starting at 1
  pub fn get_id(&mut self) -> u32 {
    let id = self.id_counter;
    self.id_counter += 1;
    id
  }
  pub fn with_theme(mut self, theme: UiTheme<'a>) -> Self {
    self.theme = theme;
    self
  }
  pub fn with_child(mut self, node: impl UiNode + 'static) -> Self {
    self.children.push(Box::new(node));
    self
  }
  pub fn add_child(&mut self, node: impl UiNode + 'static) {
    self.children.push(Box::new(node));
  }
  pub fn update(&mut self) -> Option<(u32, UiEvent)> {
    if self.children.is_empty() { return None; }
    // update states
    let mut action_available = true;
    let mut action: Option<(u32, UiEvent)> = None;
    let mut cursor_icon = CursorIcon::Default;
    let mouse_pos = mouse_position();
    let mouse_delta = (
      mouse_pos.0 - self.prev_mouse_pos.0,
      mouse_pos.1 - self.prev_mouse_pos.1
    );
    self.prev_mouse_pos = mouse_pos;
    let scrn = Rect::new(0.0, 0.0, window::screen_width(), window::screen_height());
    // recursively update all nodes in tree
    for i in (0..self.children.len()).rev() {
      self.children[i].node_update(
        &mut action_available, &mut action, &mut cursor_icon, &mouse_pos, &mouse_delta, &scrn
      );
    }
    // update mouse cursor
    set_mouse_cursor(cursor_icon);
    action
  }
  pub fn render(&self) {
    if self.children.is_empty() { return; }
    for i in 0..self.children.len() {
      self.children[i].call_render(&self.theme);
    }
  }
}

#[derive(Debug)]
pub struct UiBox {
  id: u32,
  abs_pos_size: Rect,
  rel_pos_size: Rect,
  holding: bool,
  event: UiEvent,
  children: Vec<Box<dyn UiNode>>,
  draggable: bool,
  show_hover: bool,
  pub color: Color,
  pub hover_color: Color,
  pub shadow_color: Color,
}
impl UiNode for UiBox {
  fn get_children(&self) -> Option<&Vec<Box<dyn UiNode>>> {
    if self.children.is_empty() { None }
    else { Some(&self.children) }
  }
  fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn UiNode>>> {
    if self.children.is_empty() { None }
    else { Some(&mut self.children) }
  }
  fn is_mouse_in_bounds(&self, mouse_pos: &(f32, f32)) -> bool {
    point_in_rect(mouse_pos, &self.abs_pos_size)
  }
  fn node_fetch_prev(&self) -> UiNodeParams {
    UiNodeParams { 
      event: self.event.clone(),
      holding: self.holding,
      id_read_only: self.id,
      draggable_read_only: self.draggable,
      show_hover_read_only: self.show_hover,
      rel_pos_size: self.rel_pos_size.clone(),
      abs_pos_size: self.abs_pos_size.clone(),
    }
  }
  fn node_set(&mut self, update: UiNodeParams) {
    self.event = update.event;
    self.holding = update.holding;
    self.abs_pos_size = update.abs_pos_size;
    self.rel_pos_size = update.rel_pos_size;
  }
  fn update(&mut self) {}
  fn render(&self, _theme: &UiTheme) {
    let active_color = match self.event {
      UiEvent::Hover | UiEvent::Hold | UiEvent::LClick | UiEvent::LRelease => self.hover_color,
      _ => self.color
    };
    draw_rectangle(
      self.abs_pos_size.x - 1.0,
      self.abs_pos_size.y - 1.0,
      self.abs_pos_size.w + 4.0,
      self.abs_pos_size.h + 6.0,
      self.shadow_color,
    );
    draw_rectangle(
      self.abs_pos_size.x,
      self.abs_pos_size.y,
      self.abs_pos_size.w,
      self.abs_pos_size.h,
      active_color,
    );
  }
}
impl UiBox {
  pub fn new(id: u32, pos_size: Rect, draggable: bool, show_hover: bool) -> Self {
    Self {
      id,
      abs_pos_size: pos_size,
      rel_pos_size: pos_size,
      shadow_color: Color::from_rgba(0, 0, 0, 120),
      color: LIGHTGRAY,
      hover_color: GRAY,
      draggable,
      show_hover,
      children: Vec::new(),
      event: UiEvent::None,
      holding: false
    }
  }
  pub fn with_child(mut self, node: impl UiNode + 'static) -> Self {
    self.children.push(Box::new(node));
    self
  }
  pub fn add_child(&mut self, node: impl UiNode + 'static) {
    self.children.push(Box::new(node));
  }
}

#[derive(Debug)]
pub struct UiButton {
  id: u32,
  abs_pos_size: Rect,
  rel_pos_size: Rect,
  holding: bool,
  event: UiEvent,
  pub color: Color,
  pub hover_color: Color,
  pub down_color: Color,
  pub text_color: Color,
  text: String,
}
impl UiNode for UiButton {
  fn get_children(&self) -> Option<&Vec<Box<dyn UiNode>>> {
    None
  }
  fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn UiNode>>> {
    None
  }
  fn is_mouse_in_bounds(&self, mouse_pos: &(f32, f32)) -> bool {
    point_in_rect(mouse_pos, &self.abs_pos_size)
  }
  fn node_fetch_prev(&self) -> UiNodeParams {
    UiNodeParams {
      event: self.event.clone(),
      holding: self.holding,
      id_read_only: self.id,
      draggable_read_only: false,
      show_hover_read_only: true,
      rel_pos_size: self.rel_pos_size,
      abs_pos_size: self.abs_pos_size,
    }
  }
  fn node_set(&mut self, update: UiNodeParams) {
    self.event = update.event;
    self.holding = update.holding;
    self.abs_pos_size = update.abs_pos_size;
    self.rel_pos_size = update.rel_pos_size;
  }
  fn update(&mut self) {}
  fn render(&self, theme: &UiTheme) {
    let active_color = match self.event {
      UiEvent::Hover | UiEvent::LClick => self.hover_color,
      UiEvent::Hold | UiEvent::LRelease => self.down_color,
      _ => self.color
    };
    draw_rectangle(
      self.abs_pos_size.x,
      self.abs_pos_size.y,
      self.abs_pos_size.w,
      self.abs_pos_size.h,
      active_color,
    );
    // calculate text pos
    let txt_size = measure_text(&self.text, theme.font, theme.font_size, 1.0);
    let txt_x = self.abs_pos_size.x + (self.abs_pos_size.w - txt_size.width) / 2.0;
    let txt_y = self.abs_pos_size.y + txt_size.height + (self.abs_pos_size.h - txt_size.height) / 2.0;
    draw_text_ex(&self.text, txt_x, txt_y, TextParams {
      font: theme.font,
      font_size: theme.font_size,
      color: self.text_color,
      ..Default::default()
    });
    // draw border
    draw_rectangle_lines(
      self.abs_pos_size.x,
      self.abs_pos_size.y,
      self.abs_pos_size.w,
      self.abs_pos_size.h,
      1.5,
      BLACK,
    );
  }
}
impl UiButton {
  pub fn new(id: u32, pos_size: Rect, text: String) -> Self {
    Self {
      abs_pos_size: pos_size,
      rel_pos_size: pos_size,
      id,
      text,
      holding: false,
      event: UiEvent::None,
      color: LIGHTGRAY,
      hover_color: GRAY,
      down_color: DARKGRAY,
      text_color: BLACK,
    }
  }
}

#[derive(Debug)]
pub struct UiText {
  id: u32,
  abs_pos_size: Rect,
  rel_pos_size: Rect,
  event: UiEvent,
  pub color: Color,
  pub text: String,
}
impl UiNode for UiText {
  fn get_children(&self) -> Option<&Vec<Box<dyn UiNode>>> {
    None
  }
  fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn UiNode>>> {
    None
  }
  fn is_mouse_in_bounds(&self, _mouse_pos: &(f32, f32)) -> bool {
    false
  }
  fn node_fetch_prev(&self) -> UiNodeParams {
    UiNodeParams {
      event: self.event.clone(),
      holding: false,
      id_read_only: self.id,
      draggable_read_only: false,
      show_hover_read_only: false,
      rel_pos_size: self.rel_pos_size,
      abs_pos_size: self.abs_pos_size,
    }
  }
  fn node_set(&mut self, update: UiNodeParams) {
    self.event = update.event;
    self.abs_pos_size = update.abs_pos_size;
    self.rel_pos_size = update.rel_pos_size;
  }
  fn update(&mut self) {}
  fn render(&self, theme: &UiTheme) {
    let txt_size = measure_text(&self.text, theme.font, theme.font_size, 1.0);
    let txt_y = self.abs_pos_size.y + txt_size.height / 2.0;
    draw_text_ex(&self.text, self.abs_pos_size.x, txt_y, TextParams {
      font: theme.font,
      font_size: theme.font_size,
      color: self.color,
      ..Default::default()
    });
  }
}
impl UiText {
  pub fn new(id: u32, pos_size: Rect, text: String) -> Self {
    Self {
      id,
      abs_pos_size: pos_size,
      rel_pos_size: pos_size,
      color: BLACK,
      text,
      event: UiEvent::None,
    }
  }
}
