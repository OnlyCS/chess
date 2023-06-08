use clap::Parser;

#[derive(Parser)]
#[command(
    name = "chess",
    author = "OnlyCS",
    version,
    about = "Chess TUI and grbl controller"
)]
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

    #[arg(
        short = 'm',
        long = "manual",
        help = "manually send up-down-left-right commands",
        default_value_t = false
    )]
    pub manual: bool,
}
