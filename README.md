# CLOSURIZE

Automatically make any closure implement your trait!

```rust
#[closurize]
trait Pancakes {
    fn is_delicious(&self) -> bool;
}

fn main() {
    // we can pass a closure because 
    // all suitable closures now also implement Pancakes
    expect_delicious(|| true);
}

fn expect_delicious(pancakes: impl Pancakes) {
    if !pancakes.is_delicious() {
        panic!("what happened to my pancakes :(");
    }
}
```

Using the procedural macro `closurize` as an attribute to your trait 
enables you to use closures anywhere your trait is required.

However, this only works for traits that have only a single method taking `self`.

This allows you to use an named trait instead of writing `Fn(X,Y) -> Z` everywhere.
For an actual example, see the [exrs crate](https://github.com/johannesvollmer/exrs/blob/master/src/image/rgba.rs#L108).