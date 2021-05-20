use crate::{ConcreteFilesystem, CsesHttpApi, FileStorage};
use crate::{CsesApi, Filesystem, Storage};

pub struct Resources<R: ResourcesProvider> {
    pub api: R::CsesApiImpl,
    pub storage: R::StorageImpl,
    pub filesystem: R::FilesystemImpl,
}

pub trait ResourcesProvider {
    type CsesApiImpl: CsesApi;
    type StorageImpl: Storage;
    type FilesystemImpl: Filesystem;
}

impl<A: CsesApi, S: Storage, F: Filesystem> ResourcesProvider for (A, S, F) {
    type CsesApiImpl = A;
    type StorageImpl = S;
    type FilesystemImpl = F;
}

pub type DefaultResources = (CsesHttpApi, FileStorage, ConcreteFilesystem);
