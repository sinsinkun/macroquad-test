#![allow(unused)]

use macroquad::prelude::*;

pub fn point_in_rect(point: &(f32, f32), rect: &Rect) -> bool {
  let x_in = 
    if point.0 > rect.x && point.0 < rect.x + rect.w { true }
    else { false };
  let y_in =
    if point.1 > rect.y && point.1 < rect.y + rect.h { true }
    else { false };
  x_in && y_in
}