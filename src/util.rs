pub type Error = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct MyError {
    descr: String,

}
impl std::error::Error for MyError { }
impl std::fmt::Display for MyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "MyError: {}", self.descr)?;
        Ok(())
    }
}
impl MyError {
    pub fn new(descr: &str) -> MyError {
        let er = MyError{descr: descr.to_string()};
        er
    }
    pub fn create(descr: &str) -> Error {
        Error::from(MyError::new(descr))
    }
}

macro_rules! fail {
    ($fmt:expr) => {
        return Err(MyError::create(&format!($fmt)))
    };
    ($fmt:expr, $($arg:expr),*) => {
        return Err(MyError::create(&format!($fmt, $($arg),*)))
    };
    ($fmt:expr, $($arg:expr),+ ,) => {
        fail!($fmt, $($arg),*)
    };
}

pub type Range = std::ops::Range<usize>;

#[test]
fn test_create_custom_error() {
    let my_err = MyError::create("My custom error message");
    let err: &Error = &my_err;
    assert!(err.is::<MyError>());
    let my_err_ref = err.downcast_ref::<MyError>().unwrap();
    assert_eq!(my_err_ref.descr, "My custom error message");

    assert_eq!(format!("{}", my_err_ref), "MyError: My custom error message");
}

#[test]
fn test_fail_macro() {
    type R = Result<()>;
    type Ftor = Box<dyn Fn() -> R>;

    let ftors = [
        Box::new(||fail!("arg 0, trailing comma N")) as Ftor,
        Box::new(||fail!("arg 0, trailing comma Y",)),
        Box::new(||fail!("arg 1, trailing comma N {}",42)),
        Box::new(||fail!("arg 1, trailing comma Y {}",42,)),
        Box::new(||fail!("arg 2, trailing comma N {} {}",42, 43)),
        Box::new(||fail!("arg 2, trailing comma Y {} {}",42, 43, )),
    ];

    for ftor in &ftors {
        let res = ftor();
        assert!(res.is_err())
    }
}
