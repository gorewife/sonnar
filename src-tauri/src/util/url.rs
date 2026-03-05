use thiserror::Error;

pub type ReqwestUrl = reqwest::Url;

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

#[derive(Error, Debug)]
#[error("Invalid Url: {0}")]
pub struct ParseError(String);

pub trait UrlParser<T: Url> {
    fn parse(url: String) -> Result<T, ParseError>;
}

pub struct ReqwestParser;

impl UrlParser<ReqwestUrl> for ReqwestParser {
    fn parse(url: String) -> Result<ReqwestUrl, ParseError> {
        match ReqwestUrl::parse(url.as_str()) {
            Ok(valid) => Ok(valid),
            Err(err) => Err(ParseError(err.to_string()))
        }
    }
}
