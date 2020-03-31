
#[macro_use]
extern crate closurize;


#[test]
fn fn_once() {

    #[closurize]
    trait Producer {  // TODO test async, once stabilized
        unsafe fn produce(&self) -> &'static str;
    }

    fn accept_closure(closure: impl Producer) {
        unsafe { closure.produce() };
    }

    accept_closure(|| "hello world")
}
