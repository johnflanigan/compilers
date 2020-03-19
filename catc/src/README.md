Below is a list of import lines you can use in main.rs:

```rust
/* Data Structures */
pub mod common;

#[macro_use]
pub mod lir;

#[macro_use]
pub mod x64;

#[macro_use]
pub mod x64s;

pub mod backend;

pub mod source_grammar;

pub mod lowering;

pub mod check_type;

pub mod checked_grammar;

#[cfg(test)]
pub mod test_type_check;
```

In the files included here "common" has changed to include the 
`InfixSourceOp` which is used in the type-checked and non-type-checked versions
of the AST.

"source_grammar" defines the data structure that represents the structure of
the source code.

"lowering" contains a basic template for you to fill in.

"check_type" is the type checker. Don't look at it. It isn't a model of a good
type checker, it isn't interesting, all it does is some basic checks, produces
symbols, produces type ids and the following mappings:
        symbol -> type_id
        type_id -> type
        label -> function type
These mappings will be helpful for lowering.

"checked_grammar" is the grammar which you will be lowering from. It contains
the mappings discussed above and the symbol generator which was used to
generate the symbols. So you can continue to generate symbols in the
lower passes. Here you will also find the definition of `Type` and `TypeId`.

"test_type_check" is a test of some very basic programs so that you can
see the type checking in action. You can add tests by following the pattern.
Tests pass if the function doesn't panic, and fail if they do. You can
`assert!(condition)` to check that `condition == true` inside a test (`assert`
panics if the condition is false). To run the test suit run `cargo test`.
To see the things which you have printed inside a test run
`cargo test -- --nocapture`.
