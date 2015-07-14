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
    Float(f32),
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
                    env.borrow_mut().insert(s.clone(), expr.clone());
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
            Symbol(ref s) => env.borrow_mut().lookup(s).clone(),
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
                    let mut scope = EnvTable::new(env.clone());
                    let mut a = (**params).clone();
                    let mut b = (*args).clone();
                    while a.car() != Nil && b.car() != Nil {
                        match a.car() {
                            Symbol(ref s) => scope.borrow_mut().insert(s.clone(), b.car().clone()),
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
                let f = primitives.get(s).unwrap();
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
#[derive(Clone)]
pub struct EnvTable {
    env: HashMap<String, Data>,
    parent: Option<Env>,
}

pub type Env = Rc<RefCell<EnvTable>>;

lazy_static! {
    static ref primitives: HashMap<String, fn(&Data) -> Data> = {
        let mut p: HashMap<String, fn(&Data) -> Data> = HashMap::new();
        for (ident, op) in vec!(("*", times as fn(&Data)->Data), ("+", add), ("exit", quit), ("<", lt),
            (">", gt), ("not", not), ("-", minus)) {
                p.insert(ident.to_string(), op);
            }
        p
    };
}

impl EnvTable {
    pub fn new(parent: Env) -> Env {
        let e = EnvTable {
            env: HashMap::new(),
            parent: Some((parent.clone())),
        };
        Rc::new(RefCell::new(e))
    }

    pub fn new_root() -> Env {
        let mut e = EnvTable {
            env: HashMap::new(),
            parent: None,
        };
        e.init_env();
        Rc::new(RefCell::new(e))
    }

    pub fn lookup(&self, symbol: &String) -> Data {
        if let v@Some(_) = self.env.get(symbol) { v.unwrap().clone() }
        else {
            match self.parent {
                Some(ref v) => v.borrow().lookup(symbol),
                None => panic!("Could not find free variable {}", symbol)
            }
        }
    }

    pub fn insert(&mut self, symbol: String, d: Data) {
        self.env.insert(symbol, d);
    }

    pub fn add_primitive(&mut self, name: &str) {
        let name = name.to_string();
        self.insert(name.clone(), PrimitiveFn(name));
    }

    pub fn init_env(&mut self) {
        self.add_primitive("+");
        self.add_primitive("*");
        self.add_primitive("exit");
        self.add_primitive("<");
        self.add_primitive(">");
        self.add_primitive("not");
        self.add_primitive("-");
    }
}

/// Default environment.
fn times (args: &Data) -> Data {
    match (args.car(), args.cadr()) {
        (Float(f), Float(g)) => Float(f * g),
        _ => panic!("Cannot multiply these values."),
    }
}

fn add(args: &Data) -> Data {
    match (args.car(), args.cadr()) {
        (Float(f), Float(g)) => Float(f + g),
        _ => panic!("Cannot add these values."),
    }
}

fn minus(args: &Data) -> Data {
    match (args.car(), args.cadr()) {
        (Float(f), Float(g)) => Float(f - g),
        _ => panic!("Cannot subtract these values."),
    }
}

fn not(args: &Data) -> Data {
    match args.car() {
        Nil => Float(0.0),
        _ => Nil,
    }
}

fn lt(args: &Data) -> Data {
    if let (Float(a), Float(b)) = (args.car(), args.cadr()) {
        if a < b { Float(0.0) }
        else { Nil }
    }
    else { panic!("Couldn't compare non-numbers.") }
}

fn gt(args: &Data) -> Data {
    not(&lt(args))
}

#[allow(unused_variables)]
fn quit(args: &Data) -> Data {
    println!("Exiting session.");
    exit(0)
}

impl PartialEq for EnvTable {
    fn eq(&self, _rhs:&Self) -> bool { false }
}
