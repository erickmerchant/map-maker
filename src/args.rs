use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Args {
    #[arg(short = 'H', long, default_value_t = 10)]
    pub height: usize,

    #[arg(short = 'W', long, default_value_t = 10)]
    pub width: usize,

    #[arg(short = 'S', long, default_value_t = Args::get_default_seed())]
    pub seed: String,
}

impl Args {
    fn get_default_seed() -> String {
        "xo.~".to_string()
    }
}
