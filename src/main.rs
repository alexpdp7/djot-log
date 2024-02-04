use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let source = std::fs::read_to_string("example.md")?;
    let (logs, errors) = djot_log::parse_log(&source);
    log::error!("{:?}", errors);
    println!("{}", logs.to_plain_text());
    let mut accumulated_vs_target = logs.accumulated_vs_target(chrono::Duration::hours(8));
    accumulated_vs_target.reverse();
    for (date, total, vs_target) in accumulated_vs_target {
        println!("{} {} {}", date, total, vs_target);
        if vs_target == chrono::Duration::zero() {
            break;
        }
    }

    Ok(())
}
