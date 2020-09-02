use pest::{Parser, RuleType};
use parser::ekans::*;
use parser::ekans::EkansParser;
use pest::iterators::{Pair, Pairs};
use std::result::{Result, Iter};
use pest::error::Error;
use std::borrow::Borrow;

fn print(pairs: Pairs<Rule>, depth: usize) {

    for x in pairs.clone() {
        println!("{}Token= {:?} {:?}","\t".repeat(depth),x.as_rule(),x.as_span());
        print(x.into_inner(),depth+1);

    }
}


#[test]
pub fn read_file() {
    let file = std::fs::read_to_string("tests/res/main.ek").expect("Could Not Read file");
    let parse: Result<Pairs<Rule>, Error<Rule>> = EkansParser::parse(Rule::file, file.as_str());
    
    

    print(parse.clone().expect("msg"),0);
}
