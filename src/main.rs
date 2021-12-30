use structopt::StructOpt;
use cpu_scheduling_utils::cli::operations::Operations;
use cpu_scheduling_utils::config::Config;

fn main() {
    let ops: Operations = Operations::from_args();

    let conf = Config::from(ops.config.expect("Could not read from path")).unwrap();

    println!("{:?}", conf);
}
