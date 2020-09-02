mod precedence;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use(lazy_static)]
extern crate lazy_static;


pub mod ekans {

    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "grammer.pest"]
    pub struct EkansParser;
}
