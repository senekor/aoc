# Module structure
There is no hard rule here, structuring modules well requires some experience. Some general rules of thumb:
- Things within a module should be closely related, while things in separate modules should be less related.
- Modules are too big if it's difficult to get an overview over everything that's happening in a module.
- Modules are too small if one has to jump from file to file to piece together an understanding of a single concept.
- As good starting point is to identify the few largest custom concepts / data types in you program and make a module for each one. Smaller types go into the module of the larger type they are most related to.

# API documentation
The public API of a library should always be well documented. For this purpose, put the following annotation in the file `lib.rs`:
```
#![deny(missing_docs)]
```
With this, your program won't even compile if you forget to add documentation to any part of your library that is exposed to the public. In most cases, you will need to add general documentation for the entire crate as well as for each public function. The crate is annotated with `//!` comments and functions are annotated with `///` comments. An example for `lib.rs`:
```
//! This is documentation
//! for the entire crate.
//! Is uses double slashes and a bang.

#![deny(missing_docs)]

/// This is documentation for the public function foo.
/// It uses triple slashes.
pub fn foo() {}

// The function bar is private and doesn't need to be documented.
// Regular double slash comments can be used.
fn bar() {}
```

# Naming conventions
- function names:
  - are _verbs_
  - convey whether the function is a _query_ or a _command_
  - contain no redundant information that is already contained in the function signature (parameter types & names, return type)
- variable names:
  - are _nouns_
  - clearly convey what value the variable contains
  - important: If the value of a variable changes during the execution of the program (the variable is mutable), does the variable's name _always_ match its content?

# Function size
Functions should _do one thing and one thing only_. Specifically, a function obeys this rule if it is impossible to _meaningfully extract_ another function from it.

# Abstraction layers
Here is a somewhat clumsy attempt to explain the idea in just a few paragraphs. In reality, this is a complicated problem, both to understand and to solve.

The highest abstraction layer is usually the problem domain. Examples: "Santa", "Submarine", "Bingo sheet". These concepts have nothing to do with programming or computers. The lowest abstraction layer contains strings, indexes, references and so on. Concepts in this lowest abstraction layer have little to nothing to do with your problem, but very much with programming and computers. There can be many layers between, depending on how complicated the problem domain is.

Having to think about both high and low abstraction concepts at the same time is very difficult and error prone. It is therefore good practise to separate them in the code. If you read a piece of code, you should only have to think in one abstraction layer to understand what the code is doing.

As a counter example, consider the following line of code which violates this principle:
```
let gender = "male";
let age: i32 = 41;
let person = (gender, age);
// ... imagine more code here ...
person.1 += 1;
```
Looking at all lines together, we can deduce that the last line of code must add one year to the age of the person. But if these lines are far apart, it would be almost impossible to guess what is happening. This is because the concepts "Person" and "signed 32-bit integer" are on completely different abstraction layers.

One possible way to separate the abstraction layers in this example is to wrap the code in an appropriately named method:
```
impl Person {
    fn increase_age(&mut self) {
        self.1 += 1;
    }
}

// usage:
person.increase_age();
```

# Unit testing
There are different opinions on how much testing should be required. For our purposes, we will adopt a rather strict rule of thumb: If a function is not immediately and obviously correct just by looking at it, it must be tested.
