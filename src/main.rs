
extern crate clap;
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::process::exit;

use clap::{Arg, App, ArgMatches, SubCommand};
use log::{info, trace, warn};


enum OperatingSystems {
    mac,
    linux,
}


struct ProgramData {
    name: &'static str,
    version: &'static str,
    authors: &'static str,
    description: &'static str,
}
impl ProgramData {
    fn from_env() -> ProgramData {
        ProgramData {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            authors: env!("CARGO_PKG_AUTHORS"),
            description: env!("CARGO_PKG_DESCRIPTION"),
        }
    }
}


struct ProgramOptions {
    command: String,
    verbosity: u8,
    force: bool,
}
impl ProgramOptions {
    fn from_arg_matches(arg_matches: ArgMatches) -> ProgramOptions {
        ProgramOptions {
            command: ProgramOptions::get_subcommand_or_exit(&arg_matches),
            verbosity: ProgramOptions::get_verbosity(&arg_matches),
            force: arg_matches.is_present("force"),
        }
    }

    fn get_subcommand_or_exit(arg_matches: &ArgMatches) -> String {
        match arg_matches.subcommand_name() {
            Some(s) => String::from(s),
            None => {
                eprintln!("You must specify a sub-command");
                exit(1);
            }
        }
    }

    fn get_verbosity(arg_matches: &ArgMatches) -> u8 {
        let occurrences = arg_matches.occurrences_of("verbose");
        if occurrences > 255 {
            eprintln!("We cannot be that verbose");
            exit(1)
        }
        occurrences as u8
    }
}

#[derive(Deserialize)]
struct VimConfig {
    source: String,
    vimrc: String,
    plugins: Vec<String>,
    ycm: bool,
}

#[derive(Deserialize)]
struct VSCodeConfig {
    source: String,
    plugins: Vec<String>,
}

#[derive(Deserialize)]
struct GitHubConfig {
    keyfile: String,
    repos: Vec<String>,
}

#[derive(Deserialize)]
struct MacConfig {
    brew_casks: Vec<String>,
    brew_packages: Vec<String>,
    brew_taps: Vec<String>,
}

#[derive(Deserialize)]
struct RustConfig {
    install: bool,
    components: Vec<String>,
}

#[derive(Deserialize)]
struct NodeConfig {
    install: bool,
    node_version: String,
    npm_packages: Vec<String>,
}

#[derive(Deserialize)]
struct PythonVenvConfig {
    name: String,
    version: String,
    packages: Vec<String>,
}

#[derive(Deserialize)]
struct PythonConfig {
    install: bool,
    venvs: Vec<PythonVenvConfig>,
    venv_dir: String,
}

#[derive(Deserialize)]
struct DotfileConfig {
    source: String,
    bash_profile: String,
    bashrc: String,
    aliases: String,
    globalrc: String,
}

#[derive(Deserialize)]
struct TmuxConfig {
    source: String,
    tmux_conf: String,
}

#[derive(Deserialize)]
struct ItermConfig {
    source: String,
    profiles: String,
}

#[derive(Deserialize)]
struct Config {
    dotfiles: DotfileConfig,
    mac: MacConfig,
    node: NodeConfig,
    python: PythonConfig,
    rust: RustConfig,
    tmux: TmuxConfig,
    vim: VimConfig,
    vscode: VSCodeConfig,
}

/// Create a new "force" ("-f") argument
///
/// This is a shared option for many of the sub-commands, so this helper
/// is here to generate an equivalent one for each.
fn new_force_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("force")
        .short("f")
        .help(
            "overwrite existing configuration files
            with the ones from this repo"
        )
}


fn args<'a, 'b>(program_data: &'a ProgramData) -> ArgMatches<'a> {
    App::new(program_data.name)
        .version(program_data.version)
        .author(program_data.authors)
        .about(program_data.description)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("increase verbosity")
                .global(true)
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .value_name("FILE")
                .takes_value(true)
                .help("use a custom config file")
                .global(true)
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("initialize the host")
                .arg(new_force_arg())
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("ensure the host is up to doate")
                .arg(new_force_arg())
        )
        .get_matches()
}


fn main() {
    let program_data = ProgramData::from_env();
    let arg_matches = args(&program_data);
    let opts = ProgramOptions::from_arg_matches(arg_matches);
}
