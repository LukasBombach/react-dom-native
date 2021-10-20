#![allow(clippy::unusual_byte_groupings)]
use glutin::dpi::PhysicalSize;
use skia_safe::{Color, PaintStyle};
use stretch::geometry::Size;
use stretch::number::Number;
use stretch::style::*;
use stretch::Error;

pub fn render(
  canvas: &mut skia_safe::canvas::Canvas,
  size: PhysicalSize<u32>,
) -> Result<(), Error> {
  let mut stretch = stretch::node::Stretch::new();

  let child = stretch.new_node(
    Style {
      size: Size {
        width: Dimension::Percent(0.5),
        height: Dimension::Percent(0.5),
      },
      ..Default::default()
    },
    vec![],
  )?;

  let body = stretch.new_node(
    Style {
      size: Size {
        width: Dimension::Percent(100.0),
        height: Dimension::Percent(100.0),
      },
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..Default::default()
    },
    vec![child],
  )?;

  stretch.compute_layout(
    body,
    Size {
      width: Number::Defined(size.width as f32),
      height: Number::Defined(size.height as f32),
    },
  )?;

  let child_layout = stretch.layout(child)?;

  let mut paint = skia_safe::Paint::default();
  paint.set_anti_alias(true);
  paint.set_style(PaintStyle::Fill);
  paint.set_color(0xff_ffff00);

  let top = child_layout.location.y / 100.0;
  let left = child_layout.location.x / 100.0;
  let bottom = top + (child_layout.size.height / 100.0);
  let right = left + (child_layout.size.width / 100.0);

  let shape = skia_safe::Rect {
    top,
    left,
    bottom,
    right,
  };

  canvas.clear(Color::BLACK);
  canvas.draw_rect(shape, &paint);

  Ok(())
}
