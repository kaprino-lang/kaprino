extern crate clap;
extern crate kpr;

use clap::App;
use kpr::kprc::KprcApp;

fn process() -> Result<(), String> {
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
        Err(message) => {
            println!("error: {}", message);
        }
    };
}
