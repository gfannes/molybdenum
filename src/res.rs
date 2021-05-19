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

#[test]
fn create_custom_error() {
    let my_err = MyError::create("My custom error message");
    let err: &Error = &my_err;
    assert!(err.is::<MyError>());
    let my_err_ref = err.downcast_ref::<MyError>().unwrap();
    assert_eq!(my_err_ref.descr, "My custom error message");

    assert_eq!(format!("{}", my_err_ref), "MyError: My custom error message");
}
