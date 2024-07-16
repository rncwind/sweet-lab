use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "json.pest"]
pub struct JSONParser;

// This defines our JSON AST.
// Each JSON datatype is a enum variant.
#[derive(Debug)]
enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn serialize(val: &JSONValue) -> String {
    match val {
        JSONValue::Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(n, v)| format!("\"{}\":{}", n, serialize(v)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        JSONValue::Array(a) => {
            let cont: Vec<String> = a.iter().map(serialize).collect();
            format!("[{}]", cont.join(","))
        }
        JSONValue::String(s) => format!("\"{}\"", s),
        JSONValue::Number(n) => format!("{}", n),
        JSONValue::Boolean(b) => format!("{}", b),
        JSONValue::Null => "null".to_string(),
    }
}

fn main() {
    let unparsed_file = std::fs::read_to_string("test.json").expect("Cannot read file");
    let json = parse_json_file(&unparsed_file).expect("Couldn't parse!");
    println!("{:?}\n", json);
    println!("{}\n", serialize(&json));
}

fn parse_json_file(file: &str) -> Result<JSONValue, pest::error::Error<Rule>> {
    // Parse the json
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();
    // Now, we probably want ot make an AST out of the damn thing.
    Ok(parse_json_value(json))
}

use pest::iterators::Pair;
fn parse_json_value(pair: Pair<Rule>) -> JSONValue {
    match pair.as_rule() {
        Rule::object => JSONValue::Object(
            pair.into_inner()
                .map(|pair| {
                    let mut inner_rules = pair.into_inner();
                    let name = inner_rules
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str();
                    let value = parse_json_value(inner_rules.next().unwrap());
                    (name, value)
                })
                .collect(),
        ),
        // We need to recurse on this one.
        Rule::array => JSONValue::Array(pair.into_inner().map(parse_json_value).collect()),
        // Get the text without "" for strings and turn into a rust string.
        Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
        // Can parse these directly so just extract them and rust-intern them
        Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
        Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
        Rule::null => JSONValue::Null,
        // All of these are not _real_.
        Rule::value
        | Rule::inner
        | Rule::pair
        | Rule::json
        | Rule::EOI
        | Rule::WHITESPACE
        | Rule::char => {
            unreachable!()
        }
    }
}
