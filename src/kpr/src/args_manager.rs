use clap::App;
use clap::ArgMatches;
use clap::SubCommand;
use super::kprc::KprcApp;

pub fn get_args<'a>() -> ArgMatches<'a> {
    let mut app = App::new("Kaprino Processing Runner (kpr)")
        .version(crate_version!())
        .author("Akihisa Yagi <capra314cabra@gmail.com>")
        .about("This tool provides you many ways to control kaprino-lang.\nFor more information, please refer to https://github.com/kaprino-lang/kaprino.");

    let mut kprc_app = SubCommand::with_name("compile");
    kprc_app = KprcApp::generate_args(kprc_app);

    app = app.subcommand(kprc_app);

    app.get_matches()
}
