use std::fmt::{Debug, Display};

#[macro_use]
extern crate closurize;

#[test]
fn fn_once_generic() {

    #[closurize]
    trait Cloner<T>: Clone where T: Clone + Debug {
        fn clone_pair(self, a: &T) -> T where T: Display;
    }

    fn accept_closure(closure: impl Cloner<String>) {
        let original = String::from("string");
        let cloned = closure.clone().clone_pair(&original);
        assert_eq!(original, cloned);
    }

    accept_closure(|value: &String| value.clone())
}

/*#[test]
fn fn_once_generic() {

    #[closurize]
    trait Cloner<T>: Clone where T: Clone + Debug {
        fn clone_pair<F: Copy>(self, a: &T, b: &F) -> (T, F) where F: Display;
    }

    fn accept_closure(closure: impl Cloner<String>) {
        let original = (String::from("string"), 34.0);
        let cloned = closure.clone().clone_pair(&original.0, &original.1);
        assert_eq!(original, cloned);
    }

    accept_closure(|value: &String, fun| (value.clone(), fun))
}*/
