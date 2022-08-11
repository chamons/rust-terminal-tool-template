use tracing::log;

pub mod utils;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct ToolArgs {
    /// Print additional information (pass argument one to four times for increasing detail)
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[tracing::instrument]
pub fn start(args: ToolArgs) {
    println!("Hello, world!");

    log::info!("an example trace log");
    log::warn!("Warning");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_start() {
        assert_eq!(1, 1);
    }
}
