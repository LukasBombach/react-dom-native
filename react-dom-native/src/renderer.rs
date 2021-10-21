use glutin::dpi::PhysicalSize;
use skia_safe::canvas::Canvas;
use skia_safe::Color;
use skia_safe::Paint;
use skia_safe::PaintStyle;

use yoga::prelude::*;
use yoga::Align;
use yoga::Justify;
use yoga::Node;

pub fn render(canvas: &mut Canvas, size: PhysicalSize<u32>) -> Result<(), ()> {
  let mut body = Node::new();
  style!(body,
    Width(100 %),
    Height(100 %),
    JustifyContent(Justify::Center),
    AlignItems(Align::Center)
  );

  let mut child = Node::new();
  style!(child,
    Width(50 %),
    Height(50 %)
  );

  body.insert_child(&mut child, 0);
  body.calculate_layout(size.width as f32, size.height as f32, yoga::Direction::LTR);
  let child_layout = child.get_layout();

  let mut paint = Paint::default();
  paint.set_anti_alias(true);
  paint.set_style(PaintStyle::Fill);
  paint.set_color(0xff_ffff00);

  let top = child_layout.top();
  let left = child_layout.left();
  let bottom = top + child_layout.height();
  let right = left + child_layout.width();

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
