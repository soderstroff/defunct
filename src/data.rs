    /// Implementation of lisp data types and the evaluation environment.

// Imports
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::swap;
use std::fmt;
use std::process::exit;
pub use data::Data::*;

/// Lisp datatypes. A Cons is a
/// pointer to a pair, not the pair itself.
#[derive(PartialEq, Clone)]
pub enum Data {
    Nil,
    Symbol(String),
    Float(f64),
    Function {
        args: Box<Data>,
        body: Box<Data>,
        env: Env,
    },
    PrimitiveFn(String),
    Cons(Rc<RefCell<(Data,Data)>>)
}

impl Data {
    pub fn atom(&self) -> bool {
        if let Cons(_) = *self { true }
        else { false }
    }

    pub fn car (&self) -> Data {
        match *self {
            Cons(ref c) => c.borrow().0.clone(),
            Nil => Nil,
            _ => panic!("{} is not a cons!", self)
        }
    }

    pub fn cdr (&self) -> Data {
        match *self {
            Cons(ref c) => c.borrow().1.clone(),
            Nil => Nil,
            _ => panic!("Not a cons!")
        }
    }

    pub fn cadr (&self) -> Data {
        self.cdr().car()
    }

    /// Reverses a list in place. Non-consing.
    pub fn nreverse (self) -> Data {
        let mut curr = self;
        let mut prv = Nil;

        while curr != Nil {
            match curr {
                Cons(ref c) =>{
                    let next = &mut c.borrow_mut().1;
                    swap(next, &mut prv);
                }
                _ => panic!("Not a proper list!"),
            };
            swap(&mut curr, &mut prv);
        }
        prv
    }

    pub fn len(&self) -> usize {
        match *self {
            Nil => 0,
            Cons(_) => {

                let mut count:usize = 1;
                let mut current = self.clone();

                while current.cdr() != Nil {
                    match current.cdr() {
                        Cons(_) => {
                            count += 1;
                            current = current.cdr();
                        },
                        _ => return count + 1
                    }
                }

                count
            },
            _ => panic!("Not a list!"),
        }
    }

    pub fn last(&self) -> Data {
        if self.atom() { panic!("Not a proper list.") };
        let mut c = self.clone();
        while c.cdr() != Nil {
            c = c.cdr();
        }
        c.car().clone()
    }

    fn eval_special(&self, special: &str, env: &mut Env) -> Data {
        match special {
            "if" => {
                if self.car().eval(env) != Nil {
                    self.cadr().eval(env)
                }
                else { self.cdr().cadr().eval(env) }
            },

            "quote" => self.car(),

            "define" => {
                if let Symbol(ref s) = self.car() {
                    let expr = self.cadr().eval(env);
                    env.insert(s.clone(), expr.clone());
                    expr
                }
                else { panic!("Error: {} is not a symbol!", self.cadr()) }
            },

            "begin" => self.eval_list(env).last(),

            "lambda" => Data::make_fn(self.car(), self.cadr(), env.clone()),

            _ => unreachable!(),
        }
    }

    /// Evaluates a Lisp program.
    pub fn eval(&self, env:&mut Env) -> Data {
        match *self {
            // A list is function application or a special form.
            Cons(_) => {
                match self.car() {
                    // First try special forms.
                    Symbol(ref s) if special(s) => self.cdr().eval_special(s, env),
                    // Otherwise, it must be a regular function call.
                    _ => {
                        let mut f = self.car().eval(env);
                        let args = self.cdr().eval_list(env);
                        f.apply(&args)
                    }
                }
            }
            Symbol(ref s) => env.lookup(s).clone(),
            _ => self.clone()
        }
    }

    pub fn eval_list(&self, env:&mut Env) -> Data {
        /* One day this will work. One day!!
        match *self {
            Cons(_) => self.map(|d:&Data| -> Data{ d.eval(env) }),
            _ => panic!("Not a list!"),
    }
         */
        match *self {
            Nil => Nil,
            Cons(_) => {
                let mut mapped = Nil;
                let mut curr = self.clone();
                while curr != Nil {
                    mapped = curr.car().eval(env).cons(mapped);
                    curr = curr.cdr();
                }

                mapped.nreverse()
            },
            _ => panic!("Not a list!")
        }
    }

    /// Applies a function to a list of arguments.
    pub fn apply(&mut self, args:&Data) -> Data {
        match *self {
            Function{ args: ref params, ref body, ref mut env } => {
                if params.len() == args.len() {
                    let mut scope = Env::new(env.clone());
                    let mut a = (**params).clone();
                    let mut b = (*args).clone();
                    while a.car() != Nil && b.car() != Nil {
                        match a.car() {
                            Symbol(ref s) => scope.insert(s.clone(), b.car().clone()),
                            _ => unreachable!("In apply"),
                        }
                        a = a.cdr();
                        b = b.cdr();
                    }

                    body.eval(&mut scope)
                }
                else { panic!("Arity mismatch. Needed {}, but called with {}",
                              params.len(), args.len()); }
            },

            PrimitiveFn(ref s) => {
                let f = PRIMITIVES.get(s).unwrap();
                return f(args)
            }

            _ => panic!("{} is not a function."),
        }
    }

    /// Constructs a cons.
    pub fn cons (self, cdr:Data) -> Data {
        Cons(
            Rc::new( RefCell::new( (self.clone(), cdr.clone()) ) )
        )
    }

    pub fn make_fn(args: Data, body: Data, env: Env) -> Data {
        Function {
            args: Box::new(args),
            body: Box::new(body),
            env: env,
        }
    }

}

fn special(s: &str) -> bool {
    match s {
        "define" | "if" | "quote" | "begin" | "lambda" => true,
        _ => false,
    }
}

/// Pretty printing support.
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Float(s) => write!(f, "{}", s),
            Nil => write!(f, "Nil"),
            Cons(_) => {
                write!(f, "({}", self.car());
                let mut focus = self.cdr();
                while focus != Nil {
                    match focus {
                        Nil => unreachable!(),
                        Cons(_) => {
                            write!(f, " {}", focus.car());
                            focus = focus.cdr();
                        }
                        _ => return write!(f, " . {})", focus)
                    }
                }
                write!(f, ")")
            }
            Symbol(ref s) => write!(f, "{}", *s),
            PrimitiveFn(ref s) => write!(f, "#<Function {}>", *s),
            Function{..} => write!(f, "#<Anonymous Function>"),
        }
    }
}


/// An environment frame. Has a parent, unless it is the root frame.
struct EnvTable {
    env: HashMap<String, Data>,
    parent: Option<Env>,
}

/// A pointer to an environment frame.
#[derive(Clone,PartialEq)]
pub struct Env(Rc<RefCell<EnvTable>>);

lazy_static! {
    static ref PRIMITIVES: HashMap<String, fn(&Data) -> Data> = {
        let mut p: HashMap<String, fn(&Data) -> Data> = HashMap::new();
        for (ident, op) in vec!(("*", multiply as fn(&Data)->Data), ("+", add), ("exit", quit), ("<", lt),
            (">", gt), ("not", not), ("-", subtract), ("/", divide),
            ("cons", cons), ("car", car), ("cdr", cdr), ("length", length)) {
                p.insert(ident.to_string(), op);
            }
        p
    };
}

impl Env {
    pub fn new(parent: Env) -> Env {
        let e = EnvTable {
            env: HashMap::new(),
            parent: Some((parent.clone())),
        };
        Env(Rc::new(RefCell::new(e)))
    }

    pub fn new_root() -> Env {
        let table = EnvTable {
            env: HashMap::new(),
            parent: None,
        };
        let mut env = Env(Rc::new(RefCell::new(table)));
        env.init_env();
        env
    }

    pub fn lookup(&self, symbol: &String) -> Data {
        let table = self.0.borrow();
        if let Some(ref value) = table.env.get(symbol) {
            (*value).clone()
        }
        else {
            let parent = &table.parent;
            match parent {
                &Some(ref v) => v.lookup(symbol),
                &None => panic!("Could not find free variable {}", symbol)
            }
        }
    }

    pub fn insert(&mut self, symbol: String, d: Data) {
        self.0.borrow_mut().env.insert(symbol, d);
    }

    pub fn add_primitive(&mut self, name: &str) {
        let name = name.to_string();
        self.insert(name.clone(), PrimitiveFn(name));
    }

    pub fn init_env(&mut self) {
        for prim in vec!("+", "*", "exit", "<", ">", "not", "-", "/", "cons", "car", "cdr", "length") {
            self.add_primitive(prim);
        }
    }
}

macro_rules! function {
    (name: $name:ident,
     arity: $arity:expr,
     args: $args:ident
     body: $body:block) => {
         fn $name ($args: &Data) -> Data {
             if $args.len() != $arity {
                 panic!("{} expected {}, but was called with {}", stringify!($name), $arity, $args.len());
             }
             else $body
         }}}

macro_rules! math {
    ($op:tt, $name:ident) => {
        function! {
            name: $name, arity: 2, args: args
            body: {
                match (args.car(), args.cadr()) {
                    (Float(f), Float(g)) => Float( f $op g),
                    _ => panic!("Cannot {} these values.", stringify!($name)),
                }}}}}

math!(+, add);
math!(-, subtract);
math!(*, multiply);
math!(/, divide);


use std::f64;
function!{ name: not, arity: 1, args:args
    body: {
        match args.car() {
            Nil => Float(f64::NAN),
            _ => Nil,
        }}}

function!{ name: lt, arity: 2, args:args
    body: {
        if let (Float(a), Float(b)) = (args.car(), args.cadr()) {
            if a < b { Float(f64::NAN) }
            else { Nil }
        }
        else { panic!("Couldn't compare non-numbers.") }
    }
}

function!{ name: gt, arity: 2, args: args
    body: {
        not(&lt(args).cons(Nil))
    }
}

function! { name: cons, arity: 2, args: args
    body: { args.car().cons(args.cadr()) }}

function! { name: car, arity: 1, args:arg
    body: { arg.car().car() }}

function! { name: cdr, arity: 1, args:arg
    body: { arg.car().cdr() }}

function! { name: length, arity: 1, args:arg
    body: { Float(arg.car().len() as f64) }}

#[allow(unused_variables)]
fn quit(args: &Data) -> Data {
    println!("Exiting session.");
    exit(0)
}

impl PartialEq for EnvTable {
    fn eq(&self, _rhs:&Self) -> bool { false }
}
