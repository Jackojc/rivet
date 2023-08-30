mod parser;

use parser::Rule;
use pest::Parser;

const SRC: &'static str = r#"
def a "hello, world"
def a 123
def a hello_world123asd
def a < a = b > 
def a ( a )
def a [ a ]
def a 1
symbol test
"#;

fn main() {
    let result = parser::RivetParser::parse(Rule::program, SRC);

    dbg!(result);
}
