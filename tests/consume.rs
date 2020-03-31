
#[macro_use]
extern crate closurize;


#[test]
fn fn_once() {

    #[closurize]
    trait Consumer<T> {
        fn consume(self, value: T); // TODO fn generics
    }

    fn accept_closure(closure: impl Consumer<f32>) {
        closure.consume(32.0);
    }

    accept_closure(|value| println!("consumed {}", value))
}

#[test]
fn baseline() {
    trait Consumer<T> {
        fn consume(self, value: T);
    }

    impl<F, T> Consumer<T> for F where F: FnOnce(T) {
        #[inline] fn consume(self, value: T) { self(value) }
    }

    fn accept_closure(closure: impl Consumer<f32>) {
        closure.consume(32.0);
    }

    accept_closure(|value| println!("consumed {}", value))
}