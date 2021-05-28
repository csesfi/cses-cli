pub mod api;
pub use api::{CsesApi, CsesHttpApi};

pub mod storage;
pub use storage::{FileStorage, Storage};

pub mod filesystem;
pub use filesystem::{ConcreteFilesystem, Filesystem};

pub mod resources;
use resources::DefaultResources;
pub use resources::{Resources, ResourcesProvider};
pub use ResourcesProvider as RP;

pub mod service;

pub mod command;
pub use command::Command;

pub mod ui;
use ui::Ui;

pub mod entities;

#[cfg(test)]
mod test;

#[cfg(custom_abort)]
mod abort_handler;

fn main() -> anyhow::Result<()> {
    #[cfg(custom_abort)]
    abort_handler::setup();
    let command = Command::from_command_line()?;
    let api = CsesHttpApi::default();
    let storage = FileStorage::new();
    let filesystem = ConcreteFilesystem::default();
    let resources: Resources<DefaultResources> = Resources {
        api,
        storage,
        filesystem,
    };
    let mut ui = Ui::with_resources(resources);
    ui.run(command)?;
    Ok(())
}
