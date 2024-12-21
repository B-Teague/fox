# fox
fox - f of x is a interpreted statically typed immutable language written in Rust

This syntax is still in alpha.  Suggestions for simplifying the syntax are welcome.

# features
* Statically inferred data types
* Immutable data structures
* Destructive Updates with Uniqueness Analysis
* First class functions
* Static dispatch
* Tail call optimizations
* Tagged Unions
* Handlebars String templates
* Pattern matching
* Error propogation

# Example syntax
## Hello World
```fox
hello = "Hello, World!"
main = ()->print("{hello}")
```

## Data Types
```fox
1:Int
2.0f:Float
3.0:Decimal
"hello":Str
true:Bool
[1,2,3]:List
(0,"a",true):Tuple
{ a: 1, b: 2, c: "Hi" }: {a:Int, b:Int, c:Str}
```
## Pattern Matching
```fox
  1 == 2 ??
  true -> "They match!"
  false -> "Different values"
```

## Sample Functions
```fox
factorial=num->
  num ??
  0 -> 1
  _ -> num * factorial(num - 1)

factorial(10)
```
