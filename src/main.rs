use clap::{Parser, Subcommand};

/// A powerful file processing tool
#[derive(Parser)]
#[command(name = "flightcomputer")]
#[command(author = "Vainnor")]
#[command(version = "0.1.0")]
#[command(about = "COMPUTE THINGS!", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate time for a given distance at a given speed
    Duration {
        /// Total Distance
        distance: f32,

        /// Ground Speed
        #[arg(value_name = "GROUND SPEED")]
        ground_speed: f32,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        eprintln!("Verbose mode enabled");
    }

    match cli.command {
        Commands::Duration {
            distance,
            ground_speed,
        } => calculate_duration(distance, ground_speed),
    }
}

fn calculate_duration(distance: f32, ground_speed: f32) {
    if ground_speed == 0 as f32 {
        panic!("You won't go anywhere at 0 Knots!")
    } else {
        println!("Duration: {} HR(s)", distance / ground_speed)
    }
}
