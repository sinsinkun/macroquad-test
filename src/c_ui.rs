#![allow(unused)]

use std::rc::Rc;
use std::cell::RefCell;

use macroquad::prelude::*;
use macroquad::window;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;

use crate::c_util::point_in_rect;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UiEvent{ None, Hover, Hold, LClick, RClick, LRelease, RRelease }

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum MouseState{ None, Hover, Down, Hold, Up }

#[derive(Debug, PartialEq, Clone)]
enum CursorState{ Default, Hand }

#[derive(Debug)]
pub struct UiGlobal<'a> {
  screen_size: (f32, f32),
  screen_resized: bool,
  mouse_pos: (f32, f32),
  mouse_delta: (f32, f32),
  mouse_l_state: MouseState,
  mouse_r_state: MouseState,
  cursor_state: CursorState,
  font: Option<&'a Font>,
  action_available: bool,
  held_id: u32,
  drag_ids: Vec<u32>,
  id_gen_tracker: u32,
}
impl<'a> UiGlobal<'a> {
  pub fn new(font: Option<&'a Font>) -> Self {
    UiGlobal {
      screen_size: (0.0, 0.0),
      screen_resized: false,
      mouse_pos: (0.0, 0.0),
      mouse_delta: (0.0, 0.0),
      mouse_l_state: MouseState::None,
      mouse_r_state: MouseState::None,
      cursor_state: CursorState::Default,
      font: font,
      action_available: true,
      held_id: 0,
      drag_ids: vec![],
      id_gen_tracker: 1,
    }
  }
  pub fn get_new_id(&mut self) -> u32 {
    let id = self.id_gen_tracker;
    self.id_gen_tracker += 1;
    id
  }
  pub fn update(&mut self) {
    // transient states
    let sw = window::screen_width();
    let sh = window::screen_height();
    if sw != self.screen_size.0 || sh != self.screen_size.1 {
      self.screen_resized = true;
    } else {
      self.screen_resized = false;
    }
    self.screen_size.0 = sw;
    self.screen_size.1 = sh;
    let mpos = mouse_position();
    self.mouse_delta.0 = mpos.0 - self.mouse_pos.0;
    self.mouse_delta.1 = mpos.1 - self.mouse_pos.1;
    self.mouse_pos = mpos;
    self.action_available = true;
    // track mouse btns
    self.mouse_l_state = MouseState::None;
    if is_mouse_button_pressed(MouseButton::Left) {
      self.mouse_l_state = MouseState::Down;
    } else if is_mouse_button_released(MouseButton::Left) {
      self.mouse_l_state = MouseState::Up;
    } else if is_mouse_button_down(MouseButton::Left) {
      self.mouse_l_state = MouseState::Hold;
    }
    self.mouse_r_state = MouseState::None;
    if is_mouse_button_pressed(MouseButton::Right) {
      self.mouse_r_state = MouseState::Down;
    } else if is_mouse_button_released(MouseButton::Right) {
      self.mouse_r_state = MouseState::Up;
    } else if is_mouse_button_down(MouseButton::Right) {
      self.mouse_r_state = MouseState::Hold;
    }
    // check hold state
    if self.held_id != 0 && self.mouse_l_state == MouseState::Hold {
      // prevent actions when holding
      self.action_available = false;
    } else if self.held_id != 0 && self.mouse_l_state == MouseState::Up {
      // release held objects
      self.held_id = 0;
      self.drag_ids.clear();
    }
  }
  pub fn component_update(
    &mut self, id: u32, pos_size: &mut Rect, draggable: bool
  ) -> UiEvent {
    let mut evt = UiEvent::None;
    // handle click action
    if self.action_available && point_in_rect(&self.mouse_pos, pos_size) {
      self.action_available = false;
      evt = UiEvent::Hover;
      if self.mouse_l_state < MouseState::Hover { self.mouse_l_state = MouseState::Hover; }
      if self.mouse_l_state == MouseState::Down {
        evt = UiEvent::LClick;
        self.held_id = id;
        if draggable { self.drag_ids.push(id); }
      } else if self.mouse_l_state == MouseState::Up {
        evt = UiEvent::LRelease;
      } else if self.mouse_r_state == MouseState::Down {
        evt = UiEvent::RClick;
      } else if self.mouse_r_state == MouseState::Up {
        evt = UiEvent::RRelease;
      }
    }
    // handle hold action
    if self.mouse_l_state == MouseState::Hold && self.is_holding_component(id) {
      evt = UiEvent::Hold;
    }
    // handle drag action
    if self.mouse_l_state == MouseState::Hold && self.is_dragging_component(id) {
      if evt < UiEvent::Hold { evt = UiEvent::Hold; }
      pos_size.x += self.mouse_delta.0;
      pos_size.y += self.mouse_delta.1;
    }
    evt
  }
  pub fn update_cursor(&self) {
    if self.action_available == false {
      match self.mouse_l_state {
        MouseState::None => set_mouse_cursor(CursorIcon::Default),
        _ => set_mouse_cursor(CursorIcon::Pointer),
      };
    } else {
      set_mouse_cursor(CursorIcon::Default)
    }
  }
  pub fn is_holding_component(&self, id: u32) -> bool {
    self.held_id == id
  }
  pub fn is_dragging_component(&self, id: u32) -> bool {
    for iid in &self.drag_ids {
      if id == *iid {
        return true;
      }
    }
    false
  }
}

#[derive(Debug)]
pub struct UiBox<'b> {
  id: u32,
  global: Rc<RefCell<UiGlobal<'b>>>,
  pos_size: Rect,
  pub bg_color: Color,
  pub hover_color: Color,
  pub down_color: Color,
  active_color: Color,
}
impl<'b> UiBox<'b> {
  pub fn new(ui_global: Rc<RefCell<UiGlobal<'b>>>, pos_size: Rect) -> Self {
    let id = ui_global.borrow_mut().get_new_id();
    Self {
      id: id,
      global: ui_global,
      pos_size: pos_size,
      bg_color: Color::from_rgba(200, 200, 200, 255),
      hover_color: Color::from_rgba(220, 220, 220, 255),
      down_color: Color::from_rgba(180, 180, 180, 255),
      active_color: Color::from_rgba(200, 200, 200, 255)
    }
  }
  pub fn update(&mut self) -> bool {
    let mut glb = self.global.borrow_mut();
    let evt = glb.component_update(self.id, &mut self.pos_size, true);
    match evt {
      UiEvent::LClick | UiEvent::Hold => {
        self.active_color = self.down_color;
      }
      UiEvent::Hover => {
        self.active_color = self.hover_color;
      }
      _ => {
        self.active_color = self.bg_color;
      }
    };
    
    // return click event
    evt.clone() == UiEvent::LClick
  }
  pub fn render(&self) {
    draw_rectangle(
      self.pos_size.x, self.pos_size.y, self.pos_size.w, self.pos_size.h,
      self.active_color
    );
  }
}