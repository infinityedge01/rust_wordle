use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specific the answer word
    #[arg(short, long)]
    word: Option<String>,

    /// Random answer
    #[arg(short, long)]
    random: bool,

    /// Difficult mode
    #[arg(short = 'D', long)]
    difficult: bool,

    /// Day number
    #[arg(short, long, default_value_t = 0)]
    day: u32,

    /// Randum seed
    #[arg(short, long, default_value_t = 0)]
    seed: u32,

    /// Final set file
    #[arg(short, long)]
    final_set: Option<String>,

    /// Acceptable set file
    #[arg(short, long)]
    acceptable_set: Option<String>,

    /// Game state file
    #[arg(short = 'S', long)]
    state: Option<String>,
    
    /// Config file
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();

}
