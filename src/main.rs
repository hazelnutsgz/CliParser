use std::env;
use CliParse::{CliParser, Matches, RuleType};
fn main() {
    let mut cli_parser = CliParser::new();
    cli_parser.add_rule_with_default("a", RuleType::Maybe, "asdf");
    cli_parser.add_rule("b");
    cli_parser.add_rule("c");
    cli_parser.add_rule("d");
    cli_parser.add_rule("h");
    let arg_list: Vec<String> = env::args().collect();
    let matches: Matches = cli_parser.parse(&arg_list).unwrap();

    matches.get_opt("a").unwrap();
}
