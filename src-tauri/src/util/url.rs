pub trait Url {
    fn origin(&self) -> &str;
    fn host(&self) -> Option<&str>;
    fn full_string(&self) -> &str;
}