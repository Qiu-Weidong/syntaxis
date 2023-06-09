use std::{collections::HashMap, rc::Rc};

use crate::{runtime::{token::Token, ast::{rule_context::RuleContext, terminal_context::TerminalContext, ast_context::ASTContext}, vocabulary::Vocabulary}, tool::{grammar::{Production, ProductionItem}, syntaxis::syntaxis_context::{RuleListContext, ParserRuleContext, BlockContext, AlternativeContext, EpsilonContext, ElementContext, EbnfSuffixContext, LexerRuleContext, RegularContext}}};



pub struct SyntaxisParser {
  pub tokens: Vec<Token>,

  // 这两个应该声明为常量
  pub table: HashMap<(usize, usize), Rc<Production>>,
  pub vocabulary: Vocabulary,
}


impl SyntaxisParser {

  // 使用模板生成
  pub const RULE_LIST: usize = 0;
  pub const PARSER_RULE: usize = 1;
  pub const BLOCK: usize = 2;
  pub const ALTERNATIVE: usize = 3;
  pub const EPSILON: usize = 4;
  pub const ELEMENT: usize = 5;
  pub const EBNF_SUFFIX: usize = 6;
  pub const LEXER_RULE: usize = 7;
  pub const REGULAR: usize = 8;

  // 生成一个预测分析表



  pub fn new(tokens: Vec<Token>, table: HashMap<(usize, usize), Rc<Production>>, vocabulary: Vocabulary) -> Self {

    Self {
      tokens,
      table,
      vocabulary,
    }
  }


  // 使用模板生成
  pub fn rule_list(&self) -> Box<dyn RuleListContext> {
    let (result, _) = self.parse_ast(0, Self::RULE_LIST);
    Box::new(result)
  }

  pub fn parser_rule(&self) -> Box<dyn ParserRuleContext> {
    let (result, _) = self.parse_ast(0, Self::PARSER_RULE);
    Box::new(result)
  }

  pub fn block(&self) -> Box<dyn BlockContext> {
    let (result, _) = self.parse_ast(0, Self::BLOCK);
    Box::new(result)
  }

  pub fn alternative(&self) -> Box<dyn AlternativeContext> {
    let (result, _) = self.parse_ast(0, Self::ALTERNATIVE);
    Box::new(result)
  }

  pub fn epsilon(&self) -> Box<dyn EpsilonContext> {
    let (result, _) = self.parse_ast(0, Self::EPSILON);
    Box::new(result)
  }

  pub fn element(&self) -> Box<dyn ElementContext> {
    let (result, _) = self.parse_ast(0, Self::ELEMENT);
    Box::new(result)
  }

  pub fn ebnf_suffix(&self) -> Box<dyn EbnfSuffixContext> {
    let (result, _) = self.parse_ast(0, Self::EBNF_SUFFIX);
    Box::new(result)
  }

  pub fn lexer_rule(&self) -> Box<dyn LexerRuleContext> {
    let (result, _) = self.parse_ast(0, Self::LEXER_RULE);
    Box::new(result)
  }

  pub fn regular(&self) -> Box<dyn RegularContext> {
    let (result, _) = self.parse_ast(0, Self::REGULAR);
    Box::new(result)
  }

  // 可有可无
  pub fn parse(&self) -> RuleContext {
    self.parse_ast(0, Self::RULE_LIST).0
  }


  // 直接照抄
  fn parse_ast(&self, cursor: usize, rule_index: usize) -> (RuleContext, usize) {
    let mut cursor = cursor;

    let token_type = self.tokens[cursor].token_type;
    let production = self.table.get(&(rule_index, token_type)).unwrap();
    let name = self.vocabulary.get_nonterminal_name_with_default(rule_index);

    let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };
    
    for child in production.right.iter() {
      match child {
        ProductionItem::NonTerminal(rule_id) => {
          let (rule, new_cursor) = self.parse_ast(cursor, *rule_id);
          cursor = new_cursor;
          if let Some(_) = self.vocabulary.get_nonterminal_name_by_id(*rule_id) {
            let child = ASTContext::Rule(rule);
            result.children.push(child);
          } else {
            for child in rule.children.iter() {
              result.children.push(child.clone());
            }
          }
          
        },
        ProductionItem::Terminal(token_type) => {
          // 检查是否匹配
          if *token_type != self.tokens[cursor].token_type { println!("符号不匹配") }
          let terminal = TerminalContext { symbol: self.tokens[cursor].clone() };
          cursor += 1;
          let child = ASTContext::Terminal(terminal);
          result.children.push(child);
        },
      };
    }
    (result, cursor)
  }

}




