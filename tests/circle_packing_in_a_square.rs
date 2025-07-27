use rand::Rng;
use rusty_darwinian_divingbell::run_simulation;
use rand_distr::{Normal, Distribution, Uniform};
use std::fmt;

//number of circles to pack in a square. I have delibrately choosen 82 since this makes the square with 
//side length (20 units)x(20 units) quite inefficient  
static PROBLEM_LEN: usize = 82;

//Pack PROBLEM_LEN number of circles inside as small a square as possible, 
//This algorithm gives a heuristic solution using a genetic algorithm.
#[test]
fn test_pack_circles() {
    let solution = run_simulation(
        new_genome,
        score_negative_side_length,
        output,
        vec![mutate, mutate, mutate, mutate, mutate, mutate_2, mutate_3],
        finish,
    );
    println!("points representing the dual of the problem");
    println!("(placing points in a unit square to maximize the minimum distance)");
    println!("{:?}", solution);
}

fn new_genome() -> [Coordinate; PROBLEM_LEN] {
    let mut genome: [Coordinate; PROBLEM_LEN] = [Coordinate{x_value: 0.0, y_value:0.0}; PROBLEM_LEN];
    let mut rng = rand::rng();

    for index in 0..genome.len() {
      let uniform = Uniform::new(0.0, 1.0).unwrap();
      genome[index] = Coordinate{x_value: uniform.sample(&mut rand::rng()), y_value: uniform.sample(&mut rand::rng())};  
    }

    genome
}

fn score_negative_side_length(genome: &[Coordinate; PROBLEM_LEN]) -> f64 {
    for index_a in 0..genome.len() {
        if genome[index_a].x_value < 0.0 || genome[index_a].x_value >=1.0 || genome[index_a].y_value < 0.0 || 
        genome[index_a].y_value >= 1.0 {
            return f64::MIN;
        }
    }

    let mut square_minimum_dist = 2.0;

    for index_a in 0..genome.len() {

        for index_b in index_a+1..genome.len() {
            let sqr_distance = calc_sqr_distance(genome[index_a], genome[index_b]);

            if sqr_distance < square_minimum_dist {
                 square_minimum_dist = sqr_distance;
             }
        }
    }

    -(2.0 + 2.0/(square_minimum_dist.sqrt() as f64))  //- side length of square
}

fn output(genome: &[Coordinate; PROBLEM_LEN]) {
    println!("square needs: {} meters/units to pack {} unit circles", -score_negative_side_length(genome), PROBLEM_LEN);
}

fn finish(genome: &[Coordinate; PROBLEM_LEN]) -> bool {
    -score_negative_side_length(genome) < 19.0
}

fn mutate(genomes: &[[Coordinate; PROBLEM_LEN]]) -> [Coordinate; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let mut new_genome = genomes[rng.random_range(0..genomes.len())].clone();

    let number_of_iterations = rng.random_range(1..6);

    for _ in 0..number_of_iterations {
        let change_index = rng.random_range(0..new_genome.len());

        let tmp_coord = new_genome[change_index];
        let normal = Normal::new(0.0, 0.01).unwrap();

        new_genome[change_index] = 
        Coordinate{x_value: tmp_coord.x_value + normal.sample(&mut rand::rng()), 
            y_value: tmp_coord.y_value + normal.sample(&mut rand::rng())};   
    }

    new_genome
}

fn mutate_2(genomes: &[[Coordinate; PROBLEM_LEN]]) -> [Coordinate; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let mut new_genome = genomes[rng.random_range(0..genomes.len())].clone();

    let uniform = Uniform::new(0.0, 1.0).unwrap();

    let mut move_rect_x1 = uniform.sample(&mut rand::rng());
    let mut move_rect_y1 = uniform.sample(&mut rand::rng());
    let mut move_rect_x2 = uniform.sample(&mut rand::rng());
    let mut move_rect_y2 = uniform.sample(&mut rand::rng());

    if move_rect_x2 < move_rect_x1 {
        let tmpx = move_rect_x1;
        move_rect_x1 = move_rect_x2;
        move_rect_x2 = tmpx;
    }

    if move_rect_y2 < move_rect_y1 {
        let tmpy = move_rect_y1;
        move_rect_y1 = move_rect_y2;
        move_rect_y2 = tmpy;
    }

    let normal = Normal::new(0.0, 0.01).unwrap();
    let delta_x = normal.sample(&mut rand::rng());
    let delta_y = normal.sample(&mut rand::rng());

    for index in 0..new_genome.len() {
        
        let curr_coord = new_genome[index];

        if curr_coord.x_value > move_rect_x1 && curr_coord.x_value < move_rect_x2 && 
        curr_coord.y_value > move_rect_y1 && curr_coord.y_value < move_rect_y2 {
            new_genome[index] = 
            Coordinate {x_value: curr_coord.x_value + delta_x, y_value: curr_coord.y_value + delta_y};   
        }
    }

    new_genome
}


fn mutate_3 (genomes: &[[Coordinate; PROBLEM_LEN]]) -> [Coordinate; PROBLEM_LEN] {
    let mut rng = rand::rng();
    let mut new_genome = genomes[rng.random_range(0..genomes.len())].clone();

    let mut min_dist_sqr = 2.0;
    let mut min_index_a = 0;
    let mut min_index_b = 0;

    for index_a in 0..new_genome.len() {
        for index_b in index_a + 1..new_genome.len() {
            let sqr_distance = calc_sqr_distance(new_genome[index_a], new_genome[index_b]);
            if sqr_distance < min_dist_sqr {
                min_index_a = index_a;
                min_index_b = index_b;
                min_dist_sqr = sqr_distance;
            }
        }
    }

    if rng.random::<bool>() {
        let uniform = Uniform::new(0.0, 1.0).unwrap();
        new_genome[min_index_a] = 
        Coordinate{x_value: uniform.sample(&mut rand::rng()), y_value: uniform.sample(&mut rand::rng())};
    } else {
        let uniform = Uniform::new(0.0, 1.0).unwrap();
        new_genome[min_index_b] = 
        Coordinate{x_value: uniform.sample(&mut rand::rng()), y_value: uniform.sample(&mut rand::rng())};
    }

    new_genome
}

fn calc_sqr_distance (coord_a: Coordinate, coord_b: Coordinate)-> f64 {

    let delta_x = coord_a.x_value - coord_b.x_value;
    let delta_y = coord_a.y_value - coord_b.y_value;

    (delta_x*delta_x+delta_y*delta_y) as f64
}


#[derive(Clone)]
#[derive(Copy)]
struct Coordinate {
    x_value: f32,
    y_value: f32,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x_value, self.y_value)
    }
}
