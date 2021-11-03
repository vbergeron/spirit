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
Function application is a bit weird for the moment and I am still figuring out a new synta
