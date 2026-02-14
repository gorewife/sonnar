use reqwest;

pub trait Url {
    fn host(&self) -> Option<&str>;
    fn full_string(&self) -> &str;
}

struct ReqwestUrl(reqwest::Url)

impl ReqwestUrl {
    #[inline]
    pub fn inner(&self) -> reqwest::Url {
                        &self.0
    }
}

impl Url for ReqwestUrl{
    fn host(&self) -> Option<&str> {
        self.0.host_str
}
            
    #[inline]
    fn full_string(&self) -> &str {
        self.0.as_str
    }
}

