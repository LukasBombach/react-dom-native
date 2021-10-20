#![allow(clippy::unusual_byte_groupings)]
use indextree::Arena;
use indextree::Node;
use indextree::NodeEdge;
use indextree::NodeId;
use skia_safe::{scalar, Color, PaintStyle};
use std::boxed::Box;
use std::option::Option::Some;
use std::vec::Vec;

pub fn render_frame(canvas: &mut skia_safe::canvas::Canvas) -> () {
  canvas.clear(Color::BLACK);

  let mut paint = skia_safe::Paint::default();
  paint.set_anti_alias(true);
  paint.set_style(PaintStyle::Fill);
  paint.set_color(0xff_ffff00);

  let shape = skia_safe::Rect {
    top: 50.0,
    left: 50.0,
    right: 100.0,
    bottom: 100.0,
  };

  canvas.draw_rect(shape, &paint);
}
