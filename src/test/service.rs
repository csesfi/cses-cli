use crate::{CsesApi, Filesystem, Resources, Storage};

struct FakeCsesApi {}

impl CsesApi for FakeCsesApi {}

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
    let mut resources = fake_resources();
    // assert!(service.ping());
}
