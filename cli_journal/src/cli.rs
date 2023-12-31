use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        task: String
    },
    Done {
        #[structopt()]
        position: usize
    },
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "A cli journal",
    about = "a command line to-do app written in rust"
)]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    journal_file: Option<PathBuf>,
}

