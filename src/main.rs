#![feature(test)]
extern crate test;
use clap::{App, Arg};
use std::env;

/// Debug flag
// const DEBUG: bool = true;
const DEBUG: bool = false;

fn main() {
    let matches = App::new("Towers of Hanoi")
        .version("0.0.1")
        .author("Avery Wagar <ajmw.subs@gmail.com>")
        .about("Simulates the Towers of Hanoi puzzle")
        .arg(Arg::with_name("RINGS")
             .value_name("RINGS")
             .help("Set the number of rings to simulate.")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let n = matches.value_of("RINGS").unwrap_or("32");

    let n = n.parse::<u64>().unwrap_or(32);

    let v: usize = matches.occurrences_of("v") as usize;

    let moves = (2 as u64).pow(n as u32) - 1;

    println!("Tower of Hanoi\nRings: {}\nRequired moves: {}",n, moves);

    // Setup inital vectors
    let (mut a, mut b, mut c) = setup(n, v);

    // Show our progress
    println!("\nStarting!");
    println!("============\nA: {:?}\nB: {:?}\nC: {:?}", a, b, c);

    tower_of_hanoi(n, &mut a, &mut c, &mut b, v);

    println!("\nDone!");
    println!("============\nA: {:?}\nB: {:?}\nC: {:?}", a, b, c);

}

/// Sets the inital vectors, fills a with 0..n values
fn setup(n: u64, v: usize) -> (Vec<u64>, Vec<u64>, Vec<u64>){

    // Create vectors
    let mut a: Vec<u64> = Vec::new();
    let b: Vec<u64> = Vec::new();
    let c: Vec<u64> = Vec::new();

    // Fill first pole
    for i in 0..n {
        a.push(i);
    }

    if v >= 2 {
        println!("Filling inital vector with {} values", n);
    }

    // Flip it around
    a.reverse();

    // Return
    (a, b, c)
}


/// Recursive tower_of_hanoi;
fn tower_of_hanoi(n: u64, source: &mut Vec<u64>, target: &mut Vec<u64>, aux: &mut Vec<u64>, v: usize) {
    if n > 0 {
        // move n - 1 disk from source to auxillary so they are out of the way.
        tower_of_hanoi(n - 1, source, aux, target, v);

        // move the nth disk from source to target
        target.push(source.pop().unwrap());

        if v >= 1 {
            // Show our progress
            println!("============\nSource: {:?}\nTarget: {:?}\nAux: {:?}", source, target, aux);
        }

        // Move n - 1 disks left on aux back onto target
        tower_of_hanoi(n - 1, aux, target, source, v);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    /// Tests the setup code
    fn setup_works(){
        let n: u64 = 5;

        let (a, _, _) = setup(n, 0);

        let mut a_test = Vec::new();

        for i in 0..n {
            a_test.push(i);
        }

        a_test.reverse();

        assert_eq!(a_test, a);
    }

    #[bench]
    /// This benchmark takes an empty array and move a [0] into it.
    fn one_move(bench: &mut Bencher){
        let mut c: Vec<u64> = Vec::new();
        let a: Vec<u64> = vec![0];
        bench.iter(move || { 
            c.push(*a.get(0).unwrap());
        });
    }

    #[bench]
    /// Solves the puzzle with only one ring
    fn one_ring_hanoi(bench: &mut Bencher){
        bench.iter(move || { 
            tower_of_hanoi(1, &mut vec![0], &mut Vec::new(), &mut Vec::new(), 0);
        });
    }

    #[bench]
    /// Solves the puzzle with only one ring
    fn one_ring_hanoi_verbose(bench: &mut Bencher){
        bench.iter(move || { 
            tower_of_hanoi(1, &mut vec![0], &mut Vec::new(), &mut Vec::new(), 2);
        });
    }

    #[bench]
    /// Solves the puzzle with three rings
    fn three_ring_hanoi(bench: &mut Bencher){
        bench.iter(move || { 
            tower_of_hanoi(3, &mut vec![2,1,0], &mut Vec::new(), &mut Vec::new(), 0);
        });
    }

    #[bench]
    /// Solves the puzzle with three rings
    fn three_ring_hanoi_verbose(bench: &mut Bencher){
        bench.iter(move || { 
            tower_of_hanoi(3, &mut vec![2,1,0], &mut Vec::new(), &mut Vec::new(), 2);
        });
    }

    #[bench]
    /// Solve the puzzle with seven rings
    fn seven_ring_hanoi(bench: &mut Bencher){
        bench.iter(move || { 
            tower_of_hanoi(7, &mut vec![6,5,4,3,2,1,0], &mut Vec::new(), &mut Vec::new(), 0);
        });
    }

    #[bench]
    /// Solve the puzzle with seven rings
    fn seven_ring_hanoi_verbose(bench: &mut Bencher){
        bench.iter(move || { 
            tower_of_hanoi(7, &mut vec![6,5,4,3,2,1,0], &mut Vec::new(), &mut Vec::new(), 2);
        });
    }

}
