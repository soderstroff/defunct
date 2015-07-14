# defunct
A new implementation of a dead language.

## Syntax and Semantics
An atomic value (a Float, Nil, or a String) all evaluate to themselves.
A symbol evaluates to the value bound to it in the environment.
A list evaluates to the result of applying the function represented by the symbol
in its car (head) to the arguments represented by its cdr (tail).
Evaluating a function results in a runtime error.

A list is written in the form (values*).

## Built-in Functions
`+` adds two numbers together.

`*` multiplies two numbers together.

`exit` takes no arguments and exits a session.

`-` subtracts one number from another.

`not` takes an argument and returns T if it is Nil.

`<` takes two arguments and returns T if the first is less than the second.

`>` is equivalent to (not (< a b)).

## Special Forms
Special forms are not evaluated like normal functions.

`(lambda (args*) (expr*)`

Creates a function object that can be applied to arguments.

`(if (test-form) (then-form*) (else-form*)`

If test-form evaluates to NIL, evaluates the then-form. Else, evaluates else-form.

`(define symbol (expr))`
Evaluate expr and bind its value to symbol in the nearest enclosing environment frame.

`(quote form)`
Returns the form, verbatim.

`(begin expr*)`
Evaluates any number of expressions, returning the last value. Also known as `progn`.

## Miscellaneous Concerns
Recursion will cause a stack overflow at about 618 function calls.

`(define sum (lambda (n) (if (< n 1) 0 (+ n (sum (- n 1))))))`

For instance, `(sum 500)` will panic.

This implementation is not rigorously tested for semantic compliance with any specification. There is no specification, save the prior notes.

## FAQ
Q: Why did you call it "defunct"? Are you bashing LISP?  
A: It is tongue-in-cheek. Clearly, lisp will never die. If that krevitch snorks your flads, you are needlessly pugnacious.

Q: How can I use this?  
A: Carefully.

Q: What can I do with this?  
A: It is not even good enough to be an arithmetic calculator. Do not use it.

Q: Will this get better?  
A: In time. I have barely even put my pants on.

Q: Why did you write it in Rust?  
A: We do it not because it is easy, but because it is hard.
