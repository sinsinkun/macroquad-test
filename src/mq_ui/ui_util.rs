use macroquad::prelude::*;
use crate::mq_ui::*;

pub fn point_in_rect(point: &(f32, f32), rect: &Rect) -> bool {
  let x_in = 
    if point.0 > rect.x && point.0 < rect.x + rect.w { true }
    else { false };
  let y_in =
    if point.1 > rect.y && point.1 < rect.y + rect.h { true }
    else { false };
  x_in && y_in
}

pub fn contrast_color(bg_color: &Color) -> Color {
  let brightness = bg_color.r * 0.299 + bg_color.g * 0.587 + bg_color.b * 0.114;
  if brightness > 0.7294 { BLACK } else { WHITE }
}

pub fn adjust_alpha(color: &Color, alpha: f32) -> Color {
  let mut c = color.clone();
  c.a = alpha;
  c
}

pub fn mix_colors(color_1: &Color, color_2: &Color, percent: f32) -> Color {
  if percent <= 0.0 { return color_1.clone(); }
  if percent >= 1.0 { return color_2.clone(); }
  let mut c = BLACK;
  c.r = (1.0 - percent) * color_1.r + percent * color_2.r;
  c.g = (1.0 - percent) * color_1.g + percent * color_2.g;
  c.b = (1.0 - percent) * color_1.b + percent * color_2.b;
  c.a = (1.0 - percent) * color_1.a + percent * color_2.a;
  c
}

pub(crate) fn rect_subtract(a: &Rect, b: &Rect) -> Rect {
  Rect {
    x: a.x - b.x,
    y: a.y - b.y,
    w: a.w - b.w,
    h: a.h - b.h,
  }
}

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

pub(crate) fn update_position_adv(
  prev_abs_bounds: &Rect,
  prev_rel_bounds: &UiRect,
  parent_rect: &Rect,
  parent_delta: &(f32, f32),
  alignment: &UiAlign,
  mouse_delta: &(f32, f32),
  draggable: bool,
  holding: bool,
) -> (Rect, UiRect) {
  let mut abs_bounds = *prev_abs_bounds;
  let mut rel_bounds = *prev_rel_bounds;
  // maintain relative size
  if rel_bounds.w.is_px() {
    abs_bounds.w = rel_bounds.w.value();
  }
  if rel_bounds.w.is_percent() {
    abs_bounds.w = rel_bounds.w.value() * parent_rect.w;
  }
  if rel_bounds.h.is_px() {
    abs_bounds.h = rel_bounds.h.value();
  }
  if rel_bounds.h.is_percent() {
    abs_bounds.h = rel_bounds.h.value() * parent_rect.h;
  }
  // drag and parent relative position
  if draggable && holding {
    // update absolute positioning
    abs_bounds.x += mouse_delta.0;
    abs_bounds.y += mouse_delta.1;
    // update relative positioning
    if rel_bounds.x.is_px() {
      rel_bounds.x += mouse_delta.0;
    }
    if rel_bounds.x.is_percent() {
      rel_bounds.x += mouse_delta.0 / parent_rect.w;
    }
    if rel_bounds.y.is_px() {
      rel_bounds.y += mouse_delta.1;
    }
    if rel_bounds.y.is_percent() {
      rel_bounds.y += mouse_delta.1 / parent_rect.h;
    }
  } else {
    // maintain relative distance to parent
    if rel_bounds.x.is_px() {
      abs_bounds.x = parent_rect.x + rel_bounds.x.value();
      // translate parent resize
      match alignment {
        UiAlign::TopCenter | UiAlign::FullCenter | UiAlign::BottomCenter => {
          abs_bounds.x += parent_delta.0 / 2.0;
          rel_bounds.x += parent_delta.0 / 2.0;
        }
        UiAlign::TopRight | UiAlign::CenterRight | UiAlign::BottomRight => {
          abs_bounds.x += parent_delta.0;
          rel_bounds.x += parent_delta.0;
        }
        _ => ()
      }
    }
    if rel_bounds.x.is_percent() {
      abs_bounds.x = parent_rect.x + (rel_bounds.x.value() * parent_rect.w);
      // translate parent resize
      match alignment {
        UiAlign::TopCenter | UiAlign::FullCenter | UiAlign::BottomCenter => {
          abs_bounds.x += parent_delta.0 / 2.0;
        }
        UiAlign::TopRight | UiAlign::CenterRight | UiAlign::BottomRight => {
          abs_bounds.x += parent_delta.0;
        }
        _ => ()
      }
    }
    if rel_bounds.y.is_px() {
      abs_bounds.y = parent_rect.y + rel_bounds.y.value();
      // translate parent resize
      match alignment {
        UiAlign::CenterLeft | UiAlign::FullCenter | UiAlign::CenterRight => {
          abs_bounds.y += parent_delta.1 / 2.0;
          rel_bounds.y += parent_delta.1 / 2.0;
        }
        UiAlign::BottomLeft | UiAlign::BottomCenter | UiAlign::BottomRight => {
          abs_bounds.y += parent_delta.1;
          rel_bounds.y += parent_delta.1;
        }
        _ => ()
      }
    }
    if rel_bounds.y.is_percent() {
      abs_bounds.y = parent_rect.y + (rel_bounds.y.value() * parent_rect.h);
      // translate parent resize
      match alignment {
        UiAlign::CenterLeft | UiAlign::FullCenter | UiAlign::CenterRight => {
          abs_bounds.y += parent_delta.1 / 2.0;
        }
        UiAlign::BottomLeft | UiAlign::BottomCenter | UiAlign::BottomRight => {
          abs_bounds.y += parent_delta.1;
        }
        _ => ()
      }
    }
  }
  // return new positions
  (abs_bounds, rel_bounds)
}

pub(crate) fn update_children(
  children: &mut Vec<UiElement>, 
  target: &mut Option<UiElement>,
  parent_rect: &Rect,
  parent_delta: &(f32, f32),
  mouse_pos: &(f32, f32),
  mouse_delta: &(f32, f32),
  l_mouse: &UiMouseAction,
  r_mouse: &UiMouseAction,
  time_delta: &f32,
) {
  // update children in reverse order
  for elem in children.iter_mut().rev() {
    match elem {
      UiElement::Box(e) => {
        e.update(target, parent_rect, parent_delta, mouse_pos, mouse_delta, l_mouse, r_mouse, time_delta);
      }
      UiElement::Text(e) => {
        e.update(target, parent_rect, mouse_pos, mouse_delta, l_mouse, r_mouse);
      }
      UiElement::Button(e) => {
        e.update(target, parent_rect, mouse_pos, mouse_delta, l_mouse, r_mouse);
      }
      UiElement::Input(e) => {
        e.update(target, parent_rect, mouse_pos, mouse_delta, l_mouse, r_mouse, time_delta);
      }
    }
  }
}

pub(crate) fn render_children(children: &mut Vec<UiElement>, theme: &UiTheme, parent_color: &Color) {
  for elem in children {
    match elem {
      UiElement::Box(e) => { e.render(&theme); }
      UiElement::Text(e) => { e.render(&theme, parent_color); }
      UiElement::Button(e) => { e.render(&theme); }
      UiElement::Input(e) => { e.render(&theme); }
    }
  }
}

pub(crate) fn update_event(
  action_available: &mut bool,
  inbounds: bool,
  holding: &mut bool,
  prev_event: &UiAction,
  l_mouse: &UiMouseAction,
  r_mouse: &UiMouseAction,
) -> UiAction {
  let mut evt = UiAction::None;
  if *action_available && inbounds {
    *action_available = false;
    match prev_event {
      UiAction::None | UiAction::Hover | UiAction::LRelease => {
        evt = UiAction::Hover;
      }
      _ => ()
    }
    if l_mouse == &UiMouseAction::Down {
      evt = UiAction::LClick;
      *holding = true;
    } else if l_mouse == &UiMouseAction::Hold {
      evt = UiAction::Hold;
    } else if l_mouse == &UiMouseAction::Release {
      if *holding { evt = UiAction::LRelease; }
      *holding = false;
    } else if r_mouse == &UiMouseAction::Down {
      evt = UiAction::RClick;
    } else if r_mouse == &UiMouseAction::Release {
      evt = UiAction::RRelease;
    }
  }
  if !inbounds && l_mouse == &UiMouseAction::Down {
    evt = UiAction::LClickOuter;
  }
  if l_mouse == &UiMouseAction::Release && *holding {
    *holding = false;
  }
  evt
}

pub(crate) fn find_node(children: &Vec<UiElement>, id: u32) -> Option<&UiElement> {
  let mut out = None;
  for elem in children {
    match elem {
      UiElement::Box(e) => {
        if e.id == id { out = Some(elem); }
        let deep = find_node(&e.children, id);
        if deep.is_some() { out = deep; }
      }
      UiElement::Text(e) => {
        if e.id == id { out = Some(elem); }
      }
      UiElement::Button(e) => {
        if e.id == id { out = Some(elem); }
      }
      UiElement::Input(e) => {
        if e.id == id { out = Some(elem); }
      }
    }
  }
  out
}

pub(crate) fn key_code_to_char(k: &KeyCode) -> (&str, &str) {
  match k {
    KeyCode::Space => (" ", " "),
    KeyCode::Apostrophe => ("\'", "\""),
    KeyCode::Comma => (",", "<"),
    KeyCode::Minus => ("-", "_"),
    KeyCode::Period => (".", ">"),
    KeyCode::Slash => ("/", "?"),
    KeyCode::Key0 => ("0", ")"),
    KeyCode::Key1 => ("1", "!"),
    KeyCode::Key2 => ("2", "@"),
    KeyCode::Key3 => ("3", "#"),
    KeyCode::Key4 => ("4", "$"),
    KeyCode::Key5 => ("5", "%"),
    KeyCode::Key6 => ("6", "^"),
    KeyCode::Key7 => ("7", "&"),
    KeyCode::Key8 => ("8", "*"),
    KeyCode::Key9 => ("9", "("),
    KeyCode::Semicolon => (";", ":"),
    KeyCode::Equal => ("=", "+"),
    KeyCode::A => ("a", "A"),
    KeyCode::B => ("b", "B"),
    KeyCode::C => ("c", "C"),
    KeyCode::D => ("d", "D"),
    KeyCode::E => ("e", "E"),
    KeyCode::F => ("f", "F"),
    KeyCode::G => ("g", "G"),
    KeyCode::H => ("h", "H"),
    KeyCode::I => ("i", "I"),
    KeyCode::J => ("j", "J"),
    KeyCode::K => ("k", "K"),
    KeyCode::L => ("l", "L"),
    KeyCode::M => ("m", "M"),
    KeyCode::N => ("n", "N"),
    KeyCode::O => ("o", "O"),
    KeyCode::P => ("p", "P"),
    KeyCode::Q => ("q", "Q"),
    KeyCode::R => ("r", "R"),
    KeyCode::S => ("s", "S"),
    KeyCode::T => ("t", "T"),
    KeyCode::U => ("u", "U"),
    KeyCode::V => ("v", "V"),
    KeyCode::W => ("w", "W"),
    KeyCode::X => ("x", "X"),
    KeyCode::Y => ("y", "Y"),
    KeyCode::Z => ("z", "Z"),
    KeyCode::LeftBracket => ("[", "{"),
    KeyCode::Backslash => ("\\", "|"),
    KeyCode::RightBracket => ("]", "}"),
    KeyCode::GraveAccent => ("`", "~"),
    KeyCode::Kp0 => ("0", "0"),
    KeyCode::Kp1 => ("1", "1"),
    KeyCode::Kp2 => ("2", "2"),
    KeyCode::Kp3 => ("3", "3"),
    KeyCode::Kp4 => ("4", "4"),
    KeyCode::Kp5 => ("5", "5"),
    KeyCode::Kp6 => ("6", "6"),
    KeyCode::Kp7 => ("7", "7"),
    KeyCode::Kp8 => ("8", "8"),
    KeyCode::Kp9 => ("9", "9"),
    KeyCode::KpDecimal => (".", "."),
    KeyCode::KpDivide => ("/", "/"),
    KeyCode::KpMultiply => ("*", "*"),
    KeyCode::KpSubtract => ("-", "-"),
    KeyCode::KpAdd => ("+", "+"),
    _ => ("", "")
  }
}
