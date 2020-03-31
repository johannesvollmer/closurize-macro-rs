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