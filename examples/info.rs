use pest_css_parser::{StyleSheet, Result};

const INPUT: &str = include_str!("info.css");

fn main() -> Result<()> {
    let stylesheet = StyleSheet::parse(INPUT)?;
    assert_eq!(stylesheet.0.len(), 8);
    Ok(())
}
