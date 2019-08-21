mod ga;

use ga::*;

const GENERATION: i32 = 100;

fn main() {
    let mut pop = Population::new(100, 10, 0.6, 0.2);

    pop.evaluate();
    println!("Generation : 0");
    println!("Max : {}", pop.max().rank);
    println!("Min : {}", pop.min().rank);
    println!("-----------------------------");

    for gen in 1..GENERATION {
        pop.evolution();
        println!("Generation : {}", gen);
        println!("Max : {}", pop.max().rank);
        println!("Min : {}", pop.min().rank);
        println!("-----------------------------");
    }
}
