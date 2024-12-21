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

#[allow(dead_code)]
#[derive(Debug)]
pub struct UiGlobal<'a> {
  // window related data
  screen_size: (f32, f32),
  screen_resized: bool,
  mouse_pos: (f32, f32),
  mouse_delta: (f32, f32),
  mouse_l_state: MouseState,
  mouse_r_state: MouseState,
  // component action handling
  action_available: bool,
  held_id: u32,
  drag_ids: Vec<u32>,
  id_gen_tracker: u32,
  // theme
  font: Option<&'a Font>,
  pub clr_base: Color,
  pub clr_highlight: Color,
  pub clr_lowlight: Color,
  pub clr_contrast: Color,
  pub clr_accent_1: Color,
  pub clr_accent_2: Color,
  pub clr_body: Color,
  pub clr_warning: Color,
  pub clr_error: Color,
  pub clr_shadow: Color,
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
      action_available: true,
      held_id: 0,
      drag_ids: vec![],
      id_gen_tracker: 1,
      font: font,
      clr_base: Color::from_rgba(160, 160, 160, 255),
      clr_highlight: Color::from_rgba(180, 180, 180, 255),
      clr_lowlight: Color::from_rgba(140, 140, 140, 255),
      clr_contrast: Color::from_rgba(0, 0, 0, 255),
      clr_accent_1: Color::from_rgba(120, 120, 200, 255),
      clr_accent_2: Color::from_rgba(200, 120, 120, 255),
      clr_body: Color::from_rgba(0, 0, 0, 255),
      clr_warning: Color::from_rgba(200, 200, 10, 255),
      clr_error: Color::from_rgba(200, 10, 10, 255),
      clr_shadow: Color::from_rgba(0, 0, 0, 100),
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
  render_shadow: bool,
  active_color: Color,
}
impl<'b> UiBox<'b> {
  pub fn new(ui_global: Rc<RefCell<UiGlobal<'b>>>, pos_size: Rect, render_shadow: bool) -> Self {
    let id = ui_global.borrow_mut().get_new_id();
    let base_clr = ui_global.borrow().clr_base;
    Self {
      id: id,
      global: ui_global,
      pos_size: pos_size,
      render_shadow: render_shadow,
      active_color: base_clr
    }
  }
  pub fn update(&mut self) -> bool {
    let mut glb = self.global.borrow_mut();
    let evt = glb.component_update(self.id, &mut self.pos_size, true);
    match evt {
      UiEvent::LClick | UiEvent::Hold => {
        self.active_color = glb.clr_lowlight;
      }
      UiEvent::Hover => {
        self.active_color = glb.clr_highlight;
      }
      _ => {
        self.active_color = glb.clr_base;
      }
    };

    // return click event
    evt.clone() == UiEvent::LClick
  }
  pub fn render(&self) {
    if self.render_shadow {
      draw_rectangle(
        self.pos_size.x - 0.5, self.pos_size.y - 1.0, self.pos_size.w + 3.0, self.pos_size.h + 4.5,
        self.global.borrow().clr_shadow
      );
    }
    draw_rectangle(
      self.pos_size.x, self.pos_size.y, self.pos_size.w, self.pos_size.h,
      self.active_color
    );
  }
}