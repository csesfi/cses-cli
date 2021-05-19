use crate::{CsesApi, Filesystem, Storage};

#[allow(unused)] // FIXME
pub struct ConcreteService<'a, A, S, F> {
    api: &'a mut A,
    storage: &'a mut S,
    filesystem: &'a mut F,
}

impl<'a, A: CsesApi, S: Storage, F: Filesystem> ConcreteService<'a, A, S, F> {
    pub fn with_apis(api: &'a mut A, storage: &'a mut S, filesystem: &'a mut F) -> Self {
        Self {
            api,
            storage,
            filesystem,
        }
    }
}

pub trait Service {}

impl<'a, A: CsesApi, S: Storage, F: Filesystem> Service for ConcreteService<'a, A, S, F> {}
