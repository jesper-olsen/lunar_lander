# Lunar Lander: Manual & Evolutionary Autopilot

A Rust-based simulation of the classic 1970s Lunar Lander game, featuring both a playable interactive terminal game and an automated genetic algorithm that learns how to land the spacecraft perfectly.

This project is a tribute to David Ahl's original BASIC program published in *Creative Computing*, completely rewritten in Rust with a decoupled physics engine to support AI training.

## Features

* **Classic Interactive Gameplay:** Take manual control of the Apollo descent engine 1000 feet above the lunar surface. Balance your fuel and velocity to avoid a catastrophic crash.
* **Evolutionary Autopilot:** Watch a genetic algorithm train a population of flight plans. It scores trajectories based on impact velocity and fuel conservation, mutating the best survivors over generations until it discovers the mathematically perfect landing sequence.
* **Decoupled Physics Engine:** Core mechanics (gravity, thrust, fuel consumption, and fractional-second touchdown calculations) are isolated in a reusable library.

## Prequisites

Get [Rust](https://rust-lang.org/tools/install/)

## How to Run

### 1. Play the Game (Manual Mode)

To take manual control of the lander and try to beat gravity yourself:

```bash
cargo run --bin play
```

### 2. Train the Autopilot (Evolution Mode)
To spin up the genetic algorithm and watch it evolve a flawless flight sequence over multiple generations:

```bash
cargo run --bin evolve
```

## The Physics & Scoring

The simulation runs in discrete 1-second chunks ($DT = 1.0$), with a gravity of $5.0 \text{ ft/s}^2$ and a maximum thrust of $30 \text{ ft/s}^2$.

Because the lander rarely hits the ground exactly on a whole second, the physics engine calculates the exact fractional second of touchdown to determine the true impact velocity.

Evolution Fitness Function:
* Perfect Landing (≤ 0 ft/s): Massive point reward, plus bonus points for remaining fuel.
* Hard Landing (< 2 ft/s): High points, penalized slightly by the impact speed.
* Crash (≥ 2 ft/s): Base survival points minus a heavy penalty for impact velocity.
