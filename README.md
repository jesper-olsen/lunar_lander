# Lunar Lander

A Rust implementation of the classic text-based [Lunar Lander](rocket.bas) game. The original program was written by Jim Storer in 1969 and later popularized by David Ahl's BASIC Computer Games (1978).

This project preserves the original gameplay while introducing a decoupled physics engine and an evolutionary autopilot powered by a genetic algorithm.

Although the mathematical foundations of genetic algorithms already existed in the 1970s, they were not part of the original Lunar Lander implementation. Running an evolutionary search on the small home computers of the era would have been impractical due to their limited memory and processing power. Today, a perfect landing sequence can be evolved in a fraction of a second on a modern laptop.

## Features

* **Classic Interactive Gameplay:** Balance fuel burn and velocity to avoid crashing into the surface.
* **Evolutionary Autopilot:** Let the solver find a perfect landing sequence.
* **Decoupled Physics Engine:** Core mechanics (gravity, thrust, fuel consumption, and fractional-second touchdown calculations) are isolated in a reusable library.

## Prerequisites

Get [Rust](https://rust-lang.org/tools/install/)

## How to Run

### 1. Play the Game (Manual Mode)

```bash
cargo run --bin play
```

### 2. Train the Autopilot (Evolution Mode)

```bash
cargo run --bin evolve
```

```bash
            CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY



LUNAR LANDING SIMULATION
----- ------- ----------


--- INITIATING EVOLUTIONARY AUTOPILOT ---
Gen 01 | Best Fitness: 940.7 | Impact Speed: 59.33 ft/s | Fuel left: 00 | Duration: 31.9 | Crashed
Gen 05 | Best Fitness: 9980.0 | Impact Speed: 0.00 ft/s | Fuel left: 00 | Duration: 20.0 | Perfect
Gen 10 | Best Fitness: 10031.0 | Impact Speed: 0.00 ft/s | Fuel left: 05 | Duration: 19.0 | Perfect
Gen 15 | Best Fitness: 10031.0 | Impact Speed: 0.00 ft/s | Fuel left: 05 | Duration: 19.0 | Perfect
Gen 20 | Best Fitness: 10082.0 | Impact Speed: 0.00 ft/s | Fuel left: 10 | Duration: 18.0 | Perfect
Gen 25 | Best Fitness: 10082.0 | Impact Speed: 0.00 ft/s | Fuel left: 10 | Duration: 18.0 | Perfect
Gen 30 | Best Fitness: 10082.0 | Impact Speed: 0.00 ft/s | Fuel left: 10 | Duration: 18.0 | Perfect
Gen 35 | Best Fitness: 10082.0 | Impact Speed: 0.00 ft/s | Fuel left: 10 | Duration: 18.0 | Perfect
Gen 40 | Best Fitness: 10082.0 | Impact Speed: 0.00 ft/s | Fuel left: 10 | Duration: 18.0 | Perfect
Gen 45 | Best Fitness: 10082.0 | Impact Speed: 0.00 ft/s | Fuel left: 10 | Duration: 18.0 | Perfect
Gen 50 | Best Fitness: 10082.0 | Impact Speed: 0.00 ft/s | Fuel left: 10 | Duration: 18.0 | Perfect

EVOLUTION COMPLETE: Optimal flight plan found.

===============================================================
SUCCESSFUL FLIGHT DATA RECOVERED (REPLAYING SIMULATION):
===============================================================

SEC  FEET      SPEED     FUEL     BURN  PLOT OF DISTANCE

0    1000.00   50.00     150.0    1     I                                                                  *
1    948.00    54.00     149.0    3     I                                                               *
2    893.00    56.00     146.0    0     I                                                           *
3    834.50    61.00     146.0    4     I                                                       *
4    773.00    62.00     142.0    0     I                                                   *
5    708.50    67.00     142.0    6     I                                               *
6    642.00    66.00     136.0    6     I                                          *
7    576.50    65.00     130.0    2     I                                      *
8    510.00    68.00     128.0    9     I                                  *
9    444.00    64.00     119.0    2     I                             *
10   378.50    67.00     117.0    5     I                         *
11   311.50    67.00     112.0    16    I                    *
12   250.00    56.00     96.0     8     I                *
13   195.50    53.00     88.0     4     I             *
14   142.00    54.00     84.0     7     I         *
15   89.00     52.00     77.0     17    I     *
16   43.00     40.00     60.0     22    I  *
17   11.50     23.00     38.0     28    I*
***** CONTACT *****
LANDING VELOCITY = 0.00 FEET/SEC.
10.0 UNITS OF FUEL REMAINING.
```

## The Physics & Scoring

The simulation follows the original game's one-second control intervals: each burn command specifies the fuel consumed during the next one-second time step.

Because touchdown rarely occurs exactly at the end of a time step, the engine computes the precise fractional second of contact to determine the true impact velocity.

Evolutionary Fitness Function:
* Perfect Landing (0 ft/s): Massive point reward, plus bonus points for remaining fuel.
* Hard Landing (max 2 ft/s): High points, penalized slightly by the impact speed.
* Crash (above 2 ft/s): Base survival points minus a heavy penalty for impact velocity.

The genetic algorithm seeks safe landings while minimizing fuel consumption.

## References

1. "Basic Computer Games - Microcomputer Edition", Edited by David E. Ahl, 1978 
2. "Adaptation in Natural and Artificial Systems", John Holland, 1975 
