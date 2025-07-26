# Rusty-darwinian-divingbell
An atempt to try some Rust programming, I decided to try and make a Library in Rust for running genetic algorithms.
This is as much a project for learning Rust as for making a fast and efficient way of running genetic algorithms. 

## Use
Call the function run_simulation in the module lib.rs, to run the genetic algorithm. The idea is that you can import this library into your own project, if you want to solve a problem with a genetic algorithm, as part of your own code.


## Examples

The examples are located in the test folder
| Problem name | File name | implemented | wikipedia link |
|--------------|-----------|-------------|----------------|
| Maximimize volume of a cylinder |  | ❌ |  |
| Eight queens problem | eight_queens_problem.rs | ✅️ | https://en.wikipedia.org/wiki/Eight_queens_puzzle |
|Traveling salesman problem | traveling_salesman_test.rs | ✅️ | https://en.wikipedia.org/wiki/Travelling_salesman_problem |

Note that the solutions to the traveling salesman problem are not optimal, and seldom better than the
 solutions given by the greedy algorithm. But, they are usually better than a random path :).

## Plantuml
There is a plantuml that shows how the control-flow of the library is supposed to work, but all functionality is not implemented (yet).

## Unimplemented features
Some algorithms might want to use imprecise simulations to calculate their score, it would be nice to implement a t-test or z-score or similar to decide if a solution is better than another in these cases.

Crossover mutation, a function that takes several (or just two) different genomes, and produces a new child genome.

Multi-threading, it would be nice to be able to run the genetic algorithm in several threads simultaniously.

Scoring of mutators and crossover-mutators, could help select the best mutation and speed up the process.

## Terminology
### Genome
Any datastructure describing a solution, usually an array of booleans, enums or numbers. However, this can also be another structure, such as a tree
structure. One requirement is that the genome must implement a clone method. The user of the library supplies a function for producing new untested genomes.
### Mutator
A function for making a new genome, slightly mutated, from the array slice of competing genomes. The user of the library supplies a vector of mutator functions which performs valid mutations. The reason for this signature of the function is to give the user the ability to cross genomes, with or without a mutation, as well as to mutate single genomes. The user shouldn´t rely on the size of the array slice of genomes to be large, but a few elements in size.   

## The API
`fn run_simulation<GenomeType: Clone>(
    new_genome: fn() -> GenomeType,
    score_fn: fn(genome: &GenomeType) -> f64,
    output: fn(genome: &GenomeType),
    mutators: Vec<fn(&[GenomeType]) -> GenomeType>,
    finish: fn(best_scored_genome: &GenomeType) -> bool,
) -> GenomeType`

The API consists of a run_simulation function, taking mostly as arguments, user provided functions for managing genomes.  
The new_genome function returns a whole new genome, that is untested.  
The score_fn function tries a genome/solution to decide its score, this can be done by a simple calculation or an advanced simulation.  
The output function writes out a description of the genome, so that the user can view the genome.  
The mutators, a vector of mutator functions.  
The finish function, decides to finish depending on the best current genome. You could calculate the score, depending on
the best genome, but also assess som property of the genome, or finish after some amount of time.

Support a free Palestine 🇵🇸
