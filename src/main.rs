#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::slice::Iter;
use std::iter::Peekable;
use std::collections::HashMap;

#[derive(PartialEq,Debug)]
enum Data {
    Nil,
    Symbol(String),
    Atom(f32),
    Cons ( Box<Data>, Box<Data> )
}

fn cons (car:Data, cdr:Data) -> Data {
    Data::Cons( Box::new(car), Box::new(cdr) )
}

fn nreverse (list: Data) -> Data{
    let mut curr = list;
    let mut prv = Box::new(Data::Nil);
    
    while curr != Data::Nil {
        match curr {
            Data::Nil => unreachable!(),
            Data:: Symbol(_) |Data::Atom(_) => {panic!("Not a proper list!")},
            Data::Cons(_, ref mut next) =>{
               std::mem::swap(next, &mut prv);
            }            
        };
        std::mem::swap(&mut curr, &mut prv);

    }
    *prv
}

fn car (d:Data) -> Data {
    match d {
        Data::Nil => Data::Nil,
        Data::Cons(a, _) => *a,
        v@Data::Symbol(_) | v@Data::Atom(_) => v,
    }
}

fn cdr (d:Data) -> Data {
    match d {
        Data::Nil => Data::Nil,
        Data::Cons(_, b) => *b,
        v@Data::Symbol(_) | v@Data::Atom(_) => v,
    }
}

fn cadr (d:Data) -> Data {
    car(cdr(d))
}

macro_rules! list {
    ( $( $item:expr),* ) => {
        {
            let mut list:Data = Data::Nil;
            $(
                // TODO: Make consable a trait, and get this working. 
                list = cons(Data::Atom($item), list);
                )*
                list
        }
    };
}

fn print(list:&Data) {
    printh(list);
    print!("\n");
}

fn printh(list:&Data) {
    match *list {
        Data::Atom(ref s) => print!("{}", *s),
        Data::Symbol(ref s) => print!("{}", *s),
        Data::Nil => print!("Nil"),
        Data::Cons(ref a, ref b) => {
            print!("(");
            printh(a);
            let mut c = b;
            while **c != Data::Nil {
                match **c {
                    Data::Nil => {unreachable!();},
                    Data::Cons(ref a, ref b) => {
                        print!(" ");
                        printh(a);
                        c = b;
                    }
                    _ => { print!(" . ");
                           printh(&*c);
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
        let mut l = Data::Nil;
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

fn eval(val:Data, env:HashMap<String,f32>) -> f32 {
    match val {
        Data::Symbol(ref s) => *env.get(s).unwrap(),
        Data::Atom(f) => f,
        Data::Nil => unreachable!(),
        Data::Cons(a, b) => apply(*a, *b),
    }
}

fn apply(fun:Data, arguments:Data) -> f32 {
    return 0.0;
}

fn main() {
    let mut env:HashMap<&str,Data> = HashMap::new();
    let parsed = parse("(begin (define r 10) (* pi (* r r)))");
    print(&parsed);
    print(&cons(Data::Symbol("Love".to_string()), Data::Symbol("You".to_string())));
}
