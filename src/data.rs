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
        let s = self.env.get(symbol).expect(
            &format!("Could not find free variable {}", symbol));
        s
    }

    pub fn insert(&mut self, symbol:String, d:Data) {
        self.env.insert(symbol, d);
    }

    pub fn lookupfn(&self, symbol:&String) -> &Function {
        self.fns.get(symbol).expect(
            &format!("Could not find function {}", symbol))
    }

    pub fn insertfn(&mut self, name:String, f:Function) {
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
    /* An executable function. Contains a function pointer
    and an arity. */
    pub fn new(f:LispFn, arity:usize) -> Function {
        Function {
            f: f,
            arity: arity,
        }
    }

    pub fn apply(&self, args:&Data) -> Data {
        let len = args.len();
        let arity = self.arity;
        if len != arity {
            panic!("Could not apply function of arity {} to {} args.",
                            arity, len);
        }
        else {
            (self.f)(args)
        }
    }
}

type FnMap = HashMap<String,Function>;

/* Lisp datatypes. A Cons is a
pointer to a pair, not the pair itself. */
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
        /* Reverses a list in place. Non-consing. */
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
            /* Applies f to every element in the list. */
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

    pub fn last(&self) -> &Data {
        if self.atom() {panic!("Not a proper list.")};
        let mut c = self;
        while *c.cdr() != Nil {
            c = c.cdr();
        }
        c.car()
    }

    pub fn eval(&self, env:&mut Env) -> Data {
        /* Evaluates a Lisp program. */
        match *self {
            Symbol(ref s) => env.lookup(s).clone(),
            /* A list is function application or a special form. */
            Cons(_) => {
                match *self.car() {
                    /* First try special forms. */
                    Symbol(ref s) if s == "if" => {
                        if self.car().eval(env) != Nil {
                            self.cadr().eval(env)
                        }
                        else { self.cdr().cadr().eval(env) }
                    }
                    Symbol(ref s) if s == "quote" => self.cadr().clone(),

                    Symbol(ref s) if s == "define" => {
                        match *self.cadr() {
                            Symbol(ref s) => {
                                let d = self.cdr().cadr().eval(env);
                                env.insert(s.clone(), d.clone());
                                d
                            },
                            _ => panic!("Error: {} is not a symbol!", *self.cadr()),
                        }},

                    Symbol(ref s) if s == "begin" => self.cdr()
                        .eval_list(env).last().clone(),
                        
                    _ => {
                        let args = self.cdr().eval_list(env);
                        let f = self.car().eval(env);
                        f.apply(&args, env)
                    }
                }
            }
            _ => self.clone()
        }
    }

    pub fn eval_list(&self, env:&mut Env) -> Data {
/*        match *self {
            Cons(_) => self.map(|d:&Data| d.eval(env)),
            _ => panic!("Not a list!"),
    }*/
        if self.atom() {panic!("Not a list!")};

        let mut mapped = Nil;
        let mut curr = self;
        while *curr != Nil {
            mapped = curr.car().eval(env).cons(mapped);
            curr = &curr.cdr();
        }
        
        mapped.nreverse()
    }

    pub fn apply(&self, args:&Data, env:&mut Env) -> Data {
        /* Applies a function too a list of arguments. */
        match *self {
            Func(ref s) => env.lookupfn(s).apply(args),
            _ => panic!("{} is not a function."),
        }
    }

    pub fn cons (self, cdr:Data) -> Data {
        /* Constructs a cons. Transfers ownership to new cons. */
        Cons( Box::new( (self,cdr) ) )
    }
    
}

impl fmt::Display for Data {
    /* Pretty printing support.*/
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
