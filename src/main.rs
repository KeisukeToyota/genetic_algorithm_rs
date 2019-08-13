mod ga;

use ga::*;

const GENERATION: i32 = 20;

fn main() {
    let mut pop = Population::new(10, 10, 0.1, 0.2);

    pop.evaluate();
    println!("Generation : 0");
    println!("Max : {}", pop.max().rank);
    println!("Min : {}", pop.min().rank);
    println!("-----------------------------");

    for gen in 0..GENERATION {
        pop.evolution();
        println!("Generation : {}", gen + 1);
        println!("Max : {}", pop.max().rank);
        println!("Min : {}", pop.min().rank);
        println!("-----------------------------");
    }
}
