#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]

/* Imports */
use std::slice::Iter;
use std::iter::Peekable;

/* Lisp evaluation data */
mod data;
use data::Env;
use data::Data;
use data::Data::{Nil, Symbol, Float};
use data::Function;

fn tokenize(chars:&str) -> Vec<String>{
    /* Takes a string of chars and returns a vector of tokens. */

    chars.replace("(", " ( ")
        .replace(")", " ) ")
        .split(' ')
        .filter(|s| !(s.is_empty()))
        .map(|s| s.to_string())
        .collect()
}

fn read_from_tokens(tokens:&mut Peekable<Iter<String>>) -> Data {
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

fn parse(program:&str) -> Data {
    return read_from_tokens(&mut tokenize(program).iter().peekable());
}

fn test_cons() {
    let parsed = parse("(begin (define r 10) (* pi (* r r)))");
    println!("{}",&parsed);
    println!("{}", &Symbol("Love".to_string()).cons(Symbol("You".to_string())));
    println!("{}", &Nil.cons(Nil));
    println!("{}", &Float(1.0).cons(Float(2.0)));
    println!("{}",&Nil.cons(Symbol("Test".to_string())));
    println!("{}",&parse("(1 2 3)"));
    println!("{}", &parse("(1 2 3)").map(|x| -> Data {
        match *x {
            Float(f) => Float(f + 2.0),
            _ => Float(0.0),
        }
    }))
}

/* Default Environment */
fn times(args:&Data) -> Data {
    match *args.car() {
        Float(f) => {
            match *args.cadr() {
                Float(g) => Float(f * g),
                _ => panic!("Cannot multiply these values."),
            }
        },
        _ => panic!("Cannot multiply these values."),
    }
}

fn add(args:&Data) -> Data {
    match *args.car() {
        Float(f) => {
            match *args.cadr() {
                Float(g) => Float(f + g),
                _ => panic!("Cannot add these values."),
            }
        },
        _ => panic!("Cannot add these values."),
    }
}
            

fn init() -> Env {
    let mut env:Env = Env::new();

    env.insertfn("*".to_string(),
                 Function::new(Box::new(times), 2));
    env.insertfn("+".to_string(),
                 Function::new(Box::new(add), 2));
    env
}

fn main() {
    let fn_map = &mut init();
    println!("{}", parse("(+ (* 4 (* 2 12)) 5)").eval(fn_map));
    println!("{}", parse("(begin (define r 10) (* 3.14 (* r r)))").eval(fn_map));
    println!("{}", parse("(begin (define pi 3.14) (* pi (* r r)))").eval(fn_map));
}
