use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Command-line interface for BMI"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    BMI {
        weight: f64,
        height: f64,
    },
}

//invoke lib.rs using onnx_demo namespace
//use onnx_demo::run;
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::BMI{weight, height}) => {
            let result = weight/(height*height);
            println!("Your BMI is {}.", result);
            if result <= 18.5{
                println!("{}", "You are underweight.");
            }else if (result <= 24.9) & (result > 18.5){
                println!("{}", "You have normal weight.");
            }
            else if (result > 24.9) & (result <= 29.9){
                println!("{}", "You are overweight.");
            }else{
                println!("{}", "You are obese.");
            }
        }
        None => println!("No subcommand was used"),
    }
}