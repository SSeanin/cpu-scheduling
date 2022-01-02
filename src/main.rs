use structopt::StructOpt;
use cpu_scheduling_utils::cli::operations::Operations;
use cpu_scheduling_utils::config::parser::Config;
use cpu_scheduling_utils::schedule::scheduler::{Process, Timestamp, Scheduler};

fn main() {
    let ops: Operations = Operations::from_args();

    let conf = Config::from(ops.config.expect("Could not read from path")).unwrap();

    let processes = Process::from(&conf);

    let mut scheduler = Scheduler {
        time: 0,
        timestamps: vec![],
    };

    scheduler.multi_level_feedback_queue(&processes, conf.rr_quantum, conf.multi_level_rr_quantum);

    for timestamp in scheduler.timestamps {
        print!("{:?}", timestamp.process.name);
        println!(" {}", timestamp.time);
    }
}
