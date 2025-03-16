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

    println!("Balance:");
    println!();

    let total_by_day = djot_log::total_by_day(logs.iter());
    let total_by_day_with_running = djot_log::add_running_total(total_by_day.iter());
    let target = djot_log::target(chrono::TimeDelta::try_hours(args.hours_target).unwrap());
    let total_by_day_vs_target =
        djot_log::running_total_vs_target(total_by_day_with_running, target).collect::<Vec<_>>();
    for (i, (date, total, vs_target)) in total_by_day_vs_target.iter().rev().enumerate() {
        let total = total.num_minutes();
        let (h, m) = (total / 60, total % 60);
        println!(
            "day: {} {}h {}m, delta minutes {}",
            date,
            h,
            m,
            vs_target.num_minutes()
        );
        if *vs_target == chrono::TimeDelta::zero() && i != 0 {
            break;
        }
    }

    let show = args.show.map_or(chrono::Local::now().date_naive(), |s| {
        chrono::NaiveDate::parse_from_str(s.as_ref(), "%Y-%m-%d").expect("Unparseable show date")
    });

    println!();
    println!("Logs for {show}:");
    println!();

    for log in logs.iter().filter(|l| l.start.date() == show) {
        println!("{log}");
    }

    Ok(())
}
