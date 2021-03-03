extern crate clap;
extern crate kgen;
extern crate kpr;

use clap::App;
use kgen::error::error_token::ErrorToken;
use kpr::kprc::KprcApp;

fn process() -> Result<(), Vec<ErrorToken>> {
    let mut app = App::new("Kaprino Compiler (kprc)");
    app = KprcApp::generate_args(app);
    let matches = app.get_matches();

    let instance = KprcApp::new(&matches)?;
    instance.execute()?;

    Ok(())
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
