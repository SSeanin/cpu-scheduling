use crate::config::parser::Config;

#[derive(Clone, Debug)]
pub struct Process {
    pub name: String,
    pub burst_time: i32,
    pub arrival_time: i64,
    pub waiting_time: i64,
}

#[derive(Debug)]
pub struct Timestamp {
    pub time: i64,
    pub process: Process,
}

impl Process {
    pub fn from(config: &Config) -> Vec<Process> {
        let mut processes = Vec::new();
        let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let mut i = 0;
        while i < config.service_times.len() {
            processes.push(Process {
                name: String::from(alphabet[i].to_string()),
                burst_time: config.service_times[i],
                arrival_time: config.arrival_times[i],
                waiting_time: 0,
            });
            i += 1;
        }
        processes
    }
}

impl Timestamp {
    pub fn shortest_remaining_time_next(processes: &Vec<Process>) -> Vec<Timestamp> {
        let mut timestamps = Vec::new();
        let mut time = 0;

        let mut processes = processes.clone();

        let mut queue = Vec::new();

        while !processes.is_empty() || !queue.is_empty() {
            loop {
                if !processes.is_empty() && processes[0].arrival_time <= time {
                    queue.push(processes.remove(0));
                } else {
                    break;
                }
            }

            if !queue.is_empty() {
                let mut shortest_burst_time = f32::INFINITY;
                let mut shortest_burst_time_index = 0;

                let mut i = 0;
                while i < queue.len() {
                    if (queue[i].burst_time as f32) < shortest_burst_time {
                        shortest_burst_time = queue[i].burst_time as f32;
                        shortest_burst_time_index = i;
                    }
                    i += 1;
                }

                queue[shortest_burst_time_index].burst_time -= 1;
                time += 1;
                timestamps.push(Timestamp {
                    time,
                    process: queue[shortest_burst_time_index].clone(),
                });

                if queue[shortest_burst_time_index].burst_time == 0 {
                    queue.remove(shortest_burst_time_index);
                }
            } else {
                time += 1;
            }
        }

        timestamps
    }

    pub fn highest_response_ratio_next(processes: &Vec<Process>) -> Vec<Timestamp> {
        let mut timestamps = Vec::new();
        let mut time = 0;

        let mut processes: Vec<Process> = processes.clone();

        while processes.len() > 0 {
            let mut highest_ratio = 0.0;
            let mut highest_ratio_index = 0;

            let mut i = 0;
            let mut process_in_queue = false;
            while i < processes.len() {
                if processes[i].arrival_time <= time {
                    process_in_queue = true;
                    let mut ratio = processes[i].waiting_time as f64 / processes[i].burst_time as f64;
                    if ratio.is_sign_positive() && ratio > highest_ratio {
                        highest_ratio = ratio;
                        highest_ratio_index = i;
                    }
                }
                i += 1;
            }

            if process_in_queue {
                let process = processes.remove(highest_ratio_index);
                time += process.burst_time as i64;
                timestamps.push(Timestamp {
                    time,
                    process,
                });
                process_in_queue = false;
                processes.iter_mut()
                    .for_each(|mut p|
                        p.waiting_time = if p.arrival_time <= time { time - p.arrival_time } else { 0 });
            } else {
                time += 1;
            }
        }

        timestamps
    }

    pub fn rr(processes: &Vec<Process>, quantum: i32) -> Vec<Timestamp> {
        let mut timestamps = Vec::new();
        let mut time = 0;

        let mut processes: Vec<Process> = processes.clone();
        processes.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));

        let mut queue: Vec<Process> = Vec::new();

        while !processes.is_empty() || !queue.is_empty() {
            loop {
                if !processes.is_empty() && processes[0].arrival_time <= time {
                    queue.push(processes.remove(0));
                } else {
                    break;
                }
            }

            if !queue.is_empty() {
                let mut process = queue.remove(0);

                if process.burst_time > quantum {
                    process.burst_time -= quantum;
                    time += quantum as i64;
                    timestamps.push(Timestamp {
                        time,
                        process: process.clone(),
                    });
                    loop {
                        if !processes.is_empty() && processes[0].arrival_time <= time {
                            queue.push(processes.remove(0));
                        } else {
                            break;
                        }
                    }
                    queue.push(process);
                } else {
                    time += process.burst_time as i64;
                    process.burst_time = 0;
                    timestamps.push(Timestamp {
                        time,
                        process,
                    });
                }
            } else {
                time += 1;
            }
        }

        timestamps
    }
}
