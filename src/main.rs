use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let source = std::fs::read_to_string("example.md")?;
    let (logs, errors) = djot_log::parse_log(&source);
    if !errors.is_empty() {
        log::error!("{:?}", errors);
    }
    for log in &logs {
        println!("{}", log);
    }
    let total_by_day = djot_log::total_by_day(logs.iter());
    let total_by_day_with_running = djot_log::add_running_total(total_by_day.iter());
    let target = djot_log::target(chrono::Duration::hours(8));
    let total_by_day_vs_target =
        djot_log::running_total_vs_target(total_by_day_with_running, target).collect::<Vec<_>>();
    for (date, total, vs_target) in total_by_day_vs_target.iter().rev() {
        println!("{} {} {}", date, total, vs_target);
        if *vs_target == chrono::Duration::zero() {
            break;
        }
    }

    Ok(())
}
