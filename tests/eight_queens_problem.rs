use rand::Rng;
use rusty_darwinian_divingbell::run_simulation;

static PROBLEM_LEN: usize = 8;

//the eight queens problem, which is to place eight queens on a chess board without them being able to strike each other. 
#[test]
fn test_eight_queens() {
    run_simulation(
        new_genome,
        score_function,
        output,
        vec![mutate],
        finish,
    );
}

//the genome is a permutation, in which order the nodes should be visited
fn new_genome() -> [usize; PROBLEM_LEN] {
    let mut genome: [usize; PROBLEM_LEN] = [0; PROBLEM_LEN];

    for index in 0..genome.len() {
        genome[index] = index;
    }

    let mut rng = rand::rng();

    for _iteration in 0..100 {
        let indexa = rng.random_range(..genome.len());
        let indexb = rng.random_range(..genome.len());

        let tmp = genome[indexa];
        genome[indexa] = genome[indexb];
        genome[indexb] = tmp;
    }

    genome
}

fn score_function(genome: &[usize; PROBLEM_LEN]) -> f64 {
    let mut return_score = 0.0;

    for index in 0..PROBLEM_LEN {
        for after_index in index+1..PROBLEM_LEN {

            let i: i32 = index as i32;
            let j: i32 = after_index as i32;

            //compare diagonally 
            if i - j == genome[index] as i32 - genome[after_index] as i32 || 
            i - j == genome[after_index] as i32 - genome[index] as i32  {
                return_score -= 1.0
            }
        }
    }

    return_score
}

fn output(genome: &[usize; PROBLEM_LEN]) {
    for y_value in 0..PROBLEM_LEN {
        for x_value in 0..PROBLEM_LEN {
            if genome[x_value] == y_value {
                print!("◽");
            } else {
                print!("◾");
            }
        }
        println!("");
    }
    println!("");
}

fn finish(genome: &[usize; PROBLEM_LEN]) -> bool {
    score_function(genome) == 0.0
}

fn mutate(genomes: &[[usize; PROBLEM_LEN]]) -> [usize; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let mut new_genome = genomes[rng.random_range(0..genomes.len())].clone();

    let indexa = rng.random_range(0..new_genome.len());
    let indexb = (indexa + rng.random_range(1..new_genome.len()))%PROBLEM_LEN;

    let tmp = new_genome[indexa];
    new_genome[indexa] = new_genome[indexb];
    new_genome[indexb] = tmp;

    new_genome
}
