use clap::Parser;

#[derive(Parser)]
struct CLi {
    targets: Vec<String>,
    commit_message: String
}

fn main() {
    let args = CLi::parse();
    println!("{:?}", args.targets); 
}