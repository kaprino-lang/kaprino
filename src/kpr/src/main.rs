#[macro_use]
extern crate clap;
extern crate kgen;

pub mod kprc;
pub mod args_manager;

use kprc::KprcApp;

fn process() -> Result<(), String> {
    let matches = args_manager::get_args();

    if let Some(matches) = matches.subcommand_matches("compile") {
        let instance = KprcApp::new(matches)?;
        instance.execute()?;

        Ok(())
    }
    else {
        Err("No subcommand given.".to_string())
    }
}

fn main() {
    match process() {
        Ok(_) => {
            println!("Tasks completed! (^^)/");
        },
        Err(message) => {
            println!("error: {}", message);
        }
    };
}
