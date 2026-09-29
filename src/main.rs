use clap::{Parser, Subcommand};

/// A powerful file processing tool
#[derive(Parser)]
#[command(name = "flightcomputer")]
#[command(author = "Vainnor")]
#[command(version = "0.1.0")]
#[command(about = "COMPUTE FLIGHT THINGS!", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate time for a given distance at a given ground speed
    Duration {
        /// Total Distance
        distance: f32,

        /// Ground Speed
        ground_speed: f32,
    },
    /// Calculate ground speed when given distance and time
    Speed {
        /// Total Distance
        distance: f32,

        /// Time
        time: f32,
    },
    /// Calculate distance given ground speed and time
    Distance {
        /// Ground Speed
        ground_speed: f32,

        /// Time
        time: f32,
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
        Commands::Speed { distance, time } => calculate_ground_speed(distance, time),
        Commands::Distance { ground_speed, time } => calculate_distance(ground_speed, time),
    }
}

fn calculate_duration(distance: f32, ground_speed: f32) {
    if ground_speed == 0 as f32 {
        panic!("You won't go anywhere at 0 Knots!")
    } else {
        println!("Duration: {} HR(s)", distance / ground_speed)
    }
}

fn calculate_ground_speed(distance: f32, time: f32) {
    if time == 0 as f32 {
        panic!("You cannot go somewhere in no time")
    } else {
        println!("Speed: {} KT(s) (GS)", distance / time)
    }
}

fn calculate_distance(ground_speed: f32, time: f32) {
    println!("Distance: {} NM", ground_speed * time)
}
