use kdlite::dom::{Node, Value};

#[inline]
pub fn int(node: &Node<'_>) -> Option<i128> {
    let entry = node.entries.first()?;

    match entry.value {
        Value::Integer(v) => Some(v),
        _ => None,
    }
}

#[inline]
pub fn float(node: &Node<'_>) -> Option<f64> {
    let entry = node.entries.first()?;

    match entry.value {
        Value::Float(v) => Some(v),
        _ => None,
    }
}

#[inline]
pub fn str<'a>(node: &'a Node<'_>) -> Option<&'a str> {
    let entry = node.entries.first()?;

    match entry.value {
        Value::String(std::borrow::Cow::Borrowed(v)) => Some(v),
        _ => None,
    }
}
