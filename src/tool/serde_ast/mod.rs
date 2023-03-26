// 从 json 中序列化语法树

use std::rc::Rc;
use serde_json;
use std::io::Read;
use crate::runtime::{ast::{rule_context::RuleContext, terminal_context::TerminalContext, ast_context::ASTContext}, token};

/**
 * { rule_index: 0, rule_name: "expr", children: [{ token_type: 1, token_name: "xxx" }] }
 */

#[derive(Debug)]
pub struct Error;

// 至少包含一个非终结符的语法树
pub fn from_str(input: &str) -> Result<Rc<RuleContext>, Error> {
  let data: serde_json::Value = match serde_json::from_str(input) {
    Ok(data) => data, 
    Err(_) => return  Err(Error {})
  };

  parse_rule(&data)
}

fn parse_rule(value: &serde_json::Value) -> Result<Rc<RuleContext>, Error> {
  let children = &value["children"];
  if ! children.is_array() { return Err(Error {}); }
  let children = children.as_array().unwrap();

  let rule_index = value["rule_index"].as_u64();
  if rule_index == None { return Err(Error {}); }
  let rule_index = rule_index.unwrap() as usize;

  let rule_name = value["rule_name"].as_str();
  if rule_name == None { return Err(Error {}); }
  let rule_name = rule_name.unwrap().to_owned();

  let mut result: Vec<ASTContext> = Vec::new();
  for child in children.iter() {
    if child["children"].is_null() {
      let child = parse_terminal(child)?;
      result.push(ASTContext::Ternimal(child));
    }
    else {
      let child = parse_rule(child)?;
      result.push(ASTContext::Rule(child));
    }
  }

  Ok(Rc::new(RuleContext { rule_name, rule_index, children: result }))
// 
  // todo!()
}

fn parse_terminal(value: &serde_json::Value) -> Result<Rc<TerminalContext>, Error> {
  let token_type = value["token_type"].as_u64();
  let token_name = value["token_name"].as_str();

  if token_type == None || token_name == None { return Err(Error {}); }
  let token_type = token_type.unwrap() as usize;
  let token_name = token_name.unwrap();

  Ok(Rc::new(TerminalContext {
    symbol: token::Token::new(token_type, token_name)
  }))
  
  
}

pub fn from_reader(reader: impl Read) -> Result<Rc<RuleContext>, Error> {
  let data: serde_json::Value = match serde_json::from_reader(reader) {
    Ok(data) => data, 
    Err(_) => return  Err(Error {})
  };

  parse_rule(&data)
}


