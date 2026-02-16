use reqwest::Url as ReqwestUrl;

pub trait Url {
    fn host(&self) -> Option<&str>;
    fn full_string(&self) -> &str;
}

impl Url for ReqwestUrl{
    fn host(&self) -> Option<&str> {
        self.host_str()
}
            
    #[inline]
    fn full_string(&self) -> &str {
        self.as_str()
    }
}

pub trait UrlParser<T: Url> {
    fn parse(url: String) -> T;
}
