#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

use macroquad::prelude::*;
use macroquad::window;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;

use crate::c_util::point_in_rect;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UiEvent{ None, Hover, Hold, LClickOuter, LClick, RClick, LRelease, RRelease }

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum MouseState{ None, Over, Down, Hold, Up }

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
  pub clr_contrast: Color,
  pub clr_accent_1: Color,
  pub clr_accent_2: Color,
  pub clr_warning: Color,
  pub clr_error: Color,
  pub clr_shadow: Color,
}
impl<'a> UiGlobal<'a> {
  pub fn new() -> Self {
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
      font: None,
      clr_base: Color::from_rgba(160, 160, 160, 255),
      clr_contrast: Color::from_rgba(0, 0, 0, 255),
      clr_accent_1: Color::from_rgba(120, 120, 200, 255),
      clr_accent_2: Color::from_rgba(200, 120, 120, 255),
      clr_warning: Color::from_rgba(200, 200, 10, 255),
      clr_error: Color::from_rgba(200, 10, 10, 255),
      clr_shadow: Color::from_rgba(0, 0, 0, 100),
    }
  }
  pub fn with_font(mut self, font: &'a Font) -> Self {
    self.font = Some(font);
    self
  }
  pub fn attach_font(&mut self, font: &'a Font) {
    self.font = Some(font);
  }
  pub fn get_new_id(&mut self) -> u32 {
    let id = self.id_gen_tracker;
    self.id_gen_tracker += 1;
    id
  }
  /** Note: needs to happen before all other UI updates */
  pub fn update_start(&mut self) {
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
    }
  }
  pub fn component_update(&mut self, id: u32, pos_size: &mut Rect) -> UiEvent {
    let mut evt = UiEvent::None;
    let inbounds = point_in_rect(&self.mouse_pos, pos_size);
    // handle click action
    if self.action_available && inbounds {
      self.action_available = false;
      evt = UiEvent::Hover;
      if self.mouse_l_state < MouseState::Over { self.mouse_l_state = MouseState::Over; }
      if self.mouse_l_state == MouseState::Down {
        evt = UiEvent::LClick;
        self.held_id = id;
      } else if self.mouse_l_state == MouseState::Up && self.held_id == id {
        evt = UiEvent::LRelease;
      } else if self.mouse_r_state == MouseState::Down {
        evt = UiEvent::RClick;
      } else if self.mouse_r_state == MouseState::Up && self.held_id == id {
        evt = UiEvent::RRelease;
      }
    }
    // handle click outside action
    if self.mouse_l_state == MouseState::Down && !inbounds {
      evt = UiEvent::LClickOuter;
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
  /** Note: needs to happen after all other UI updates */
  pub fn update_end(&mut self) {
    if self.held_id != 0 && self.mouse_l_state == MouseState::Up {
      // release held objects
      self.held_id = 0;
      self.drag_ids.clear();
    }
    if self.action_available == false {
      match self.mouse_l_state {
        MouseState::Hold => {
          if self.drag_ids.is_empty() {
            set_mouse_cursor(CursorIcon::Pointer);
          } else {
            set_mouse_cursor(CursorIcon::Move);
          }
        }
        MouseState::Over | MouseState::Down => set_mouse_cursor(CursorIcon::Pointer),
        _ => set_mouse_cursor(CursorIcon::Default),
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
  pub fn add_drag_component(&mut self, id: u32) {
    self.drag_ids.push(id);
  }
}

#[derive(Debug)]
pub struct UiBox<'a> {
  id: u32,
  child_ids: Vec<u32>,
  global: Rc<RefCell<UiGlobal<'a>>>,
  pos_size: Rect,
  draggable: bool,
  render_shadow: bool,
  pub bg_color: Color,
  pub hover_color: Color,
  pub click_color: Color,
  active_color: Color,
}
impl<'a> UiBox<'a> {
  pub fn new(
    ui_global: Rc<RefCell<UiGlobal<'a>>>, pos_size: Rect, render_shadow: bool, draggable: bool
  ) -> Self {
    let id = ui_global.borrow_mut().get_new_id();
    let base_clr = ui_global.borrow().clr_base;
    let hl = Color::new(base_clr.r + 0.1, base_clr.g + 0.1, base_clr.b + 0.1, base_clr.a);
    let cl = Color::new(base_clr.r + 0.2, base_clr.g + 0.2, base_clr.b + 0.2, base_clr.a);
    Self {
      id: id,
      child_ids: Vec::new(),
      global: ui_global,
      pos_size: pos_size,
      draggable: draggable,
      render_shadow: render_shadow,
      bg_color: base_clr,
      hover_color: hl,
      click_color: cl,
      active_color: base_clr
    }
  }
  /** Note: updates should happen in reverse order of render order
    so top layer components handle inputs first
  */
  pub fn update(&mut self) {
    let mut glb = self.global.borrow_mut();
    let evt = glb.component_update(self.id, &mut self.pos_size);
    match evt {
      UiEvent::LClick => {
        self.active_color = self.hover_color;
        if self.draggable {
          glb.add_drag_component(self.id);
          for id in &self.child_ids {
            glb.add_drag_component(id.clone());
          }
        }
      }
      UiEvent::Hold => {
        self.active_color = self.click_color;
      }
      UiEvent::Hover => {
        self.active_color = self.hover_color;
      }
      _ => {
        self.active_color = self.bg_color;
      }
    };
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
  pub fn get_id(&self) -> u32 {
    self.id
  }
  pub fn attach_child(&mut self, id: u32) {
    self.child_ids.push(id);
  }
  pub fn attach_children(&mut self, ids: Vec<u32>) {
    let mut owned = ids;
    self.child_ids.append(&mut owned);
  }
}

#[derive(Debug)]
pub struct UiButton<'a> {
  id: u32,
  global: Rc<RefCell<UiGlobal<'a>>>,
  pos_size: Rect,
  text: String,
  pub bg_color: Color,
  pub hover_color: Color,
  pub click_color: Color,
  pub txt_color: Color,
  active_color: Color,
}
impl<'a> UiButton<'a> {
  pub fn new(
    ui_global: Rc<RefCell<UiGlobal<'a>>>,
    pos_size: Rect,
    text: String
  ) -> Self {
    let id = ui_global.borrow_mut().get_new_id();
    let base_clr = ui_global.borrow().clr_base;
    let txt_clr = ui_global.borrow().clr_contrast;
    let bg = Color::new(base_clr.r - 0.05, base_clr.g, base_clr.b + 0.05, base_clr.a);
    let bg2 = Color::new(base_clr.r + 0.05, base_clr.g + 0.1, base_clr.b + 0.15, base_clr.a);
    let bg3 = Color::new(base_clr.r + 0.15, base_clr.g + 0.2, base_clr.b + 0.25, base_clr.a);
    Self {
      id: id,
      global: ui_global,
      pos_size: pos_size,
      text: text,
      bg_color: bg,
      hover_color: bg2,
      click_color: bg3,
      txt_color: txt_clr,
      active_color: bg,
    }
  }
  /** Note: updates should happen in reverse order of render order
    so top layer components handle inputs first
  */
  pub fn update(&mut self) -> bool {
    let mut glb = self.global.borrow_mut();
    let evt = glb.component_update(self.id, &mut self.pos_size);
    match evt {
      UiEvent::LClick | UiEvent::Hold => {
        self.active_color = self.click_color;
      }
      UiEvent::Hover => {
        self.active_color = self.hover_color;
      }
      _ => {
        self.active_color = self.bg_color;
      }
    };
    evt == UiEvent::LRelease
  }
  pub fn render(&self) {
    let glb = self.global.borrow();
    draw_rectangle(
      self.pos_size.x, self.pos_size.y, self.pos_size.w, self.pos_size.h,
      self.active_color
    );
    let txt_size = measure_text(&self.text, glb.font, 20, 1.0);
    let txt_x = self.pos_size.x + (self.pos_size.w - txt_size.width) / 2.0;
    let txt_y = self.pos_size.y + (self.pos_size.h - txt_size.height);
    draw_text_ex(&self.text, txt_x, txt_y, TextParams {
      font: glb.font,
      font_size: 20,
      font_scale: 1.0,
      color: glb.clr_contrast,
      ..Default::default()
    });
    draw_rectangle_lines(
      self.pos_size.x, self.pos_size.y, self.pos_size.w, self.pos_size.h,
      2.0, glb.clr_contrast
    );
  }
  pub fn get_id(&self) -> u32 {
    self.id
  }
}