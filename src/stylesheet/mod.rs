use pest::{Parser, iterators::{Pairs, Pair}};

use crate::{grammar::Grammar, Rule, Result};

use self::constants::COLORS;
pub use self::rule::*;

mod rule;
mod constants;

#[derive(Debug, Default)]
pub struct StyleSheet {
    pub rules: Vec<CssRule>,
    pub errors: Vec<String>,
}

impl StyleSheet {
    pub fn parse(input: &str) -> Result<Self> {
        let pairs = Grammar::parse(Rule::css, input).unwrap_or_else(|e| panic!("{}", e));

        Self::build_stylesheet(pairs)
    }

    fn build_stylesheet(pairs: Pairs<Rule>) -> Result<Self> {
        let mut stylesheet = StyleSheet::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::rule_comment => {
                    stylesheet
                        .rules.push(CssRule::Comment(pair.into_inner().as_str().to_string()));
                },
                Rule::rule_normal => match Self::build_normal_rule(pair) {
                    Ok(rule) => {
                        let normal_rule = CssRule::Normal(rule);
                        stylesheet.rules.push(normal_rule);
                    }
                    Err(error) => {
                        stylesheet.errors.push(error.to_string());
                    }
                }
                _ => {}
            }
        }

        Ok(stylesheet)
    }

    fn build_normal_rule(pair: Pair<Rule>) -> Result<NormalRule> {
        let mut normal_rule = NormalRule::default();
        
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::sel_normal => {
                    normal_rule.selectors.push(Self::build_simple_selector(pair.into_inner())?);
                }
                Rule::declaration => {
                    let del = Self::build_declaration(pair.into_inner())?;
                    normal_rule.declarations.insert(del.0, del.1);
                }
                _ => {}
            }
        }

        Ok(normal_rule)
    }

    fn build_simple_selector(pairs: Pairs<Rule>) -> Result<Selector> {
        let mut selector = SimpleSelector::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::sel_id_body => selector.id = Some(pair.as_str().to_string()),
                Rule::sel_class_body => selector.class.push(pair.as_str().to_string()),
                Rule::sel_type => selector.tag_name = Some(pair.as_str().to_string()),
                _ => {}
            }
        }

        Ok(Selector::Simple(selector))
    }

    fn build_declaration(pairs: Pairs<Rule>) -> Result<(String, Value)> {
        let mut declaration = (String::default(), Value::StringLiteral(String::default()));

        for pair in pairs {
            match pair.as_rule() {
                Rule::del_property => declaration.0 = pair.as_str().to_string(),
                Rule::del_val_keyword => {
                    let colors = COLORS.lock().unwrap();
                    if let Some(color) = colors.get(pair.as_str()) {
                        declaration.1 = Value::Color(*color)
                    } else {
                        declaration.1 = Value::Keyword(pair.as_str().to_string())
                    }
                },
                Rule::del_val_length => {
                    let mut inner_pairs = pair.into_inner();
                    let len_value = inner_pairs.next().unwrap();
                    let len_type = inner_pairs.next().unwrap();

                    declaration.1 = Value::Length(
                        len_value.as_str().parse().unwrap(),
                        Unit::from_str(len_type.as_str()).unwrap()
                    );
                }
                Rule::del_val_color => declaration.1 = Value::Color(Color::from_hex(pair.as_str())),
                _ => {}
            }
        }

        Ok(declaration)
    }
}
