use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct RivetParser;

#[cfg(test)]
mod tests {
    use super::{RivetParser, Rule};
    use pest::Parser;

    #[test]
    fn valid_identifier() {
        let val = RivetParser::parse(Rule::identifier, "hello_world123_asd");

        assert!(val.is_ok());
    }

    #[test]
    #[should_panic]
    fn invalid_identifier() {
        let val = RivetParser::parse(Rule::identifier, "_hello_world123_asd");

        assert!(val.is_ok())
    }
}
