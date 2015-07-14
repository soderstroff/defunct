// Directives
#![allow(unused_must_use)]

// Crates
#[macro_use]
extern crate lazy_static;

// Imports
use std::slice::Iter;
use std::iter::Peekable;
use std::io::Write;


// Lisp data-types and evaluation environment
mod data;
use data::*;

fn tokenize(chars:&str) -> Vec<String>{
    /* Takes a string of chars and returns a vector of tokens. */

    chars.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .filter(|s| !(s.is_empty()))
        .map(|s| s.to_string())
        .collect()
}

fn read_from_tokens(tokens: &mut Peekable<Iter<String>>) -> Data {
    /* Reads an expression from a sequence of tokens. " */

    if tokens.len() == 0 {
        panic!("Unexpected EOF while parsing.");
    }

    let token = tokens.next().unwrap();

    if token == "(" {
        let mut l = Nil;
        while *tokens.peek().expect("Syntax error! Expected ')'") != ")" {
            l = read_from_tokens(tokens).cons(l);
        }
        tokens.next();
        return l.nreverse();
    }

    match token.parse::<f32>() {
        Ok(f) => return Float(f),
        _ => {;}
    }

    return Symbol(token.to_string())
}

fn parse(program: &str) -> Data {
    return read_from_tokens(&mut tokenize(program).iter().peekable());
}


fn repl(env: &mut Env) {
    let mut input = String::new();
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        print!("defunct> ");
        stdout.flush();
        stdin.read_line(&mut input);
        println!("{}\n", parse(&input).eval(env));
        input.clear();
    }
}


fn main() {
    let env = &mut Env::new_root();
    repl(env);
}
