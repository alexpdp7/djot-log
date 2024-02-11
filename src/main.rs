use std::error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: std::path::PathBuf,

    #[arg(long, default_value_t = 8)]
    hours_target: i64,

    /// Day to show logs for, defaults to today
    #[arg(long)]
    show: Option<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    env_logger::init();
    let source = std::fs::read_to_string(args.file)?;
    let (logs, errors) = djot_log::parse_log(&source);
    if !errors.is_empty() {
        log::error!("{:?}", errors);
    }

    let total_by_day = djot_log::total_by_day(logs.iter());
    let total_by_day_with_running = djot_log::add_running_total(total_by_day.iter());
    let target = djot_log::target(chrono::TimeDelta::hours(args.hours_target));
    let total_by_day_vs_target =
        djot_log::running_total_vs_target(total_by_day_with_running, target).collect::<Vec<_>>();
    for (date, total, vs_target) in total_by_day_vs_target.iter().rev() {
        println!(
            "day: {} {}h, delta minutes: {}",
            date,
            total.num_minutes() / 60,
            vs_target.num_minutes()
        );
        if *vs_target == chrono::TimeDelta::zero() {
            break;
        }
    }

    let show = args
        .show
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s.as_ref(), "%Y-%m-%d")
                .expect("Unparseable show date")
        })
        .unwrap_or(chrono::Local::now().date_naive());

    for log in logs.iter().filter(|l| l.start.date() == show) {
        println!("{}", log);
    }

    Ok(())
}
