use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let source = std::fs::read_to_string("example.md")?;
    let (logs, errors) = djot_log::parse_log(&source);
    log::error!("{:?}", errors);
    println!("{}", logs.to_plain_text());
    for (date, duration, running_total) in
        djot_log::add_running_total(logs.total_by_day().iter(), chrono::Duration::zero())
            .collect::<Vec<_>>()
    {
        println!("{} {} {}", date, duration, running_total);
    }

    Ok(())
}
