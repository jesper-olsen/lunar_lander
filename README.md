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

```bash
            CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY



LUNAR LANDING SIMULATION
----- ------- ----------


--- INITIATING EVOLUTIONARY AUTOPILOT ---
Gen 01 | Best Fitness: 939.5 | Impact Speed: 60.50 ft/s | Crashed
Gen 05 | Best Fitness: 988.0 | Impact Speed: 12.00 ft/s | Crashed

SUCCESS! Perfect landing sequence evolved at Generation 7!

===============================================================
SUCCESSFUL FLIGHT DATA RECOVERED (REPLAYING SIMULATION):
===============================================================

SEC  FEET      SPEED     FUEL     BURN  PLOT OF DISTANCE

0    1000.00   50.00     150.0    1     I                                                                  *
1    948.00    54.00     149.0    3     I                                                               *
2    893.00    56.00     146.0    1     I                                                           *
3    835.00    60.00     145.0    6     I                                                       *
4    775.50    59.00     139.0    0     I                                                   *
5    714.00    64.00     139.0    0     I                                               *
6    647.50    69.00     139.0    2     I                                           *
7    577.00    72.00     137.0    20    I                                      *
8    512.50    57.00     117.0    0     I                                  *
9    453.00    62.00     117.0    3     I                              *
10   390.00    64.00     114.0    3     I                          *
11   325.00    66.00     111.0    2     I                     *
12   257.50    69.00     109.0    29    I                 *
13   200.50    45.00     80.0     2     I             *
14   154.00    48.00     78.0     7     I          *
15   107.00    46.00     71.0     20    I       *
16   68.50     31.00     51.0     14    I    *
17   42.00     22.00     37.0     6     I  *
18   20.50     21.00     31.0     16    I *
19   5.00      10.00     15.0     15    I*
***** CONTACT *****
LANDING VELOCITY = 0.00 FEET/SEC.
0.0 UNITS OF FUEL REMAINING.
```

## The Physics & Scoring

The simulation runs in discrete 1-second chunks ($DT = 1.0$), with a gravity of $5.0 \text{ ft/s}^2$ and a maximum thrust of $30 \text{ ft/s}^2$.

Because the lander rarely hits the ground exactly on a whole second, the physics engine calculates the exact fractional second of touchdown to determine the true impact velocity.

Evolution Fitness Function:
* Perfect Landing (≤ 0 ft/s): Massive point reward, plus bonus points for remaining fuel.
* Hard Landing (< 2 ft/s): High points, penalized slightly by the impact speed.
* Crash (≥ 2 ft/s): Base survival points minus a heavy penalty for impact velocity.
