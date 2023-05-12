use std::{sync::Mutex, collections::HashMap};

use once_cell::sync::Lazy;

use crate::Color;

pub static COLORS: Lazy<Mutex<HashMap<String, Color>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Basic colors
    m.insert(String::from("black"), Color::from_hex("#000000"));
    m.insert(String::from("silver"), Color::from_hex("#c0c0c0"));
    m.insert(String::from("gray"), Color::from_hex("#808080"));
    m.insert(String::from("white"), Color::from_hex("#ffffff"));
    m.insert(String::from("maroon"), Color::from_hex("#800000"));
    m.insert(String::from("red"), Color::from_hex("#ff0000"));
    m.insert(String::from("purple"), Color::from_hex("#800080"));
    m.insert(String::from("fuchsia"), Color::from_hex("#ff00ff"));
    m.insert(String::from("green"), Color::from_hex("#008000"));
    m.insert(String::from("lime"), Color::from_hex("#00ff00"));
    m.insert(String::from("olive"), Color::from_hex("#808000"));
    m.insert(String::from("yellow"), Color::from_hex("#ffff00"));
    m.insert(String::from("navy"), Color::from_hex("#000080"));
    m.insert(String::from("blue"), Color::from_hex("#0000ff"));
    m.insert(String::from("teal"), Color::from_hex("#008080"));
    m.insert(String::from("aqua"), Color::from_hex("#00ffff"));

    // Extended colors
    m.insert(String::from("aliceblue"), Color::from_hex("#f0f8ff"));

    Mutex::new(m)
});
