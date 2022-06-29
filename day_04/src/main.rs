mod read;
mod lib;

use lib::{Log, Scheduler};

fn main() {

    let logs_string = read::read_lines("input.txt".to_owned());
    let logs : Vec<Log> = logs_string.into_iter()
        .map(|log_string| Log::new(log_string)).collect();

    let mut scheduler : Scheduler = Scheduler::new(logs);
    scheduler.process_log_guards();

    // Part 1
    let guard_most_sleepy = scheduler.get_most_sleepy_guard().unwrap();
    let guard_most_probable = scheduler.get_most_probable_sleep_time(guard_most_sleepy).unwrap();
    println!("\r👮 Code according to Strategy 1: '{}' x '{}' = '{}' (Part 1)", guard_most_sleepy, guard_most_probable, guard_most_sleepy * guard_most_probable);
    
    // Part 2
    let (guard_most_sleepy, guard_most_probable) = scheduler.get_most_probable_sleep_time_all().unwrap();
    println!("\r👮 Code according to Strategy 2: '{}' x '{}' = '{}' (Part 2)", guard_most_sleepy, guard_most_probable, guard_most_sleepy * guard_most_probable);
}