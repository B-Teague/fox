# fox
fox - f of x is a interpreted statically typed immutable language written in Rust

This syntax is still in alpha.  Suggestions for simplifying the syntax are welcome.

# features
* Statically inferred data types
* Immutable
* First class functions
* Functions as values
* Function chaining
* Tail call optimizations
* No nulls, no thrown errors, only monads
* Handlebars String templates
* pattern matching
* error propogation

# Example syntax
## Hello World
```fox
hello = "Hello, World!"
main = [] println(`{hello}`)
```

## Data Types
```fox
int = 1
float = 2.0f
double = 3.0
string = "hello"
bool = true
tuple = [0,"a",true]
map = [ a: 1, 2: 2, "hi": 3 ]
```
## Branching
```fox
  match(1 == 2)
  .when(true, "They match!")
  .else("Different values")
```
## Monads
```fox
  map = [ a:1, b:2 ]
  map
  .c
  .Some([val] val)
  .None([] "Not Found")
```

## Sample Functions
```fox
factorial = [num] match(num)
  .when([0,1], 1)
  .else([num] num * factorial(num - 1))

factorial(10)
```
