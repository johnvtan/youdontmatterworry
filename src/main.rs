use std::env;
use std::fs;
use rand::Rng;
use sentiment::{negativity, positivity};

fn main() {
    println!("Reading in file");
    let verb_lines = fs::read_to_string("verbs.txt")
        .expect("something went wrong reading the file");
    let verbs: Vec<String> = verb_lines
                            .lines()
                            .map(|x| x.to_string())
                            .collect();

    let mut rng = rand::thread_rng();
        
    let neg_verbs: Vec<String> = verbs.clone().into_iter().filter(|x| negativity(x.to_string()).score > 1.0).collect();
    let pos_verbs: Vec<String> = verbs.clone().into_iter().filter(|x| positivity(x.to_string()).score > 1.0).collect();
    println!("{:?}", neg_verbs);
    println!("{:?}", pos_verbs);

    for i in 0..100 {
        let neg = &neg_verbs[rng.gen_range(0, neg_verbs.len())];
        let pos = &pos_verbs[rng.gen_range(0, pos_verbs.len())];
        println!("you {}. don't {}.", pos, neg);
        println!("you don't {}. {}.", pos, neg);
        println!("-------------------------");
    } 
}
