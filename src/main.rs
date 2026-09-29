use clap::{Parser, Subcommand, ValueEnum};

use crate::GasType::{AvGas, JetA};

const AVGAS_WEIGHT: f32 = 6.0;
const JETA_WEIGHT: f32 = 6.7;

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

#[derive(ValueEnum, Clone)]
enum GasType {
    AvGas,
    JetA,
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
    /// Calculate fuel burn given duration, fuel burn rate, and gas type
    Fuel {
        ///Duration
        duration: f32,

        /// Fuel Per Hour
        fuel_per_hour: f32,
        gas_type: GasType,
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
        Commands::Fuel {
            duration,
            fuel_per_hour,
            gas_type,
        } => calculate_fuel_burn(duration, fuel_per_hour, gas_type),
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

fn calculate_fuel_burn(duration: f32, fuel_per_hour: f32, gas_type: GasType) {
    let gas_weight: f32;

    match gas_type {
        AvGas => gas_weight = AVGAS_WEIGHT,
        JetA => gas_weight = JETA_WEIGHT,
    }

    let gal: f32 = duration * fuel_per_hour;
    let lbs = gal * gas_weight;

    print!(
        "Fuel Burn: {} GAL - {} LBS @{} LBS/GAL ",
        gal, lbs, gas_weight
    )
}
