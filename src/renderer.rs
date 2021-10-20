#![allow(clippy::unusual_byte_groupings)]
use indextree::Arena;
use indextree::Node;
use indextree::NodeEdge;
use indextree::NodeId;
use skia_safe::{scalar, Color, PaintStyle};
use std::boxed::Box;
use std::option::Option::Some;
use std::vec::Vec;

/* struct Rect {
  shape: skia_safe::Rect,
  paint: skia_safe::Paint,
}

impl Rect {
  pub fn new(top: scalar, left: scalar, width: scalar, height: scalar) -> Self {
    let mut paint = skia_safe::Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(0xff_ffff00);

    let shape = skia_safe::Rect {
      top: top,
      left: left,
      right: top + width,
      bottom: left + height,
    };

    Self { shape, paint }
  }
} */

/* pub fn render_tree(
  canvas: &mut skia_safe::canvas::Canvas,
  arena: &mut Arena<Rect>,
  node_id: NodeId,
) {
  if let Some(node) = arena.get(node_id) {
    let rect = node.get();
    canvas.draw_rect(rect.shape, &rect.paint);

    if let Some(child_id) = node.first_child() {
      render_tree(canvas, arena, child_id);
      if let Some(child) = arena.get(child_id) {
        while let Some(child) = child.next_sibling() {
          render_tree(canvas, arena, child);
        }
      }
    }
  }
} */

/* struct RenderNode {
  arena: Arena<Rect>;
  node_id: NodeId;
}

impl  RenderNode {
  new(arena: &mut Arena<Rect>, rect: Rect) -> Self {
    let node_id = arena.new_node(rect);
    Self {arena,node_id    }
  }
}

impl Iterator for RenderNode {
  type Item = RenderNode;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(node) = arena.get(self.node_id) {
      if let Some(node_id) = node.first_child() {
        Some(RenderNode {

        })
      }
    } else {
      None
    }
  }
} */

pub fn render_frame(canvas: &mut skia_safe::canvas::Canvas) -> () {
  // let mut arena = &mut Arena::new();

  // let root = arena.new_node(Rect::new(100.0, 100.0, 100.0, 100.0));

  // let child_a = arena.new_node(Rect::new(200.0, 200.0, 100.0, 100.0));
  // let child_b = arena.new_node(Rect::new(300.0, 300.0, 100.0, 100.0));

  // root.append(child_a, &mut arena);
  // root.append(child_b, &mut arena);

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

  // render_tree(canvas, arena, root);
}
