use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let source = std::fs::read_to_string("example.md")?;
    log::debug!("{:?}", djot_log::parse_markdown(&source));
    Ok(())
}
