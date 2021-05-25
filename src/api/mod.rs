use crate::service::Login;
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

pub trait CsesApi {
    fn login(login: &Login) -> ApiResult<String>;
}

impl CsesApi for CsesHttpApi {
    fn login(_login: &Login) -> ApiResult<String> {
        todo!()
    }
}
