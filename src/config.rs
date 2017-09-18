use regex::Regex;

pub static TVDB_API_KEY: &'static str = "0629B785CE550C8D";

struct ParsedConfig{
    patterns: Vec<Regex>,
}
