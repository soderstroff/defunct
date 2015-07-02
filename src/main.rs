#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;
use Data::{Nil, Symbol, Float, Cons};

#[derive(PartialEq,Debug)]
enum Data {
    Nil,
    Symbol(String),
    Float(f32),
    Cons(Box<(Data,Data)>)
}

fn cons (car:Data, cdr:Data) -> Data {
    Data::Cons( Box::new( (car,cdr) ) )
}

fn atom(v:&Data) -> bool {
    match *v {
        Cons(_) => false,
        _ => true,
    }
}

fn nreverse (list: Data) -> Data{
    let mut curr =  list;
    let mut prv = Nil;
    
    while curr != Nil {
        match curr {
            Cons(ref mut c) =>{
                let next = &mut c.1;
                std::mem::swap(next, &mut prv);
            }
            _ => panic!("Not a proper list!"),
        };
        std::mem::swap(&mut curr, &mut prv);
    }
    prv
}

fn car (d:&Data) -> &Data {
    match *d {
        Cons(ref c) => &c.0,
        ref v @ _ => v,
    }
}

fn cdr (d:&Data) -> &Data {
    match *d {
        Cons(ref c) => &c.1,
        ref v @ _ => v,
    }
}

fn cadr (d:&Data) -> &Data {
    car(cdr(d))
}

fn print(list:&Data) {
    /* Pretty prints a list. Calls a helper function. */
    printh(list);
    print!("\n");
}

fn printh(list:&Data) {
    match *list {
        Float(s) => print!("{}", s),
        Symbol(ref s) => print!("{}", *s),
        Nil => print!("Nil"),
        Cons(ref c) => {
            print!("(");
            printh(&c.0);
            let mut focus = &c.1;
            while *focus != Nil {
                match *focus {
                    Nil => {unreachable!();},
                    Cons(ref c) => {
                        print!(" ");
                        printh(&c.0);
                        focus = &c.1;
                    }
                    _ => { print!(" . ");
                           printh(&focus);
                           print!(")");
                           return;
                    }
                }
            }
            print!(")");
        }
    };
}

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
        while *tokens.peek().unwrap() != ")" {
            l = cons(read_from_tokens(tokens), l);
        }
        tokens.next();
        return nreverse(l);
    }
            
    return Data::Symbol(token.to_string())
}

fn parse(program:&str) -> Data {
    return read_from_tokens(&mut tokenize(program).iter().peekable());
}

fn main() {
    let parsed = parse("(begin (define r 10) (* pi (* r r)))");
    print(&parsed);
    print(&cons(Symbol("Love".to_string()), Symbol("You".to_string())));
    print(&cons(Nil, Nil));
    print(&cons(Float(1.0), Float(2.0)));
    print(&cons(Nil, Symbol("Test".to_string())));
    print(&parse("(1 2 3 ())"));
}
