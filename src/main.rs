use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let source = std::fs::read_to_string("example.md")?;
    let (logs, errors) = djot_log::parse_log(&source);
    let mut logs = logs.iter().collect::<Vec<_>>();
    logs.sort_by_key(|l| l.start);
    log::debug!(
        "{}",
        logs.iter()
            .map(|l| format!("{}", l))
            .collect::<Vec<_>>()
            .join("\n")
    );
    log::error!("{:?}", errors);
    Ok(())
}
