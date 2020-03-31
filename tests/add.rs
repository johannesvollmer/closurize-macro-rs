
#[macro_use]
extern crate closurize;


#[test]
fn fn_once() {

    #[closurize]
    trait Add<A, B> {
        fn add(self, a: A, b: B) -> f64;
    }

    fn accept_closure(closure: impl Add<f32, u32>) {
        assert_eq!(closure.add(32.0, 2), 34.0);
    }

    accept_closure(|a, b| a as f64 + b as f64)
}

#[test]
fn fn_mut() {

    #[closurize]
    trait Add<A, B> {
        fn add(&mut self, a: A, b: B) -> f64;
    }

    fn accept_closure(mut closure: impl Add<f32, u32>) {
        assert_eq!(closure.add(32.0, 2), 34.0);
    }

    let mut did_it = false;

    accept_closure(|a, b| {
        did_it = true;
        a as f64 + b as f64
    });

    assert!(did_it);
}
