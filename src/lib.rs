use rand::Rng;

struct SimulationInstance<GenomeType> {
    genome: GenomeType,
    score: f64
}


pub fn run_simulation<GenomeType: Clone>(new_genome: fn() -> GenomeType, 
                                score_fn: fn(genome: &GenomeType) -> f64,
                                output: fn(genome: &GenomeType),
                                mutators: Vec<fn(genome: &GenomeType) -> GenomeType>,
                                finish: fn(high_score: f64) -> bool) {


    let mut complete: Vec<SimulationInstance<GenomeType>> = vec![];
    let mut not_yet_complete: Vec<SimulationInstance<GenomeType>> = vec![];

    let mut high_score = 0.0;

    while !finish(high_score) {

        let run_sim_instance = get_sim_instance_to_run(new_genome, &mut complete, &mut not_yet_complete);
        let mut genome_array: [GenomeType; 8] = [run_sim_instance.genome.clone(), 
        get_mutated_genome(&run_sim_instance.genome, &mutators), 
        get_mutated_genome(&run_sim_instance.genome, &mutators), 
        get_mutated_genome(&run_sim_instance.genome, &mutators), 
        get_mutated_genome(&run_sim_instance.genome, &mutators), 
        get_mutated_genome(&run_sim_instance.genome, &mutators), 
        get_mutated_genome(&run_sim_instance.genome, &mutators), 
        get_mutated_genome(&run_sim_instance.genome, &mutators)];
        let mut calc_score = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        for i in 0..calc_score.len() {
            calc_score[i] = score_fn(&genome_array[i]);
        }

        for _iterations in 1..100 {
            let mut rng = rand::rng();

            let mutated_genome = get_mutated_genome(&genome_array[rng.random_range(0..calc_score.len())], &mutators);
            let temp_score = score_fn(&mutated_genome);

            for index in 0..calc_score.len() {
                if temp_score > calc_score[index] {
                    calc_score[index] = temp_score;
                    genome_array[index] = mutated_genome;
                    break;
                }
            }    
        }
        let mut highest_new_score = 0.0;
        let mut best_new_genome = genome_array[0].clone();
        for index in 0..calc_score.len() {
            if calc_score[index] > highest_new_score {
                highest_new_score = calc_score[index];
                best_new_genome = genome_array[index].clone();
            }
        }

        if highest_new_score > high_score {
            high_score = highest_new_score;
            output(&best_new_genome);
        } 
        if highest_new_score > run_sim_instance.score {
            not_yet_complete.push(SimulationInstance{genome: best_new_genome, score: highest_new_score});
        } else if highest_new_score == run_sim_instance.score {
            complete.push(SimulationInstance{genome: best_new_genome, score: highest_new_score});
        }


    }
}

fn get_sim_instance_to_run<GenomeType>(new_genome: fn() -> GenomeType, 
    complete: &mut Vec<SimulationInstance<GenomeType>>,
    not_yet_complete: &mut Vec<SimulationInstance<GenomeType>>) 
    -> SimulationInstance<GenomeType> {

    let mut rng = rand::rng();

    if rng.random_range(0..10)==0 {
        let complete_element = complete.pop();

        if complete_element.is_some() {
            return complete_element.unwrap();
        }
    }

    match not_yet_complete.pop() {
        Some(instance) => instance,
        None => SimulationInstance{genome: new_genome(), score: 0.0},
    }
}

fn get_mutated_genome<GenomeType>(original_genome: &GenomeType, 
                                mutators: &Vec<fn(genome: &GenomeType) -> GenomeType>) -> GenomeType {
    mutators[0](original_genome)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_framework() {
        run_simulation(new_test_genome, calculate_score, output_blocks, vec![mutate], finish);
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

        for i in 0..genome.len() {
            if genome[i] {
                score_rtn+=1.0;
            } 
        }

        score_rtn
    }

    fn finish(score: f64) -> bool {
        score >=25.0
    }

    fn mutate(old_genome: &[bool; 25]) -> [bool; 25] {
        let mut new_genome = old_genome.clone();
        let mut rng = rand::rng();

        for _i in 0..5 {
            let rndint = rng.random_range(0..new_genome.len());
            new_genome[rndint] = rng.random::<bool>();
        }

        new_genome
    }

}
