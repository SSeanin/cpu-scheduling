use structopt::StructOpt;
use cpu_scheduling_utils::cli::operations::Operations;

fn main() {
    let args = Operations::from_args();
    println!("{:?}", args);
}
