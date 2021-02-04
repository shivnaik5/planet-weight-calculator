use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    pub weight: f32,

    #[structopt(short, long)]
    pub planet: String,
}

pub fn get_args() -> Cli {
    Cli::from_args()
}
