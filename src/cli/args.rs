use clap::Parser;

#[derive(Parser)]
#[command(author = "OnlyCS", version, about = "Chess GUI and AI")]
pub struct Cli {
    #[arg(
        short = 's',
        long = "serial",
        help = "Communicate with a Serial Port (arduino)",
        default_value_t = false
    )]
    pub serial: bool,
}
