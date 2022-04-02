use clap::Parser;

#[derive(Parser)]
#[clap(name="CryptoPrice")]
#[clap(author="Chai Phonbopit")]
#[clap(version="1.0.0")]
#[clap(about = "CLI to get crypto price from ftx spot market.")]
struct Args {
    // Name of the market.
    #[clap(short, long, default_value="market")]
    name: String
}

#[allow(unused)]
fn fetch() {
    // TODO: // fetch data from api.
}

fn main() {
    let args = Args::parse();

    println!("Hello, world! {:?}", args.name);
}
