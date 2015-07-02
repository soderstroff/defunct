#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]


use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;
use std::fmt;
use Data::{Nil, Symbol, Float, Cons};

#[derive(PartialEq,Debug)]
enum Data {
    Nil,
    Symbol(String),
    Float(f32),
    Cons(Box<(Data,Data)>)
}

impl Data {
    fn atom(&self) -> bool {
        match *self {
        Cons(_) => false,
            _ => true,
        }
    }

    fn car (&self) -> &Data {
        match *self {
            Cons(ref c) => &c.0,
            ref v @ _ => v,
        }
    }

    fn cdr (&self) -> &Data {
        match *self {
            Cons(ref c) => &c.1,
            ref v @ _ => v,
        }
    }

    fn cadr (&self) -> &Data {
        self.cdr().car()
    }

    fn nreverse (self) -> Data{
        let mut curr = self;
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
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Float(s) => write!(f, "{}", s),
            Symbol(ref s) => write!(f, "{}", *s),
            Nil => write!(f, "Nil"),
            Cons(ref c) => {
                write!(f, "({}", &c.0);
                let mut focus = &c.1;
                while *focus != Nil {
                    match *focus {
                        Nil => {unreachable!();},
                        Cons(ref c) => {
                            write!(f, " {}", &c.0);
                            focus = &c.1;
                        }
                        _ => return write!(f, " . {})", focus)
                    }
                }
                write!(f, ")")
            }
        }
    }
}
    
fn cons (car:Data, cdr:Data) -> Data {
    Data::Cons( Box::new( (car,cdr) ) )
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
        return l.nreverse();
    }
            
    return Data::Symbol(token.to_string())
}

fn parse(program:&str) -> Data {
    return read_from_tokens(&mut tokenize(program).iter().peekable());
}

fn main() {
    let parsed = parse("(begin (define r 10) (* pi (* r r)))");
    println!("{}",&parsed);
    println!("{}",&cons(Symbol("Love".to_string()), Symbol("You".to_string())));
    println!("{}", cons(Nil, Nil));
    println!("{}",(&cons(Float(1.0), Float(2.0))));
    println!("{}",&cons(Nil, Symbol("Test".to_string())));
    println!("{}",&parse("(1 2 3 ())"));
}
