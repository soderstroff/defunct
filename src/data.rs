/// Implementation of lisp data types and the evaluation environment.

// Imports
use std::collections::HashMap;
use std::fmt;
use std::mem::swap;
use std::process::exit;
pub use data::Data::*;

/// An environment frame. Has a parent, unless it is the root frame.
pub struct Env {
    env: HashMap<String,Data>,
    fns: FnMap,
    parent: Box<Option<Env>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            env: HashMap::new(),
            fns: HashMap::new(),
            parent: Box::new(None),
        }
    }
    
    pub fn lookup(&self, symbol: &String) -> &Data {
        self.env.get(symbol)
            .expect(&format!("Could not find free variable {}", symbol))
    }

    pub fn insert(&mut self, symbol: String, d: Data) {
        self.env.insert(symbol, d);
    }

    pub fn lookupfn(&self, symbol: &String) -> &Function {
        self.fns.get(symbol).expect(
            &format!("Could not find function {}", symbol))
    }

    pub fn insertfn(&mut self, name: String, f: Function) {
        self.env.insert(name.clone(), Func(name.clone()));
        self.fns.insert(name.clone(), f);
    }

    pub fn init() -> Env {
        let mut env:Env = Env::new();

        env.insertfn("*".to_string(),
                     Function::new(Box::new(times), 2));
        env.insertfn("+".to_string(),
                     Function::new(Box::new(add), 2));
        env.insertfn("exit".to_string(),
                     Function::new(Box::new(quit),0));

        env
    }

}

/// Default environment. 
fn times (args: &Data) -> Data {
    match (args.car(), args.cadr()) {
        (&Float(f), &Float(g)) => Float(f * g),
        _ => panic!("Cannot multiply these values."),
    }
}

fn add(args: &Data) -> Data {
    match (args.car(), args.cadr()) {
        (&Float(f), &Float(g)) => Float(f + g),
        _ => panic!("Cannot add these values."),
    }
}

#[allow(unused_variables)]
fn quit(args: &Data) -> Data {
    println!("Exiting session.");
    exit(0)
}

pub type LispFn = Box<Fn(&Data) -> Data>;
pub struct Function {
    pub f: Box<Fn(&Data) -> Data>,
    pub arity: usize
}

/// An executable function.
/// Contains a function pointer and an arity.
impl Function {
    pub fn new(f: LispFn, arity: usize) -> Function {
        Function {
            f: f,
            arity: arity,
        }
    }

    pub fn apply(&self, args: &Data) -> Data {
        let len = args.len();
        if len != self.arity {
            panic!("Could not apply function of arity {} to {} args.",
                            self.arity, len);
        }
        else {
            (self.f)(args)
        }
    }
}

type FnMap = HashMap<String,Function>;

/// Lisp datatypes. A Cons is a
/// pointer to a pair, not the pair itself.
#[derive(PartialEq, Clone, Debug)]
pub enum Data {
    Nil,
    Symbol(String),
    Float(f32),
    Func(String),
    Cons(Box<(Data,Data)>)
}

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

    /// Reverses a list in place. Non-consing.
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

    pub fn map (&self, f: fn(&Data) -> Data) -> Data {
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

    fn eval_special(&self, s: &str, env: &mut Env) -> Data {
        match s {
            "if" => {
                if self.car().eval(env) != Nil {
                    self.cadr().eval(env)
                }
                else { self.cdr().cadr().eval(env) }
            },
            
            "quote" => self.clone(),

            "define" => {
                match *self.car() {
                    Symbol(ref s) => {
                        let d = self.cadr().eval(env);
                        env.insert(s.clone(), d.clone());
                        d
                    },
                    _ => panic!("Error: {} is not a symbol!", *self.cadr()),
                }},

            "begin" => self.eval_list(env).last().clone(),

            _ => unreachable!(),
        }
    }

    /// Evaluates a Lisp program.
    pub fn eval(&self, env:&mut Env) -> Data {
        match *self {
            
            // A list is function application or a special form. 
            Cons(_) => {
                match *self.car() {
                    // First try special forms.
                    Symbol(ref s) if special(s) => self.cdr().eval_special(s, env),
                    
                    // Function application. Retrieve the function object
                    // from the environment, evaluate all the argument expressions,
                    // then apply the function to the arguments.
                    _ => {
                        let f = self.car().eval(env);
                        let args = self.cdr().eval_list(env);
                        f.apply(&args, env)
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
        if self.atom() && *self != Nil {panic!("Not a list!")};

        let mut mapped = Nil;
        let mut curr = self;
        while *curr != Nil {
            mapped = curr.car().eval(env).cons(mapped);
            curr = &curr.cdr();
        }
        
        mapped.nreverse()
    }

    /// Applies a function too a list of arguments. 
    pub fn apply(&self, args:&Data, env:&mut Env) -> Data {
        match *self {
            Func(ref s) => env.lookupfn(s).apply(args),
            _ => panic!("{} is not a function."),
        }
    }

    
    /// Constructs a cons. Transfers ownership to new cons. 
    pub fn cons (self, cdr:Data) -> Data {
        Cons( Box::new( (self,cdr) ) )
    }
    
}

fn special(s: &str) -> bool {
    match s {
        "define" | "if" | "quote" | "begin" => true,
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
                while *focus != Nil {
                    match *focus {
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
            Func(ref s) => write!(f, "#<Function {}>", *s),
        }
    }
}
