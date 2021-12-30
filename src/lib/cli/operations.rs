use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Schedule", about = "Schedule cpu tasks", author)]
pub struct Operations {
    #[structopt(short = "c", long = "config", help = "Path to config file", parse(from_os_str))]
    config: Option<PathBuf>,
}
