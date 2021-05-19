use thiserror::Error;

pub struct CsesHttpApi {}

impl CsesHttpApi {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CsesHttpApi {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ApiError {}

pub type ApiResult<T> = Result<T, ApiError>;

pub trait CsesApi {}

impl CsesApi for CsesHttpApi {}
