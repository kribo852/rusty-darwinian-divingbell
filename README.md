#Rusty-darwinian-divingbell
An atempt to try some Rust programming, I decided to try and make a Library in Rust for running genetic algorithms.

##Example of use
The lib.rs file contains a unit test that shows a use of the library. But the idea is really to import the library into other projects.


##Plantuml
There is a plantuml that shows how the control-flow of the library is supposed to work, but all functionality is not implemented (yet).

##Unimplemented features
Some algorithms might want to use imprecise simulations to calculate their score, it would be nice to implement a t-test or z-score or similar to decide if a souloution is better than another in these cases.

Crossover mutation, a function that takes several (or just two) different genomes, and produces a new child genome.

Multi-threading, it would be nice to be able to run the genetic algorithm in several threads simultaniously.

Scoring of mutators and crossover-mutators, could help select the best mutation and speed up the process.

##Terminology
###Genome
-
###Mutator
-

