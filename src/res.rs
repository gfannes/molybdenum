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

pub fn print_error(error: Error) {
    println!("{}", error);
}
