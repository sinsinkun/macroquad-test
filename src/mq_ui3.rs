#![allow(dead_code)]

use macroquad::prelude::*;
use macroquad::color::colors;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;

use crate::mq_util::point_in_rect;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UiEvent{ None, Hover, Hold, LClickOuter, LClick, RClick, LRelease, RRelease }

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UiMouseAction{ None, Down, Hold, Release }

#[derive(Debug, Clone)]
pub struct UiTheme<'a> {
  pub font: Option<&'a Font>,
  pub font_size: u16,
  pub base_color: Color,
  pub contrast_color: Color,
  pub high_color: Color,
  pub low_color: Color,
  pub accent_color_1: Color,
  pub accent_color_2: Color,
  pub shadow_color: Color,
}
impl Default for UiTheme<'_> {
  fn default() -> Self {
    Self {
      font: None,
      font_size: 18,
      base_color: colors::GRAY,
      contrast_color: colors::BLACK,
      high_color: colors::LIGHTGRAY,
      low_color: colors::DARKGRAY,
      accent_color_1: colors::BLUE,
      accent_color_2: colors::LIME,
      shadow_color: Color::from_rgba(0, 0, 0, 120),
    }
  }
}

#[derive(Debug, Clone)]
pub enum UiElement {
  Box(UiBox),
  Text(UiText),
  Button(UiButton),
}

// --- --- --- --- --- --- --- --- --- --- //
// --- --- -- -- HELPER UTILS -- -- -- --- //
// --- --- --- --- --- --- --- --- --- --- //
fn update_position(
  abs_origin: &mut (f32, f32),
  rel_origin: &mut (f32, f32),
  parent_origin: &(f32, f32),
  mouse_delta: &(f32, f32),
  draggable: bool,
  holding: bool,
) {
  if draggable && holding {
    abs_origin.0 += mouse_delta.0;
    abs_origin.1 += mouse_delta.1;
    rel_origin.0 += mouse_delta.0;
    rel_origin.1 += mouse_delta.1;
  } else {
    // maintain relative distance from parent
    abs_origin.0 = parent_origin.0 + rel_origin.0;
    abs_origin.1 = parent_origin.1 + rel_origin.1;
  }
}

fn update_event<'a>(
  action_available: &mut bool,
  inbounds: bool,
  holding: &mut bool,
  prev_event: &UiEvent,
  l_mouse: &UiMouseAction,
  r_mouse: &UiMouseAction,
) -> UiEvent {
  let mut evt = UiEvent::None;
  if *action_available && inbounds {
    *action_available = false;
    match prev_event {
      UiEvent::None | UiEvent::Hover | UiEvent::LRelease => {
        evt = UiEvent::Hover;
      }
      _ => ()
    }
    if l_mouse == &UiMouseAction::Down {
      evt = UiEvent::LClick;
      *holding = true;
    } else if l_mouse == &UiMouseAction::Hold {
      evt = UiEvent::Hold;
    } else if l_mouse == &UiMouseAction::Release {
      if *holding { evt = UiEvent::LRelease; }
      *holding = false;
    } else if r_mouse == &UiMouseAction::Down {
      evt = UiEvent::RClick;
    } else if r_mouse == &UiMouseAction::Release {
      evt = UiEvent::RRelease;
    }
  }
  if !inbounds {
    if l_mouse == &UiMouseAction::Down {
      evt = UiEvent::LClickOuter;
    }
    if l_mouse == &UiMouseAction::Release && *holding {
      *holding = false;
    }
  }
  evt
}

// --- --- --- --- --- --- --- --- --- --- //
// --- --- --- --- ELEMENTS -- --- --- --- //
// --- --- --- --- --- --- --- --- --- --- //
#[derive(Debug, Clone)]
pub struct UiRoot<'a> {
  theme: UiTheme<'a>,
  children: Vec<UiElement>,
  prev_mouse_pos: (f32, f32),
}
impl<'a> UiRoot<'a> {
  pub fn new() -> Self {
    Self {
      theme: UiTheme::default(),
      children: Vec::new(),
      prev_mouse_pos: (0.0, 0.0),
    }
  }
  pub fn with_theme(mut self, theme: UiTheme<'a>) -> Self {
    self.theme = theme;
    self
  }
  pub fn update(&mut self) -> Option<UiElement> {
    if self.children.is_empty() { return None; }
    // setup transient state
    let mut action_target = None;
    let mouse_pos = mouse_position();
    let mouse_delta = (
      mouse_pos.0 - self.prev_mouse_pos.0,
      mouse_pos.1 - self.prev_mouse_pos.1
    );
    self.prev_mouse_pos = mouse_pos;
    let origin = (0.0, 0.0);
    let mut l_mouse = UiMouseAction::None;
    let mut r_mouse = UiMouseAction::None;
    if is_mouse_button_pressed(MouseButton::Left) { l_mouse = UiMouseAction::Down; }
    else if is_mouse_button_released(MouseButton::Left) { l_mouse = UiMouseAction::Release; }
    else if is_mouse_button_down(MouseButton::Left) { l_mouse = UiMouseAction::Hold; }
    if is_mouse_button_pressed(MouseButton::Right) { r_mouse = UiMouseAction::Down; }
    else if is_mouse_button_released(MouseButton::Right) { r_mouse = UiMouseAction::Release; }
    else if is_mouse_button_down(MouseButton::Right) { r_mouse = UiMouseAction::Hold; }
    // update children in reverse order
    for elem in self.children.iter_mut().rev() {
      match elem {
        UiElement::Box(e) => {
          e.update(&mut action_target, &origin, &mouse_pos, &mouse_delta, &l_mouse, &r_mouse);
        }
        UiElement::Text(e) => {
          e.update(&mut action_target, &origin, &mouse_pos, &mouse_delta, &l_mouse, &r_mouse);
        }
        UiElement::Button(e) => {
          e.update(&mut action_target, &origin, &mouse_pos, &mouse_delta, &l_mouse, &r_mouse);
        }
      }
    }
    // update cursor
    let mut cursor_icon = CursorIcon::Default;
    if action_target.is_some() {
      let (event, show_hover) = match action_target.as_ref().unwrap() {
        UiElement::Box(e) => (e.event.clone(), e.show_hover),
        UiElement::Button(e) => (e.event.clone(), true),
        UiElement::Text(e) => (e.event.clone(), false),
      };
      match event {
        UiEvent::Hover => {
          if show_hover { cursor_icon = CursorIcon::Pointer; }
          else { cursor_icon = CursorIcon::Default; }
        }
        UiEvent::Hold | UiEvent::LClick | UiEvent::LRelease => {
          cursor_icon = CursorIcon::Pointer;
        }
        _ => ()
      };
    }
    set_mouse_cursor(cursor_icon);
    // surface action target
    action_target
  }
  pub fn render(&self) {
    for elem in &self.children {
      match elem {
        UiElement::Box(e) => { e.render(&self.theme); }
        UiElement::Text(e) => { e.render(&self.theme); }
        UiElement::Button(e) => { e.render(&self.theme); }
      }
    }
  }
  pub fn add_child(&mut self, elem: UiElement) {
    self.children.push(elem);
  }
}

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
  show_hover: bool,
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
  fn update(
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
      self.draggable,
      self.holding,
    );
    // update children in reverse order
    for elem in self.children.iter_mut().rev() {
      match elem {
        UiElement::Box(e) => {
          e.update(target, &self.abs_origin, mouse_pos, mouse_delta, l_mouse, r_mouse);
        }
        UiElement::Text(e) => {
          e.update(target, &self.abs_origin, mouse_pos, mouse_delta, l_mouse, r_mouse);
        }
        UiElement::Button(e) => {
          e.update(target, &self.abs_origin, mouse_pos, mouse_delta, l_mouse, r_mouse);
        }
      }
    }
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
  fn render(&self, theme: &UiTheme) {
    let active_color = match self.event {
      UiEvent::Hover | UiEvent::Hold | UiEvent::LClick | UiEvent::LRelease => theme.high_color,
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
    for elem in &self.children {
      match elem {
        UiElement::Box(e) => { e.render(theme); }
        UiElement::Text(e) => { e.render(theme); }
        UiElement::Button(e) => { e.render(theme); }
      }
    }
  }
  pub fn add_child(&mut self, elem: UiElement) {
    self.children.push(elem);
  }
}

#[derive(Debug, Clone)]
pub struct UiText {
  pub id: u32,
  pub event: UiEvent,
  holding: bool,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  draggable: bool,
  text: String,
}
impl UiText {
  pub fn new(id: u32, pos_size: Rect, text: String, draggable: bool) -> Self {
    Self {
      id,
      event: UiEvent::None,
      holding: false,
      origin: (pos_size.x, pos_size.y),
      abs_origin: (pos_size.x, pos_size.y),
      size: (pos_size.w, pos_size.h),
      draggable,
      text,
    }
  }
  fn update(
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
      self.draggable,
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
    // clone self into target
    if !action_available && target.is_none() {
      target.replace(UiElement::Text(self.clone()));
    }
  }
  fn render(&self, theme: &UiTheme) {
    let txt_size = measure_text(&self.text, theme.font, theme.font_size, 1.0);
    let txt_y = self.abs_origin.1 + txt_size.height / 2.0;
    draw_text_ex(&self.text, self.abs_origin.0, txt_y, TextParams {
      font: theme.font,
      font_size: theme.font_size,
      color: theme.contrast_color,
      ..Default::default()
    });
  }
}

#[derive(Debug, Clone)]
pub struct UiButton {
  pub id: u32,
  pub event: UiEvent,
  holding: bool,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  text: String,
}
impl UiButton {
  pub fn new(id: u32, pos_size: Rect, text: String) -> Self {
    Self {
      id,
      event: UiEvent::None,
      holding: false,
      origin: (pos_size.x, pos_size.y),
      abs_origin: (pos_size.x, pos_size.y),
      size: (pos_size.w, pos_size.h),
      text,
    }
  }
  fn update(
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
    // clone self into target
    if !action_available && target.is_none() {
      target.replace(UiElement::Button(self.clone()));
    }
  }
  fn render(&self, theme: &UiTheme) {
    let active_color = match self.event {
      UiEvent::Hover | UiEvent::LClick => theme.high_color,
      UiEvent::Hold | UiEvent::LRelease => theme.accent_color_1,
      _ => theme.base_color
    };
    draw_rectangle(
      self.abs_origin.0,
      self.abs_origin.1,
      self.size.0,
      self.size.1,
      active_color,
    );
    // calculate text pos
    let txt_size = measure_text(&self.text, theme.font, theme.font_size, 1.0);
    let txt_x = self.abs_origin.0 + (self.size.0 - txt_size.width) / 2.0;
    let txt_y = self.abs_origin.1 + txt_size.height + (self.size.1 - txt_size.height) / 2.0;
    draw_text_ex(&self.text, txt_x, txt_y, TextParams {
      font: theme.font,
      font_size: theme.font_size,
      color: theme.contrast_color,
      ..Default::default()
    });
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
