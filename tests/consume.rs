
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

    let val = 32.0;
    accept_closure(move |value| assert_eq!(value, val))
}

#[test]
fn fn_ref() {

    #[closurize]
    trait Consumer<T> {
        fn consume(&self, value: T); // TODO fn generics
    }

    fn accept_closure(closure: impl Consumer<f32>) {
        closure.consume(32.0);
    }

    let expected = 32.0;
    accept_closure(|value| assert_eq!(value, expected))
}

#[test]
fn fn_mut() {

    #[closurize]
    trait Consumer<T> {
        fn consume(&mut self, value: T); // TODO fn generics
    }

    fn accept_closure(mut closure: impl Consumer<f32>) {
        closure.consume(32.0);
    }

    let mut works = None;
    accept_closure(|value| {
        works = Some(value == 32.0)
    });

    assert_eq!(works, Some(true));
}
