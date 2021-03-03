#[macro_use]
extern crate clap;
extern crate kgen;

pub mod kprc;
pub mod args_manager;

use kgen::error::error_token::ErrorToken;
use kprc::KprcApp;

fn process() -> Result<(), Vec<ErrorToken>> {
    let matches = args_manager::get_args();

    if let Some(matches) = matches.subcommand_matches("compile") {
        let instance = KprcApp::new(matches)?;
        instance.execute()?;

        Ok(())
    }
    else {
        Err(
            vec![ErrorToken::fatal_error("No subcommand given.".to_string())]
        )
    }
}

fn main() {
    match process() {
        Ok(_) => {
            println!("Tasks completed! (^^)/");
        },
        Err(messages) => {
            for message in messages {
                println!("{}", message);
            }
        }
    };
}
