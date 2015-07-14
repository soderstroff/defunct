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
