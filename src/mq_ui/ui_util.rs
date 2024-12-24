use macroquad::prelude::*;
use crate::mq_ui::*;

pub(crate) fn get_mouse_actions() -> (UiMouseAction, UiMouseAction) {
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

pub(crate) fn update_position(
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

pub(crate) fn update_children(
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

pub(crate) fn render_children(children: &Vec<UiElement>, theme: &UiTheme) {
  for elem in children {
    match elem {
      UiElement::Box(e) => { e.render(&theme); }
      UiElement::Text(e) => { e.render(&theme); }
      UiElement::Button(e) => { e.render(&theme); }
      UiElement::Input(e) => { e.render(&theme); }
    }
  }
}

pub(crate) fn update_event<'a>(
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

pub fn point_in_rect(point: &(f32, f32), rect: &Rect) -> bool {
  let x_in = 
    if point.0 > rect.x && point.0 < rect.x + rect.w { true }
    else { false };
  let y_in =
    if point.1 > rect.y && point.1 < rect.y + rect.h { true }
    else { false };
  x_in && y_in
}
