/* Implementation of lisp data types, especially Cons. */

/* Imports */
use std::collections::HashMap;
use std::fmt;
use std::mem::swap;

/* An Environment frame. Has a parent, unless it is the root frame.*/
pub struct Env {
    env:HashMap<String,Data>,
    fns:FnMap,
    parent:Box<Option<Env>>
}

impl Env {
    pub fn new() -> Env {
        Env {
            env: HashMap::new(),
            fns: HashMap::new(),
            parent: Box::new(None),
        }
    }
    
    pub fn lookup(&self, symbol:&String) -> &Data {
        let s = self.env.get(symbol).expect("Could not find free variable!");
        println!("Found {} in environment!", s);
        s
    }

    fn lookupfn(&self, symbol:&String) -> &Function {
        self.fns.get(symbol).expect("Could not find function in environment.")
    }

    pub fn insertfn(&mut self, name:String, f:Function) {
        let name = name.to_string();
        self.env.insert(name.clone(), Func(name.clone()));
        self.fns.insert(name.clone(), f);
    }
}

pub type LispFn = Box<Fn(&Data) -> Data>;
pub struct Function {
    pub f:Box<Fn(&Data) -> Data>,
    pub arity:usize
}

impl Function {
    pub fn new(f:LispFn, arity:usize) -> Function {
        Function {
            f: f,
            arity: arity,
        }
    }

    pub fn apply(&self, args:&Data) -> Data {
        if args.len() != self.arity {
            panic!("Arity error!");
        }
        else {
            (self.f)(args)
        }
    }
}

type FnMap = HashMap<String,Function>;

#[derive(PartialEq, Clone, Debug)]
pub enum Data {
    Nil,
    Symbol(String),
    Float(f32),
    Func(String),
    Cons(Box<(Data,Data)>)
}
use data::Data::{Nil, Symbol, Float, Cons, Func};

impl Data {
    pub fn atom(&self) -> bool {
        match *self {
            Cons(_) => false,
            _ => true,
        }
    }

    pub fn car (&self) -> &Data {
        match *self {
            Cons(ref c) => &c.0,
            Nil => self,
            _ => panic!("Not a cons!")
        }
    }

    pub fn cdr (&self) -> &Data {
        match *self {
            Cons(ref c) => &c.1,
            Nil => self,
            _ => panic!("Not a cons!")
        }
    }

    pub fn cadr (&self) -> &Data {
        self.cdr().car()
    }

    pub fn nreverse (self) -> Data{
        let mut curr = self;
        let mut prv = Nil;
        
        while curr != Nil {
            match curr {
                Cons(ref mut c) =>{
                    let next = &mut c.1;
                    swap(next, &mut prv);
                }
                _ => panic!("Not a proper list!"),
            };
            swap(&mut curr, &mut prv);
        }
        prv
    }

    pub fn map<F> (&self, f:F) -> Data 
        where F : Fn(&Data) -> Data {
            if self.atom() {panic!("Not a list!")};

            let mut mapped = Nil;
            let mut curr = self;
            while *curr != Nil {
                mapped = (f(curr.car())).cons(mapped);
                curr = &curr.cdr();
            }

            mapped.nreverse() 
        }

    pub fn len(&self) -> usize {
        match *self {
            Nil => 0,
            Cons(_) => {
                
                let mut count:usize = 1;
                let mut current = self;
                
                while *current.cdr() != Nil {
                    match *current.cdr() {
                        Cons(_) => {
                            count += 1;
                            current = &current.cdr();
                        },
                        _ => return count + 1
                    }
                }

                count
            },
            _ => panic!("Not a list!"),
        }
    }
    
    
    pub fn eval(&self, env:&Env) -> Data {
        match *self {
            Symbol(ref s) => { println!("Found {}", *s); env.lookup(s).clone()},
            Cons(_) => {
                match *self.car() {
                    Symbol(ref s) if *s == "if".to_string() => {
                        if self.car().eval(env) != Nil {
                            self.cadr().eval(env)
                        }
                        else { self.cdr().cadr().eval(env) }
                    }
                    Symbol(ref s) if *s == "quote".to_string() => self.cadr().clone(),

                    //Symbol("define") => to-do,
                    _ => self.car().eval(env)
                        .apply(&(self.cdr().map(|d| d.eval(env)))
                               , env)
                }
            }
            _ => self.clone()
        }
    }

    pub fn apply(&self, args:&Data, env:&Env) -> Data {
        match *self {
            Func(ref s) => env.lookupfn(s).apply(args),
            _ => panic!("{} is not a function."),
        }
    }

    pub fn cons (self, cdr:Data) -> Data {
        Cons( Box::new( (self,cdr) ) )
    }
    
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Float(s) => write!(f, "{}", s),

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
            Symbol(ref s) | Func(ref s) => write!(f, "{}", *s),
        }
    }
}
