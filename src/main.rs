mod parser;

use parser::Rule;
use pest::Parser;

fn main() {
    let result = parser::RivetParser::parse(
        Rule::program,
        r#"
        symbol a
        "#,
    )
    .unwrap()
    .next()
    .unwrap();

    dbg!(result);
}
