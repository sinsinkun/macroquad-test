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
      accent_color_1: colors::BLUE,
      accent_color_2: colors::LIME,
      shadow_color: Color::from_rgba(0, 0, 0, 120),
    }
  }
}
impl UiTheme<'_> {
  fn base_color_plus(&self, n: f32) -> Color {
    let val = n / 100.0;
    let mut clr = self.base_color;
    clr.r += val;
    clr.g += val;
    clr.b += val;
    clr
  }
  fn contrast_color_plus(&self, n: f32) -> Color {
    let val = n / 100.0;
    let mut clr = self.contrast_color;
    clr.r += val;
    clr.g += val;
    clr.b += val;
    clr
  }
}

#[derive(Debug, Clone)]
pub enum UiElement {
  Box(UiBox),
  Text(UiText),
  Button(UiButton),
  Input(UiInput),
}

// --- --- --- --- --- --- --- --- --- --- //
// --- --- -- -- HELPER UTILS -- -- -- --- //
// --- --- --- --- --- --- --- --- --- --- //
fn get_mouse_actions() -> (UiMouseAction, UiMouseAction) {
  let mut l_mouse = UiMouseAction::None;
  let mut r_mouse = UiMouseAction::None;
  if is_mouse_button_pressed(MouseButton::Left) { l_mouse = UiMouseAction::Down; }
  else if is_mouse_button_released(MouseButton::Left) { l_mouse = UiMouseAction::Release; }
  else if is_mouse_button_down(MouseButton::Left) { l_mouse = UiMouseAction::Hold; }
  if is_mouse_button_pressed(MouseButton::Right) { r_mouse = UiMouseAction::Down; }
  else if is_mouse_button_released(MouseButton::Right) { r_mouse = UiMouseAction::Release; }
  else if is_mouse_button_down(MouseButton::Right) { r_mouse = UiMouseAction::Hold; }
  (l_mouse, r_mouse)
}

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

fn update_children(
  children: &mut Vec<UiElement>, 
  target: &mut Option<UiElement>,
  parent_origin: &(f32, f32),
  mouse_pos: &(f32, f32),
  mouse_delta: &(f32, f32),
  l_mouse: &UiMouseAction,
  r_mouse: &UiMouseAction,
) {
  // update children in reverse order
  for elem in children.iter_mut().rev() {
    match elem {
      UiElement::Box(e) => {
        e.update(target, parent_origin, &mouse_pos, &mouse_delta, &l_mouse, &r_mouse);
      }
      UiElement::Text(e) => {
        e.update(target, parent_origin, &mouse_pos, &mouse_delta, &l_mouse, &r_mouse);
      }
      UiElement::Button(e) => {
        e.update(target, parent_origin, &mouse_pos, &mouse_delta, &l_mouse, &r_mouse);
      }
      UiElement::Input(e) => {
        e.update(target, parent_origin, &mouse_pos, &mouse_delta, &l_mouse, &r_mouse);
      }
    }
  }
}

fn render_children(children: &Vec<UiElement>, theme: &UiTheme) {
  for elem in children {
    match elem {
      UiElement::Box(e) => { e.render(&theme); }
      UiElement::Text(e) => { e.render(&theme); }
      UiElement::Button(e) => { e.render(&theme); }
      UiElement::Input(e) => { e.render(&theme); }
    }
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
  if !inbounds && l_mouse == &UiMouseAction::Down {
    evt = UiEvent::LClickOuter;
  }
  if l_mouse == &UiMouseAction::Release && *holding {
    *holding = false;
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
    let (l_mouse, r_mouse) = get_mouse_actions();
    // update children
    update_children(
      &mut self.children,
      &mut action_target,
      &origin,
      &mouse_pos,
      &mouse_delta,
      &l_mouse,
      &r_mouse
    );
    // update cursor
    let mut cursor_icon = CursorIcon::Default;
    if action_target.is_some() {
      let event;
      let show_hover;
      let mut text_input = false;
      match action_target.as_ref().unwrap() {
        UiElement::Box(e) => {
          event = e.event.clone();
          show_hover = e.show_hover;
        }
        UiElement::Button(e) => {
          event = e.event.clone();
          show_hover = true;
        }
        UiElement::Text(e) => {
          event = e.event.clone();
          show_hover = false;
        },
        UiElement::Input(e) => {
          event = e.event.clone();
          show_hover = true;
          text_input = true;
        },
      };
      match event {
        UiEvent::Hover | UiEvent::Hold | UiEvent::LClick | UiEvent::LRelease => {
          if text_input { cursor_icon = CursorIcon::Text; }
          else if show_hover { cursor_icon = CursorIcon::Pointer; }
          else { cursor_icon = CursorIcon::Default; }
        }
        _ => ()
      };
    }
    set_mouse_cursor(cursor_icon);
    // surface action target
    action_target
  }
  pub fn render(&self) {
    render_children(&self.children, &self.theme);
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
    // update children
    update_children(
      &mut self.children,
      target,
      &self.abs_origin,
      mouse_pos,
      mouse_delta,
      l_mouse,
      r_mouse
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
  fn render(&self, theme: &UiTheme) {
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
      UiEvent::Hover | UiEvent::LClick => theme.base_color_plus(20.0),
      UiEvent::Hold | UiEvent::LRelease => theme.base_color_plus(40.0),
      _ => theme.base_color_plus(10.0)
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

#[derive(Debug, Clone)]
pub struct UiInput {
  pub id: u32,
  pub event: UiEvent,
  holding: bool,
  origin: (f32, f32),
  abs_origin: (f32, f32),
  size: (f32, f32),
  is_active: bool,
  pub input: String,
  pub placeholder: String,
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
    }
    // clone self into target
    if !action_available && target.is_none() {
      target.replace(UiElement::Input(self.clone()));
    }
  }
  fn render(&self, theme: &UiTheme) {
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
    if self.is_active || !self.input.is_empty() {
      let txt_size = measure_text(&self.input, theme.font, theme.font_size, 1.0);
      let txt_x = self.abs_origin.0 + 5.0;
      let txt_y = self.abs_origin.1 + 5.0 + txt_size.height;
      draw_text_ex(&self.input, txt_x, txt_y, TextParams {
        font: theme.font,
        font_size: theme.font_size,
        color: theme.contrast_color,
        ..Default::default()
      });
    } else if !self.placeholder.is_empty() {
      let txt_size = measure_text(&self.placeholder, theme.font, theme.font_size, 1.0);
      let txt_x = self.abs_origin.0 + 5.0;
      let txt_y = self.abs_origin.1 + 5.0 + txt_size.height;
      draw_text_ex(&self.placeholder, txt_x, txt_y, TextParams {
        font: theme.font,
        font_size: theme.font_size,
        color: theme.contrast_color_plus(30.0),
        ..Default::default()
      });
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