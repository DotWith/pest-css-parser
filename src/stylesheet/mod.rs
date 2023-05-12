use std::collections::HashMap;

use pest::{Parser, iterators::{Pairs, Pair}};
use regex::Regex;

use crate::{grammar::Grammar, Rule, Result};

pub use self::rule::*;

mod rule;
mod constants;

#[derive(Debug)]
pub struct StyleSheet(pub Vec<CssRule>);

impl StyleSheet {
    pub fn parse(input: &str) -> Result<Self> {
        let pairs = Grammar::parse(Rule::stylesheet, input).unwrap_or_else(|e| panic!("{}", e));

        println!("Pairs: {:#?}", pairs);

        let mut rules = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::comment_body => {
                    rules.push(CssRule::Comment(pair.as_str().to_owned()));
                },
                Rule::normal_rule => {
                    match Self::build_normal_rule(pair.into_inner())? {
                        Some(css_rule) => {
                            rules.push(css_rule);
                        },
                        None => println!("Broken rule"),
                    }
                },
                Rule::at_rule => {
                    match Self::build_at_rule(pair.into_inner())? {
                        Some(css_rule) => {
                            rules.push(css_rule);
                        },
                        None => println!("Broken rule"),
                    }
                },
                _ => {}
            }
        }

        Ok(Self(rules))
    }

    fn build_normal_rule(pairs: Pairs<Rule>) -> Result<Option<CssRule>> {
        let mut selectors = Vec::new();
        let mut declarations = HashMap::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::selector => {
                    let mut id = None;
                    let mut class = Vec::new();
                    let mut tag_name = None;
                    
                    let inner_pairs = pair.into_inner().next().unwrap().into_inner();
                    for pair in inner_pairs {
                        match pair.as_rule() {
                            Rule::id_selector => id = Some(pair.as_str().trim_start_matches('#').to_owned()),
                            Rule::class_selector => class.push(pair.as_str().trim_start_matches('.').to_owned()),
                            Rule::type_selector => tag_name = Some(pair.as_str().to_owned()),
                            _ => {}
                        }
                    }

                    selectors.push(Selector::Simple(SimpleSelector {
                        id,
                        class,
                        tag_name,
                    }));
                },
                Rule::declaration => Self::build_declarations(&mut declarations, pair),
                _ => {}
            }
        }

        if !selectors.is_empty() {
            Ok(Some(CssRule::Normal(NormalRule {
                selectors,
                declarations,
            })))
        } else {
            Ok(None)
        }
    }

    fn build_at_rule(pairs: Pairs<Rule>) -> Result<Option<CssRule>> {
        let mut name = String::default();
        let mut rules = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::atr_full => {
                    let inner_pairs = pair.into_inner();
                    for inner_pair in inner_pairs {
                        match inner_pair.as_rule() {
                            Rule::atr_name => name = inner_pair.as_str().strip_prefix('@').unwrap().to_owned(),
                            Rule::declaration => {
                                let mut declarations = HashMap::new();
                                Self::build_declarations(&mut declarations, inner_pair);
                                rules.push(CssRule::Normal(NormalRule {
                                    selectors: vec![],
                                    declarations,
                                }));
                            },
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Some(CssRule::AtRule(AtRule {
            name,
            value: None,
            rules,
        })))
    }

    fn build_declarations(declarations: &mut HashMap<String, Value>, pair: Pair<Rule>) {
        let mut property = String::from("none");
        let mut value = Value::Keyword(String::from("none"));

        let inner_pairs = pair.into_inner();
        for pair in inner_pairs {
            match pair.as_rule() {
                Rule::property => property = pair.as_str().to_owned(),
                Rule::keyword => value = Value::Keyword(pair.as_str().to_owned()),
                Rule::keyword_color => value = Value::Color(Color::from_keyword(pair.as_str())),
                // Rule::length => {
                //     let re = Regex::new(r"\d+").unwrap();

                //     // Content
                //     let content = re.find_iter(pair.as_str())
                //         .map(|m| m.as_str())
                //         .collect::<Vec<&str>>()
                //         .join("");

                //     // Unit
                //     let raw_unit = re.replace_all(pair.as_str(), "").to_string();
                //     let unit = Unit::from_str(&raw_unit).unwrap();

                //     value = Value::Length(content.parse().unwrap(), unit);
                // }
                Rule::color => value = Value::Color(Color::from_hex(pair.as_str())),
                Rule::number => value = Value::Number(pair.as_str().parse().unwrap()),
                Rule::string_literal_body => value = Value::StringLiteral(pair.as_str().to_owned()),
                Rule::fun_body => value = Value::FunCall(FunCall {
                    name: property.clone(),
                    arguments: vec![
                        Value::StringLiteral(pair.as_str().to_owned()),
                    ],
                }),
                _ => {}
            }
        }

        declarations.insert(property, value);
    }
}
