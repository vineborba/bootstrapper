mod options;
mod package_managers;
mod techs;
mod ui;

use anyhow::{bail, Result};
use std::{collections::HashSet, fs};

pub use options::Options;
pub use package_managers::PackageManagers;
pub use techs::Techs;

pub fn run(opts: Options) -> Result<()> {
    let Options {
        mut project_prefix,
        package_manager,
        mut techs,
    } = opts;

    if project_prefix.is_none() {
        project_prefix = Some(ui::render_project_naming());
    }

    if techs.is_empty() {
        techs = ui::render_tech_selection()?;
    }

    let mut possible_pkg_managers: HashSet<String> = HashSet::new();

    for i in 0..techs.len() {
        let t = techs.get(i).unwrap();
        for pkg_mngrs in t.get_package_managers().into_iter() {
            possible_pkg_managers.insert(pkg_mngrs.executable_name());
        }
    }

    let package_manager = if let Some(package_manager) = package_manager {
        package_manager.executable_name()
    } else {
        ui::render_package_manager_selection()?.executable_name()
    };

    if !possible_pkg_managers.contains(&package_manager) {
        bail!("Invalid package manager!");
    }

    let project_prefix = project_prefix.unwrap();
    fs::create_dir(&project_prefix)?;

    for t in techs {
        match t.bootstrap_project(&project_prefix, &package_manager) {
            Ok(_) => {
                println!("Successfully bootstrapped {t} project!");
            }
            Err(err) => {
                eprintln!("Failed to bootstrap {t}. Error: {err}");
            }
        }
    }

    Ok(())
}
