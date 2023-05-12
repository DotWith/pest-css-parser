use std::fs;

use pest_css_parser::{StyleSheet, Result};

fn main() -> Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    let input = fs::read_to_string(args.next().expect("No CSS file provided")).unwrap();

    let stylesheet = StyleSheet::parse(&input)?;
    assert_eq!(stylesheet.rules.len(), 8);
    
    Ok(())
}
