pub mod api;
pub use api::{CsesApi, CsesHttpApi};

pub mod storage;
pub use storage::{FileStorage, Storage};

pub mod filesystem;
pub use filesystem::{ConcreteFilesystem, Filesystem};

pub mod service;
pub use service::{ConcreteService, Service};

pub mod command;
pub use command::Command;

pub mod ui;
use ui::Ui;

#[cfg(test)]
mod test;

fn main() -> anyhow::Result<()> {
    let command = Command::from_command_line()?;
    let mut http_api = CsesHttpApi::default();
    let mut file_storage = FileStorage::default();
    let mut fs = ConcreteFilesystem::default();
    let service = ConcreteService::with_apis(&mut http_api, &mut file_storage, &mut fs);
    let mut ui = Ui::with_service(service);
    ui.run(command)?;
    Ok(())
}
