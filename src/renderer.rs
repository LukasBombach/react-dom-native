#![allow(clippy::unusual_byte_groupings)]
use skia_safe::{Color, Paint, PaintStyle, Rect};

pub fn render_frame(canvas: &mut skia_safe::canvas::Canvas) -> () {
  let mut paint = Paint::default();
  paint.set_anti_alias(true);
  paint.set_style(PaintStyle::Fill);
  paint.set_color(0xff_00ff00);

  canvas.clear(Color::BLACK);

  canvas.draw_rect(
    Rect {
      left: 0.0,
      top: 0.0,
      right: 100.0,
      bottom: 100.0,
    },
    &paint,
  );
}
