use pest_css_parser::{StyleSheet, Result};

const INPUT: &str = include_str!("font.css");

fn main() -> Result<()> {
    let stylesheet = StyleSheet::parse(INPUT)?;
    println!("{:#?}", stylesheet.0);
    Ok(())
}
