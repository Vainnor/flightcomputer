# What is an E6B?
A flight calculator able to calculate:
- Time
- Speed
- Distance
- Fuel Consumption
- Radius of action?? What is this
  - How far an aircraft can fly. Just takes in wind, fuel, fuel burn, alt, etc and spits out how far it can go in all directions
- Groundspeed
- wind correction angle
- true airspeed
- density altitude
- Conversions

# MVP 
- CLI tool that can calculate Time, speed, distance, fuel consumption and a few other metrics

# Rust specifics
- Clap for CLI specifics

# CLI functionality

```bash
flightcomputer dur <dist> <gs> -> Time 0.5 HR(s)
```

# What to do for day 2?

const AVGAS_WEIGHT: f32 = 6.0;
const JETA_WEIGHT: f32 = 6.7;

add the ability to custiomize these values ^
implement wind calculations?
