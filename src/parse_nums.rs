use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_nums<'a, T>(string: &'a str) -> impl Iterator<Item=T> + 'a where T: 'a, T: FromStr {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").expect("Invalid Regex");
    }

    RE.find_iter(string)
        .flat_map(|m| m.as_str().parse().ok())
}
