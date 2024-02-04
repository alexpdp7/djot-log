use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let source = std::fs::read_to_string("example.md")?;
    let (logs, errors) = djot_log::parse_log(&source);
    log::error!("{:?}", errors);
    println!("{}", logs.to_plain_text());
    for (date, total, vs_target) in logs.accumulated_vs_target(chrono::Duration::hours(8)) {
        println!("{} {} {}", date, total, vs_target);
    }

    Ok(())
}
