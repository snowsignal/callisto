use clap::Parser;

mod amulet;
mod engine;
use engine::Engine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut engine = Engine::init();
    engine.execute(args)
}
