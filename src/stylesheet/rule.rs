use std::collections::HashMap;

use super::constants::COLORS;

pub type Specificity = (usize, usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub enum CssRule {
    Normal(NormalRule),
    Comment(String),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NormalRule {
    pub selectors: Vec<Selector>,
    pub declarations: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleSelector {
    pub id: Option<String>,
    pub class: Vec<String>,
    pub tag_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
    StringLiteral(String),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Unit {
    /// Centimeters
    Cm,
    /// Millieters
    Mm,
    /// Inches
    In,
    /// Pixels
    Px,
    /// Points
    Pt,
    /// Picas
    Pc,
    /// Relative to the font-size of the element
    Em,
    /// Relative to the x-height of the current font
    Ex,
    /// Relative to the width of the "0"
    Ch,
    /// Relative to font-size of the root element
    Rem,
    /// Relative to 1% of the width of the viewport*
    Vw,
    /// Relative to 1% of the height of the viewport*
    Vh,
    /// Relative to 1% of viewport's* smaller dimension
    VMin,
    /// Relative to 1% of viewport's* larger dimension
    VMax,
    /// Relative to the parent element
    Percent,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Selector {
    /// Computes the specificity of a CSS selector as defined by the W3C specification.
    ///
    /// Returns the count of ID, class, and tag name selectors in a [`Selector::Simple`].
    /// If the selector is not a [`Selector::Simple`], returns `None`.
    ///
    /// See [W3C Selectors Level 3](https://www.w3.org/TR/selectors/#specificity).
    pub fn specificity(&self) -> Option<Specificity> {
        match self {
            Selector::Simple(simple) => {
                let a = simple.id.iter().count();
                let b = simple.class.len();
                let c = simple.tag_name.iter().count();
                Some((a, b, c))
            }
        }
    }
}

impl Unit {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "cm" => Some(Unit::Cm),
            "mm" => Some(Unit::Mm),
            "in" => Some(Unit::In),
            "px" => Some(Unit::Px),
            "pt" => Some(Unit::Pt),
            "pc" => Some(Unit::Pc),

            "em" => Some(Unit::Em),
            "ex" => Some(Unit::Ex),
            "ch" => Some(Unit::Ch),
            "rem" => Some(Unit::Rem),
            "vw" => Some(Unit::Vw),
            "vh" => Some(Unit::Vh),
            "vmin" => Some(Unit::VMin),
            "vmax" => Some(Unit::VMax),
            "%" => Some(Unit::Percent),

            _ => None,
        }
    }
}

impl Value {
    /// Return the length in px, or zero for non-lengths.
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, _) => f,
            _ => 0.0,
        }
    }
}

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let num = i32::from_str_radix(&hex[0..], 16).unwrap();
        let r = (num >> 16) as u8;
        let g = (num >> 8) as u8;
        let b = num as u8;

        Self {
            r,
            g,
            b,
            a: 255
        }
    }

    pub fn from_keyword(name: &str) -> Self {
        let unlocked = COLORS.lock().unwrap();
        unlocked.get(&name.to_lowercase()).unwrap().clone()
    }
}
