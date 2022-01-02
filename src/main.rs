use structopt::StructOpt;
use cpu_scheduling_utils::cli::operations::Operations;
use cpu_scheduling_utils::config::parser::Config;
use cpu_scheduling_utils::schedule::scheduler::{Process, Timestamp};

fn main() {
    let ops: Operations = Operations::from_args();

    let conf = Config::from(ops.config.expect("Could not read from path")).unwrap();

    let processes = Process::from(&conf);

    let timestamps = Timestamp::highest_response_ratio_next(&processes);

    for timestamp in timestamps {
        print!("{:?}", timestamp.process.name);
        println!(" {}", timestamp.time);
    }
}
