# spirit
A small functional language.

## Syntax

### Nil
`Nil` is a special construct that represent the absence of result.
It is worth noting Nil cannot be constructed.

### Numbers
Spirit numbers are Rust's `i64`.

### Litterals
Spirit feature litterals / symbols.

### Scoping
To add a binding in the current scope, use the `def .. = ..` keyword. The binding will never be cleaned so keep it for the top level.
```
def one = 1
```
To have a more refined definition and have a cleaning of the variable after use `let .. = .. in ..`.
```
let one = 1 in one
```

### Functions

#### Function declaration
All functions are anonymous and currified.
Here is an exemple of a function that returns the litteral `awesome` for any input.
```
fn n -> awesome
```
Functions of two of more arguments are simply functions that return functions.
```
fn x -> fn y -> double_take
```
#### Function application
Function application is a bit weird for the moment and I am still figuring out a good syntax.
The to apply `f` to `x` you meed to write.
```
@ f x
```
For instance (and showcase the use of the print function) :
```
def x = youdabest
> Nil
@ print x
> youdabest
```
The syntax get a bit messy to use function of two variables :
```
@ @ add 1 3
> 4
```

#### Partial application
Since everything is currified, you can define a partial function easily :
```
@ print add
> fn y -> fn x -> native native:add x y
def add2 = @ add 2
> Nil
@ print add2
> fn y -> let x = 2 in native native:add x y
```
The partial application create a closure (the `let x = 2 in ...`) that get cleaned from the environment afterwards.

### Conditional
There is am `if` expression syntax. The only trick is that the condition is realized if and only if it evaluates to the litteral `true`.
The `eq`, `lt`, and `gt` builtin functions works with both litterals (with lexicographic order) and numbers.
```
def spirit? = fn word -> 
  if @ @ eq word spirit 
  then yay
  else neh
> Nil
@ spirit? spirit
> yay
@ spitit? potato
> neh
```

### Showcase
Here is a definition of the factorial :

```
def fact = n ->
  if @ @ lt n 2 then 
    1
  else
    @ @ mul 
      n 
      @ fact @ @ sub n 1 
```
