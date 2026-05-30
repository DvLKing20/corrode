use crate::config::helpers::{float, int, str};
use gtk4_layer_shell::{Edge, Layer};
use kdlite::dom::Node;
use std::iter::Peekable;
use std::slice::Iter;

pub struct BarConfig {
    pub width: i32,
    pub height: i32,
    pub exclusive_zone: i32,
    pub opacity: f64,
    pub anchors: [Edge; 3],
    pub layer: Layer,
}

impl Default for BarConfig {
    fn default() -> Self {
        let anchors = [Edge::Top, Edge::Left, Edge::Right];
        Self {
            width: -1,
            height: 30,
            exclusive_zone: 30,
            opacity: 0.85,
            anchors,
            layer: Layer::Top,
        }
    }
}

impl BarConfig {
    pub fn new(stream: &mut Peekable<Iter<'_, Node<'_>>>) -> Self {
        let mut config = Self::default();
        Self::parse(stream, &mut config);
        config
    }

    pub fn parse(stream: &mut Peekable<Iter<'_, Node<'_>>>, config: &mut BarConfig) {
        while let Some(node) = stream.next_if(|n| is_bar_property(n.name())) {
            match node.name() {
                "Position" => {
                    let Some(pos) = str(node) else { continue };
                    match pos {
                        "top" => config.anchors = [Edge::Top, Edge::Left, Edge::Right],
                        "bottom" => config.anchors = [Edge::Bottom, Edge::Left, Edge::Right],
                        "left" => config.anchors = [Edge::Left, Edge::Top, Edge::Bottom],
                        "right" => config.anchors = [Edge::Right, Edge::Top, Edge::Bottom],
                        _ => {}
                    }
                }

                "Opacity" => {
                    let Some(v) = float(node) else { continue };
                    config.opacity = v
                }

                "Exclusive" => {
                    let Some(v) = int(node) else { continue };
                    config.exclusive_zone = v as i32
                }

                "Layer" => {
                    let Some(pos) = str(node) else { continue };
                    match pos {
                        "top" => config.layer = Layer::Top,
                        "bottom" => config.layer = Layer::Bottom,
                        _ => {}
                    }
                }

                "Width" => {
                    let Some(v) = int(node) else { continue };
                    config.width = v as i32
                }

                "Height" => {
                    let Some(v) = int(node) else { continue };
                    config.height = v as i32
                }

                _ => break,
            }
        }
    }
}

#[inline]
fn is_bar_property(name: &str) -> bool {
    matches!(
        name,
        "Position" | "Opacity" | "Exclusive" | "Layer" | "Width" | "Height"
    )
}
