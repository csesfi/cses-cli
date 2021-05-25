use crate::{api::ApiResult, service::Login};
use crate::{CsesApi, Filesystem, Resources, Storage};

struct FakeCsesApi {}

impl CsesApi for FakeCsesApi {
    fn login(_login: &Login) -> ApiResult<String> {
        todo!()
    }
}

struct FakeStorage {}

impl Storage for FakeStorage {}

struct FakeFilesystem {}

impl Filesystem for FakeFilesystem {}

fn fake_resources() -> Resources<(FakeCsesApi, FakeStorage, FakeFilesystem)> {
    Resources {
        api: FakeCsesApi {},
        storage: FakeStorage {},
        filesystem: FakeFilesystem {},
    }
}

#[test]
fn ping_works() {
    let mut _resources = fake_resources();
    // assert!(service.ping());
}
