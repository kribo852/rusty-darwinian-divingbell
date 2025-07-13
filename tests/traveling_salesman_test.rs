use rand::Rng;
use rusty_darwinian_divingbell::run_simulation;
use std::sync::LazyLock;

static PROBLEM_LEN: usize = 100;

//a global static instance of a traveling salesman problem
static PROBLEM: LazyLock<[Coordinate; PROBLEM_LEN]> = LazyLock::new(|| {
    let mut problem_to_solve: [Coordinate; PROBLEM_LEN] =
        [Coordinate { x: 0.0, y: 0.0 }; PROBLEM_LEN];
    let mut rng = rand::rng();

    for index in 0..PROBLEM_LEN {
        problem_to_solve[index] = Coordinate {
            x: rng.random::<f64>(),
            y: rng.random::<f64>(),
        };
    }

    problem_to_solve
});

#[test]
fn test_traveling_salesman() {
    run_simulation(
        new_genome,
        score_function,
        output,
        vec![mutate, mutate2, mutate3],
        finish,
    );
    println!(
        "Unoptimized sample distance was {}",
        distance(&new_genome())
    );
    greedy_solve();
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
    -distance(genome)
}

//calculates the length/distance of a path between the nodes of the problem instance.
//the path is given by the permutation in the genome.
fn distance(genome: &[usize; PROBLEM_LEN]) -> f64 {
    let mut dist = 0.0;

    for index in 1..genome.len() {
        dist += ((PROBLEM[genome[index]].x - PROBLEM[genome[index - 1]].x).powi(2)
            + (PROBLEM[genome[index]].y - PROBLEM[genome[index - 1]].y).powi(2))
        .sqrt();
    }

    dist += ((PROBLEM[genome[0]].x - PROBLEM[genome[genome.len() - 1]].x).powi(2)
        + (PROBLEM[genome[0]].y - PROBLEM[genome[genome.len() - 1]].y).powi(2))
    .sqrt();

    dist
}

fn output(genome: &[usize; PROBLEM_LEN]) {
    println!("{:?}", genome);
    println!(
        "Distance of TSP path calculated with the genetic algorithm {}",
        distance(genome)
    );
}

fn finish(genome: &[usize; PROBLEM_LEN]) -> bool {
    score_function(genome) > -13.0
}

fn mutate(genomes: &[[usize; PROBLEM_LEN]]) -> [usize; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let mut new_genome = genomes[rng.random_range(0..genomes.len())].clone();
    let no_swaps = rng.random_range(0..10);

    for _index in 0..no_swaps {
        let indexa = rng.random_range(0..new_genome.len());
        let indexb = rng.random_range(0..new_genome.len());

        let tmp = new_genome[indexa];
        new_genome[indexa] = new_genome[indexb];
        new_genome[indexb] = tmp;
    }

    new_genome
}

fn mutate2(genomes: &[[usize; PROBLEM_LEN]]) -> [usize; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let mut new_genome = genomes[rng.random_range(0..genomes.len())].clone();

    let indexa = rng.random_range(0..new_genome.len());
    let indexb = rng.random_range(0..new_genome.len());

    let tmp = new_genome[indexa];
    new_genome[indexa] = new_genome[indexb];
    new_genome[indexb] = tmp;

    new_genome
}

fn mutate3(genomes: &[[usize; PROBLEM_LEN]]) -> [usize; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let genome = genomes[rng.random_range(0..genomes.len())].clone();
    let swap_index = rng.random_range(0..genome.len());

    mutate3_single(&genome, swap_index)
}

fn mutate3_single(genome: &[usize; PROBLEM_LEN], swap_index: usize) -> [usize; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let mut tmp_genome = genome.clone();

    let tmp = tmp_genome[swap_index];
    tmp_genome[swap_index] = tmp_genome[(swap_index + 1) % PROBLEM_LEN];
    tmp_genome[(swap_index + 1) % PROBLEM_LEN] = tmp;

    if rng.random::<bool>() {
        return mutate3_single(&tmp_genome, (swap_index + 1) % PROBLEM_LEN);
    }

    tmp_genome
}

fn greedy_solve() {
    let mut nodes_left = Vec::new();
    let mut nodes_store = Vec::new();
    let mut distance_answer = 0.0;

    for index in 0..PROBLEM_LEN {
        nodes_left.push(PROBLEM[index].clone());
    }

    let mut current_node: Coordinate = nodes_left.pop().unwrap();

    while !nodes_left.is_empty() {
        let mut test_dist = f64::MAX;
        let mut test_index = 0;

        for index in 0..nodes_left.len() {
            let tempdist = ((nodes_left[index].x - current_node.x).powi(2)
                + (nodes_left[index].y - current_node.y).powi(2))
            .sqrt();

            if tempdist < test_dist {
                test_index = index;
                test_dist = tempdist;
            }
        }

        nodes_store.push(current_node);
        current_node = nodes_left.remove(test_index);
        distance_answer += test_dist;
    }
    distance_answer += ((nodes_store[0].x - current_node.x).powi(2)
        + (nodes_store[0].y - current_node.y).powi(2))
    .sqrt();

    println!(
        "Distance of TSP problem path calculated with greedy algorithm {:?}",
        distance_answer
    );
}

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: f64,
    y: f64,
}
