use rand::RngExt;
use std::ops::RangeInclusive;
use lunar_lander::{Outcome, Lander, MAX_THRUST};


// --- AUTOMATED SIMULATIONS ---

struct TrajectoryResult {
    /// Stores tuples of (Burn Rate Applied, Resulting Altitude)
    path: Vec<(u8, f64)>,
    outcome: Outcome,
    final_speed: f64,
    fuel_remaining: f64,
}

const SEQUENCE_LENGTH: usize = 30; // Max expected seconds of flight
const POPULATION_SIZE: usize = 1000;
const GENERATIONS: usize = 50;

// Evaluates a specific sequence of burns and returns the result
fn simulate_genome(genome: &[u8]) -> TrajectoryResult {
    let mut lander = Lander::new();
    let mut path = Vec::new();
    let mut time = 0;

    while !lander.is_landed() {
        // If the genome runs out before we land, the engines shut off (burn 0)
        let intended_burn = *genome.get(time).unwrap_or(&0);
        let max_burn = MAX_THRUST.min(lander.fuel as u8);
        let actual_burn = intended_burn.min(max_burn);

        lander.step(actual_burn);
        path.push((actual_burn, lander.altitude));
        time += 1;
    }
    
    let final_speed = lander.impact_velocity.expect("No impact velocity");
    let outcome = lander.get_outcome().unwrap();

    TrajectoryResult {
        path,
        outcome,
        final_speed,
        fuel_remaining: lander.fuel, 
    }
}

// The Fitness Function: Higher is better
fn calculate_fitness(result: &TrajectoryResult) -> f64 {
    match result.outcome {
        // Massive reward for perfection, plus points for leftover fuel
        Outcome::Perfect => 10000.0 + result.fuel_remaining,
        
        // Base points for surviving, but penalized for how close to 2.0 ft/s it was
        Outcome::Hard => 5000.0 - result.final_speed,
        
        // Crashes are penalized heavily based on the impact speed.
        // A softer crash scores higher than a meteor strike.
        Outcome::Crashed => {
            let penalty = result.final_speed.abs();
            // Using max to prevent negative fitness scores
            1000.0 - penalty.min(1000.0) 
        }
    }
}

fn run_evolution() {
    println!("\n--- INITIATING EVOLUTIONARY AUTOPILOT ---");
    let mut rng = rand::rng();

    // 1. Initialize random population
    let mut population: Vec<Vec<u8>> = (0..POPULATION_SIZE)
        .map(|_| (0..SEQUENCE_LENGTH).map(|_| rng.random_range(0..=MAX_THRUST)).collect())
        .collect();

    for generation in 1..=GENERATIONS {
        // 2. Evaluate Fitness
        let mut scored_population: Vec<(f64, Vec<u8>, TrajectoryResult)> = population
            .into_iter()
            .map(|genome| {
                let result = simulate_genome(&genome);
                let fitness = calculate_fitness(&result);
                (fitness, genome, result)
            })
            .collect();

        // Sort descending by fitness (highest score first)
        scored_population.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let best_fitness = scored_population[0].0;
        let best_speed = scored_population[0].2.final_speed;
        let best_outcome = &scored_population[0].2.outcome;

        if generation % 5 == 0 || generation == 1 {
            println!(
                "Gen {:02} | Best Fitness: {:.1} | Impact Speed: {:.2} ft/s | {:?}", 
                generation, best_fitness, best_speed, best_outcome
            );
        }

        // If we found a perfect landing, we can stop early!
        if *best_outcome == Outcome::Perfect {
            println!("\nSUCCESS! Perfect landing sequence evolved at Generation {generation}!");
            print_winning_trajectory(&scored_population[0].2);
            return;
        }

        // 3. Selection: Keep the top 10% (Elitism)
        let elite_count = POPULATION_SIZE / 10;
        let mut next_generation: Vec<Vec<u8>> = scored_population
            .iter()
            .take(elite_count)
            .map(|(_, genome, _)| genome.clone())
            .collect();

        // 4. Mutation: Fill the rest of the population by mutating the elites
        while next_generation.len() < POPULATION_SIZE {
            // Pick a random elite parent
            let parent_idx = rng.random_range(0..elite_count);
            let mut child = scored_population[parent_idx].1.clone();

            // Mutate: Randomly change ~15% of the sequence
            for gene in child.iter_mut() {
                if rng.random_bool(0.15) {
                    *gene = rng.random_range(0..=MAX_THRUST);
                }
            }
            next_generation.push(child);
        }

        population = next_generation;
    }

    println!("\nEvolution finished, but no perfect landing was found.");
}

fn print_winning_trajectory(result: &TrajectoryResult) {
    println!("Final Speed: {:.2} ft/s", result.final_speed);
    println!("Fuel Remaining: {:.1}", result.fuel_remaining);
    
    let formatted_path: Vec<String> = result.path
        .iter()
        .map(|(burn, alt)| format!("(Burn: {burn}, Alt: {alt:.0}ft)"))
        .collect();
        
    println!("Flight Path:\n  {}\n", formatted_path.join(" -> "));
}

fn main() {
    println!("{:>57}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY\n\n\n");
    println!("LUNAR LANDING SIMULATION");
    println!("----- ------- ----------\n");

    //if ask_yes_no("DO YOU WANT TO RUN AUTOMATED RANDOM SIMULATIONS FIRST?") {
    //    //let count = get_int::<usize>("HOW MANY SIMULATIONS", 1..=1000000);
    //}
    run_evolution();
}

