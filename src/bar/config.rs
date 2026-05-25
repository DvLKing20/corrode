ususe gtk4_layer_shell::{Edge, Layer};
use kdlite::dom::Node;
use std::iter::Peekable;
use std::slice::Iter;

pub struct BarConfig {
    pub width: i64,
    pub height: i64,
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

        while let Some(node) = stream.peek() {
            match node.name() {
                "Position" => {
                    let mut pos = "top";
                    if let Some(entry) = node.entries.first() {
                        if let kdlite::dom::Value::String(std::borrow::Cow::Borrowed(raw_str)) =
                            entry.value
                        {
                            pos = raw_str
                        }
                    }
                    match pos {
                        "top" => config.anchors = [Edge::Top, Edge::Left, Edge::Right],
                        "bottom" => config.anchors = [Edge::Bottom, Edge::Left, Edge::Right],
                        "left" => config.anchors = [Edge::Left, Edge::Top, Edge::Bottom],
                        "right" => config.anchors = [Edge::Right, Edge::Top, Edge::Bottom],
                        _ => {}
                    }
                    stream.next();
                }

                "Opacity" => {
                    if let Some(entry) = node.entries.first() {
                        if let kdlite::dom::Value::Float(v) = entry.value {
                            config.opacity = v
                        }
                    }
                    stream.next();
                }

                "Exclusive" => {
                    if let Some(entry) = node.entries.first() {
                        if let kdlite::dom::Value::Integer(v) = entry.value {
                            config.exclusive_zone = v as i32;
                        }
                    }
                    stream.next();
                }

                "Layer" => {
                    let mut pos = "top";
                    if let Some(entry) = node.entries.first() && 
                        let kdlite::dom::Value::String(std::borrow::Cow::Borrowed(raw_str)) = entry.value
                        {
                            pos = raw_str
                        }

                    match pos {
                        "top" => config.layer = Layer::Top,
                        "bottom" => config.layer = Layer::Bottom,
                        _ => {}
                    }

                    stream.next();
                }

                "Width" => {
                    if let Some(entry) = node.entries.first()
                        && let kdlite::dom::Value::Integer(v) = entry.value
                    {
                        config.width = v as i64;
                    }
                }

                _ => break,
            }
        }
        config
    }
}e gtk4_layer_shell::{Edge, Layer};
use kdlite::dom::Node;
use std::iter::Peekable;
use std::slice::Iter;

pub struct BarConfig {
    pub width: i64,
    pub height: i64,
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

        while let Some(node) = stream.peek() {
            match node.name() {
                "Position" => {
                    let mut pos = "top";
                    if let Some(entry) = node.entries.first() {
                        if let kdlite::dom::Value::String(std::borrow::Cow::Borrowed(raw_str)) =
                            entry.value
                        {
                            pos = raw_str
                        }
                    }
                    match pos {
                        "top" => config.anchors = [Edge::Top, Edge::Left, Edge::Right],
                        "bottom" => config.anchors = [Edge::Bottom, Edge::Left, Edge::Right],
                        "left" => config.anchors = [Edge::Left, Edge::Top, Edge::Bottom],
                        "right" => config.anchors = [Edge::Right, Edge::Top, Edge::Bottom],
                        _ => {}
                    }
                    stream.next();
                }

                "Opacity" => {
                    if let Some(entry) = node.entries.first() {
                        if let kdlite::dom::Value::Float(v) = entry.value {
                            config.opacity = v
                        }
                    }
                    stream.next();
                }

                "Exclusive" => {
                    if let Some(entry) = node.entries.first() {
                        if let kdlite::dom::Value::Integer(v) = entry.value {
                            config.exclusive_zone = v as i32;
                        }
                    }
                    stream.next();
                }

                "Layer" => {
                    let mut pos = "top";
                    if let Some(entry) = node.entries.first() && 
                        let kdlite::dom::Value::String(std::borrow::Cow::Borrowed(raw_str)) = entry.value
                        {
                            pos = raw_str
                        }

                    match pos {
                        "top" => config.layer = Layer::Top,
                        "bottom" => config.layer = Layer::Bottom,
                        _ => {}
                    }

                    stream.next();
                }

                "Width" => {
                    if let Some(entry) = node.entries.first()
                        && let kdlite::dom::Value::Integer(v) = entry.value
                    {
                        config.width = v as i64;
                    }
                }

                _ => break,
            }
        }
        config
    }
}
