use crate::{ConcreteService, CsesApi, Filesystem, Service, Storage};

struct FakeCsesApi {}

impl CsesApi for FakeCsesApi {}

struct FakeStorage {}

impl Storage for FakeStorage {}

struct FakeFilesystem {}

impl Filesystem for FakeFilesystem {}

fn fake_apis() -> (FakeCsesApi, FakeStorage, FakeFilesystem) {
    (FakeCsesApi {}, FakeStorage {}, FakeFilesystem {})
}

fn construct_service(apis: &mut (FakeCsesApi, FakeStorage, FakeFilesystem)) -> impl Service + '_ {
    ConcreteService::with_apis(&mut apis.0, &mut apis.1, &mut apis.2)
}

#[test]
fn ping_works() {
    let mut apis = fake_apis();
    let mut service = construct_service(&mut apis);
    // assert!(service.ping());
}
