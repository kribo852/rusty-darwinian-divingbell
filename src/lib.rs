use rand::Rng;

struct SimulationInstance<GenomeType> {
    genome: GenomeType,
    score: f64,
}

const COMPLETE_VECTOR_MAX_SIZE: usize = 100;

pub fn run_simulation<GenomeType: Clone>(
    new_genome: fn() -> GenomeType,
    score_fn: fn(&GenomeType) -> f64,
    output: fn(&GenomeType),
    mutators: Vec<fn(&[GenomeType]) -> GenomeType>,
    finish: fn(&GenomeType) -> bool,
) -> GenomeType {
    let mut complete: Vec<SimulationInstance<GenomeType>> = vec![];
    let mut not_yet_complete: Vec<SimulationInstance<GenomeType>> = vec![];

    let mut high_score = 0.0;
    let mut best_genome = new_genome();

    while !finish(&best_genome) {
        let run_sim_instance =
            get_sim_instance_to_run(new_genome, &mut complete, &mut not_yet_complete);
        let mut genome_array: [GenomeType; 8] = [
            run_sim_instance.genome.clone(),
            run_sim_instance.genome.clone(),
            run_sim_instance.genome.clone(),
            run_sim_instance.genome.clone(),
            run_sim_instance.genome.clone(),
            run_sim_instance.genome.clone(),
            run_sim_instance.genome.clone(),
            run_sim_instance.genome.clone(),
        ];
        let mut measured_scores = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        for index in 0..measured_scores.len() {
            measured_scores[index] = score_fn(&genome_array[index]);
        }

        for _iterations in 1..1000 {
            let mutated_genome = get_mutated_genome(&genome_array, &mutators);
            let temp_score = score_fn(&mutated_genome);

            for index in 0..measured_scores.len() {
                if temp_score > measured_scores[index] {
                    measured_scores[index] = temp_score;
                    genome_array[index] = mutated_genome;
                    break;
                }
            }
        }

        let (highest_new_score, best_new_genome) =
            find_highest_score_genome(&measured_scores, &genome_array);

        if highest_new_score > high_score {
            high_score = highest_new_score;
            best_genome = best_new_genome.clone();
            output(&best_new_genome);
        }
        if highest_new_score > run_sim_instance.score {
            not_yet_complete.push(SimulationInstance {
                genome: best_new_genome,
                score: highest_new_score,
            });
        } else if highest_new_score == run_sim_instance.score
            && complete.len() < COMPLETE_VECTOR_MAX_SIZE
        {
            complete.push(SimulationInstance {
                genome: best_new_genome,
                score: highest_new_score,
            });
        } else if highest_new_score == run_sim_instance.score
            && complete.len() == COMPLETE_VECTOR_MAX_SIZE
        {
            for index in 0..complete.len() {
                if highest_new_score > complete[index].score {
                    complete[index] = SimulationInstance {
                        genome: best_new_genome,
                        score: highest_new_score,
                    };
                    break;
                }
            }
        }
    }

    return best_genome;
}

fn find_highest_score_genome<GenomeType: Clone>(
    measured_scores: &[f64; 8],
    genome_array: &[GenomeType; 8],
) -> (f64, GenomeType) {
    let mut highest_new_score = measured_scores[0];
    let mut best_new_genome_index = 0;
    for index in 1..measured_scores.len() {
        if measured_scores[index] > highest_new_score {
            highest_new_score = measured_scores[index];
            best_new_genome_index = index;
        }
    }

    (
        highest_new_score,
        genome_array[best_new_genome_index].clone(),
    )
}

fn get_sim_instance_to_run<GenomeType>(
    new_genome: fn() -> GenomeType,
    complete: &mut Vec<SimulationInstance<GenomeType>>,
    not_yet_complete: &mut Vec<SimulationInstance<GenomeType>>,
) -> SimulationInstance<GenomeType> {
    let mut rng = rand::rng();

    if rng.random_ratio(complete.len() as u32, (3 * COMPLETE_VECTOR_MAX_SIZE) as u32) {
        if !complete.is_empty() {
            return complete.remove(rng.random_range(0..complete.len()));
        }
    }

    match not_yet_complete.pop() {
        Some(instance) => instance,
        None => SimulationInstance {
            genome: new_genome(),
            score: 0.0,
        },
    }
}

fn get_mutated_genome<GenomeType>(
    original_genomes: &[GenomeType],
    mutators: &Vec<fn(genome: &[GenomeType]) -> GenomeType>,
) -> GenomeType {
    let mut rng = rand::rng();

    mutators[rng.random_range(0..mutators.len())](original_genomes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_framework() {
        run_simulation(
            new_test_genome,
            calculate_score,
            output_blocks,
            vec![mutate],
            finish,
        );
    }

    fn new_test_genome() -> [bool; 25] {
        let mut rng = rand::rng();
        let mut rtn = [false; 25];

        for index in 0..rtn.len() {
            rtn[index] = rng.random::<bool>();
        }
        rtn
    }

    fn output_blocks(genome: &[bool; 25]) {
        println!("Printing out the genome {:?}", genome);
        println!("Score: {}", calculate_score(genome));
    }

    fn calculate_score(genome: &[bool; 25]) -> f64 {
        let mut score_rtn = 0.0;

        if genome[0] && !genome[1] {
            score_rtn += 1.0;
        }

        if !genome[genome.len() - 2] && genome[genome.len() - 1] {
            score_rtn += 1.0;
        }

        for i in 1..genome.len() - 1 {
            if !genome[i - 1] && genome[i] && !genome[i + 1] {
                score_rtn += 1.0;
            }
        }

        score_rtn
    }

    fn finish(genome: &[bool; 25]) -> bool {
        calculate_score(genome) >= 13.0
    }

    fn mutate(old_genome: &[[bool; 25]]) -> [bool; 25] {
        let mut rng = rand::rng();

        let mut new_genome = old_genome[rng.random_range(..old_genome.len())].clone();

        for _i in 0..5 {
            let rndint = rng.random_range(..new_genome.len());
            new_genome[rndint] = rng.random::<bool>();
        }

        new_genome
    }
}
