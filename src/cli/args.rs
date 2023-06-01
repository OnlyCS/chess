use clap::Parser;

#[derive(Parser)]
#[command(author = "OnlyCS", version, about = "Chess GUI and AI")]
pub struct Cli {
    #[arg(
        short = 's',
        long = "serial",
        help = "Communicate with a Serial Port (arduino), requires hexapawn",
        default_value_t = false
    )]
    pub serial: bool,

    #[arg(
        short = 'x',
        long = "hexapawn",
        help = "Play Hexapawn instead of Chess",
        default_value_t = false
    )]
    pub hexapawn: bool,
}
