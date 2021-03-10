use std::fs;
use std::path::PathBuf;
use std::process::Command;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use inkwell::context::Context;
use kgen::ast::CodeGen;
use kgen::error::error_token::ErrorToken;

pub struct KprcApp {
    source_file: String,
    output_file: Option<String>,
    llvm: bool
}

impl KprcApp {
    pub fn new(args: &ArgMatches) -> Result<Self, Vec<ErrorToken>> {
        let source_file = args.value_of("SOURCES");
        let output_file = match args.value_of("output") {
            Some(val) => Some(val.to_string()),
            None => None
        };
        let llvm = args.is_present("llvm-only");

        if let Some(source_file) = source_file {
            let source_file = source_file.to_string();

            Ok(Self {
                source_file, output_file, llvm
            })
        }
        else {
            Err(
                vec![ErrorToken::fatal_error("No source files given.".to_string())]
            )
        }
    }

    pub fn generate_args<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.version(crate_version!())
            .author("Akihisa Yagi <capra314cabra@gmail.com>")
            .about("Compiles source files given.")
            .arg(Arg::with_name("SOURCES")
                .help("Source files"))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .takes_value(true)
                .help("Output file name"))
            .arg(Arg::with_name("llvm-only")
                .long("llvm-only")
                .help("Emits LLVM IR files only")
            )
    }

    fn get_source_content(&self) -> Result<String, Vec<ErrorToken>> {
        match fs::read_to_string(&self.source_file) {
            Ok(content) => Ok(content),
            Err(_) => Err(
                vec![ErrorToken::fatal_error(format!("Cannot read a source file named \"{}.\"", &self.source_file))]
            )
        }
    }

    fn get_output_path(&self) -> String {
        if let Some(filename) = &self.output_file {
            filename.clone()
        }
        else {
            let mut path_buf = PathBuf::from(&self.source_file);
            path_buf.set_extension("");
            path_buf.into_os_string().into_string().unwrap()
        }
    }

    fn write_llvm_ir(&self, gen: &CodeGen, output_path: &str) -> Result<(), Vec<ErrorToken>> {
        let result = gen.module.print_to_file(format!("{}.ll", output_path));
        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                Err(
                    vec![ErrorToken::fatal_error(error.to_string())]
                )
            }
        }
    }

    fn compile_by_clang(&self, output_path: &str) -> Result<(), Vec<ErrorToken>> {
        let mut command = Command::new("clang");
        command.arg(format!("{}.ll", output_path));
        command.arg("-o");
        command.arg(output_path);

        match command.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => {
                        if status.success() {
                            Ok(())
                        }
                        else {
                            Err(
                                vec![ErrorToken::fatal_error("Clang was finished unexpectedly.".to_string())]
                            )
                        }
                    },
                    Err(_) => {
                        Err(
                            vec![ErrorToken::fatal_error("Clang was finished unexpectedly.".to_string())]
                        )
                    }
                }
            },
            Err(_) => {
                Err(
                    vec![ErrorToken::fatal_error("Failed to launch Clang. Have you installed Clang?".to_string())]
                )
            }
        }
    }

    pub fn execute(&self) -> Result<(), Vec<ErrorToken>> {
        let context = &Context::create();

        let gen = CodeGen::new(context, &self.source_file);

        println!("log: Loading \"{}\"...", self.source_file);

        let content = self.get_source_content()?;
        let output_path = self.get_output_path();

        println!("log: Parsing \"{}\"...", self.source_file);
        gen.codegen(content.as_str())?;

        println!("log: Generating \"{}\"...", output_path);
        self.write_llvm_ir(&gen, &output_path)?;

        if self.llvm {
            Ok(())
        }
        else {
            println!("log: Building intermediate files by Clang...");
            self.compile_by_clang(&output_path)?;

            Ok(())
        }
    }
}
