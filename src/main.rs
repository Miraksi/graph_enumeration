mod maekinen;
mod lm;

use std::{collections::{HashMap, HashSet}, usize};
use maekinen::MaekinenEnum;
use lm::LMEnum;
use ndarray::array;

fn main() {

    println!("Lookahead Matrix:");
    test_lm();

    println!("Mäkinen:");
    test_maekinen();
}

fn test_lm() {
    let mut delta = vec![HashMap::new(), HashMap::new()];
    delta[0].insert('0', 0);
    delta[0].insert('1', 1);
    delta[1].insert('1', 1);
    let mut f: HashSet<usize> = HashSet::new();
    f.insert(1);
    let m = array![[1, 1],[0, 1]];

    let mut enumerator = LMEnum::new(delta, f, m);
    enumerator.enum_cross_section(4);
}

fn test_maekinen() {
    let mut delta = vec![HashMap::new(), HashMap::new()];
    delta[0].insert('0', 0);
    delta[0].insert('1', 1);
    delta[1].insert('1', 1);
    let mut f: HashSet<usize> = HashSet::new();
    f.insert(1);

    let mut enumerator = MaekinenEnum::new(delta, f);
    // enumerator.init_q_min(3);
    // println!("{:?}", enumerator.q_min);

    enumerator.enum_cross_section(4)
    // enumerator.s.push(vec![0]);
    // println!("{:?}", enumerator.min_word(3));
}