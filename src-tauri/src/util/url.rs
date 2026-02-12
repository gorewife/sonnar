pub trait Url {
    fn host(&self) -> Option<&str>;
    fn full_string(&self) -> &str;
}
