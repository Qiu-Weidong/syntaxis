use std::fs::File;
use std::rc::Rc;

use syntaxis::tool::grammar::Grammar;
use syntaxis::tool::serde_ast;
use syntaxis::tool::syntaxis::syntaxis_context::RuleListContext;
// use syntaxis::tool::grammar;
use syntaxis::tool::visitor::grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor};

/**
rule_list -> 10;
10 ->;
10 -> 9 10;
9 -> lexer_rule;
9 -> parser_rule;
parser_rule -> RULE_REF COLON block SEMI;
lexer_rule -> TOKEN_REF COLON regular SEMI;
block -> alternative 12;
12 -> 11 12;
12 ->;
11 -> OR alternative;
alternative -> epsilon;
alternative -> 13;
epsilon -> EPSILON;
element -> 14 15;

14 -> LPAREN block RPAREN;
14 -> RULE_REF;
14 -> TOKEN_REF;
14 -> STRING_LITERAL;

regular -> REGULAR_LITERAL;

16 -> PLUS;
16 -> STAR;
16 -> QUESTION;
17 -> QUESTION;
17 ->;
ebnf_suffix -> 16 17;

15 -> ebnf_suffix;
15 ->;

13 -> element;
13 -> element 13;

 */

/**

rule_list:
{_STOP, }

alternative:
{OR, SEMI, RPAREN, }

parser_rule:
{_STOP, TOKEN_REF, RULE_REF, }

block:
{SEMI, RPAREN, }

epsilon:
{SEMI, RPAREN, OR, }

element:
{RPAREN, SEMI, LPAREN, OR, STRING_LITERAL, TOKEN_REF, RULE_REF, }

regular:
{SEMI, }

ebnf_suffix:
{TOKEN_REF, RULE_REF, LPAREN, STRING_LITERAL, OR, RPAREN, SEMI, }

lexer_rule:
{_STOP, TOKEN_REF, RULE_REF, }

16:
{RPAREN, SEMI, QUESTION, RULE_REF, OR, TOKEN_REF, LPAREN, STRING_LITERAL, }

9:
{TOKEN_REF, _STOP, RULE_REF, }

14:
{STRING_LITERAL, STAR, RULE_REF, RPAREN, SEMI, OR, QUESTION, LPAREN, PLUS, TOKEN_REF, }

12:
{SEMI, RPAREN, }
11:
{RPAREN, OR, SEMI, }
17:
{TOKEN_REF, RPAREN, SEMI, STRING_LITERAL, OR, RULE_REF, LPAREN, }
15:
{OR, RPAREN, SEMI, LPAREN, TOKEN_REF, RULE_REF, STRING_LITERAL, }
10:
{_STOP, }
13:
{SEMI, RPAREN, OR, }


 */

#[test]
fn follow_set_test() {
  // 测试求 first 集合


  let file = File::open("src/tool/syntaxis/syntaxis.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap() as Rc<dyn RuleListContext>;

  let mut grammar = Grammar::new("我的文法");
  let token_cnt;
  {
    let mut visitor = StringLiteralToTokenVisitor::new(
      &mut grammar, 2
    );

    ast.accept(&mut visitor);
    token_cnt = visitor.next_token_id;
  }
  
  let rule_cnt; {
    let mut visitor = SymbolVisitor::new(&mut grammar, token_cnt, 0);
    ast.accept(&mut visitor);
    rule_cnt = visitor.next_rule_id;
  }

  {
    let mut visitor = ProductionVisitor::new(&mut grammar, rule_cnt);
    ast.accept(&mut visitor);
  }


  println!("{}", grammar);

  let (first, _) = grammar.first_set();
  grammar.terminals.insert(1, "_STOP".to_owned());

  let follow = grammar.follow_set(&first);

  // for (id, collection) in first.iter() {
  //   let name = grammar.nonterminals.get(id).unwrap();
  //   let name = match name {
  //     Some(name) => name.clone(),
  //     None => id.to_string(),
  //   };
  //   println!("{}:", name);
  //   print!("{{");
  //   for item in collection.set.iter() {
  //     let name = grammar.terminals.get(item).unwrap();
  //     print!("{}, ", name);
  //   }
  //   if collection.allow_epsilon { print!("ε") }
  //   println!("}}");
  // }
  
  for (id, collection) in follow.iter() {
    let name = grammar.nonterminals.get(id).unwrap();
    let name = match name {
      Some(name) => name.clone(),
      None => id.to_string(),
    };
    println!("{}:", name);
    print!("{{");
    for item in collection.iter() {
      let name = grammar.terminals.get(item).unwrap();
      print!("{}, ", name);
    }
    // if collection.allow_epsilon { print!("ε") }
    println!("}}");
  }


}

