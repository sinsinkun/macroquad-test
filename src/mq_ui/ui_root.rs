use macroquad::prelude::*;
use miniquad::window::set_mouse_cursor;
use miniquad::CursorIcon;
use crate::mq_ui::*;

#[derive(Debug, Clone)]
pub struct UiRoot<'a> {
  pub theme: UiTheme<'a>,
  children: Vec<UiElement>,
  prev_mouse_pos: (f32, f32),
  id_counter: u32,
}
#[allow(unused)]
impl<'a> UiRoot<'a> {
  pub fn new() -> Self {
    Self {
      theme: UiTheme::default(),
      children: Vec::new(),
      prev_mouse_pos: (0.0, 0.0),
      id_counter: 1,
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
    let t_delta = get_frame_time();
    // update children
    update_children(
      &mut self.children,
      &mut action_target,
      &origin,
      &mouse_pos,
      &mouse_delta,
      &l_mouse,
      &r_mouse,
      &t_delta,
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
        UiAction::Hover | UiAction::Hold | UiAction::LClick | UiAction::LRelease => {
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
  pub fn render(&mut self) {
    render_children(&mut self.children, &self.theme, &WHITE);
  }
  pub fn add_child(&mut self, elem: UiElement) {
    self.children.push(elem);
  }
  pub fn find_element(&self, id: u32) -> Option<&UiElement> {
    find_node(&self.children, id)
  }
  pub fn new_id(&mut self) -> u32 {
    let id = self.id_counter;
    self.id_counter += 1;
    id
  }
}