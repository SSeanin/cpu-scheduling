use std::fs::File;
use std::io::{Result, BufReader, BufRead};
use std::path::{Path, PathBuf};

fn read_lines(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(filename).expect("No Such file.");
    let lines =
        BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
    Ok(lines)
}

#[derive(Debug)]
pub struct Config {
    pub pr_count: i32,
    pub service_times: Vec<i32>,
    pub arrival_times: Vec<i64>,
    pub rr_quantum: i32,
    pub multi_level_rr_quantum: i32,
}

impl Config {
    pub fn from(config_file: PathBuf) -> Result<Config> {
        let mut config = Config {
            pr_count: 0,
            service_times: vec![],
            arrival_times: vec![],
            rr_quantum: 0,
            multi_level_rr_quantum: 0,
        };

        if let Ok(lines) = read_lines(config_file) {
            config.pr_count = lines[0].parse::<i32>().expect("Could not parse pr_count");
            config.service_times = lines[1]
                .split(",")
                .map(|s| s.trim().parse::<i32>().expect("Could not parse service_times"))
                .collect();
            config.arrival_times = lines[2]
                .split(",")
                .map(|s| s.trim().parse::<i64>().expect("Could not parse arrival_times"))
                .collect();
            config.rr_quantum = lines[3].parse::<i32>().expect("Could not parse rr_quantum");
            config.multi_level_rr_quantum = lines[4]
                .parse::<i32>()
                .expect("Could not parse multi_level_rr_quantum");
        }

        Ok(config)
    }
}
