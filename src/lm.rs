use std::{collections::{HashMap, HashSet}, usize};
use ndarray::{Array, Ix2};
use ndarray::prelude::*;

pub struct LMEnum {
    pub delta: Vec<HashMap<char,usize>>,
    pub m: Vec<Array<usize, Ix2>>,
    f: HashSet<usize>,
    pub s: Vec<Vec<usize>>,
}

impl LMEnum {
    pub fn new(delta: Vec<HashMap<char,usize>>, f: HashSet<usize>, m: Array<usize, Ix2>) -> Self {
        Self {
            delta,
            f,
            s: Vec::new(),
            m: vec![m],
        }
    }

    pub fn init_m(&mut self, n: usize) {
        while self.m.len() < n {
            self.m.push(self.m.last().unwrap().dot(self.m.get(0).unwrap()));
        }
    }

    pub fn min_word(&mut self, n: usize) -> Option<String> {
        self.init_m(n);
        let mut is_empty = true;
        for ((_p, q), &val) in self.m.last().unwrap().indexed_iter() {
            if val != 0 && self.f.contains(&q) {
                is_empty = false;
            }
        }
        if is_empty {
            return None;
        }
        let mut w = String::new();
        for i in 0..n {
            // println!("i: {i}\t {:?}", self.s.last());
            let s_last = self.s.last().unwrap();
            let mut min = None;
            for &u in s_last.iter() {
                for (&a, v) in self.delta[u].iter() {
                    if i == n-1 {
                        if self.f.contains(v) {
                            match min {
                                Some(b) if a < b => min = Some(a),
                                None => min = Some(a),
                                _ => {}, 
                            };
                        }
                        continue;
                    }

                    let m_power = self.m.get(n-i-2).unwrap();
                    for (f, &val) in m_power.slice(s![*v, ..]).iter().enumerate() {
                        // println!("u: {u} v: {} (val: {val},f: {f})", v);
                        if val == 0 || !self.f.contains(&f) {
                            continue;
                        }
                        match min {
                            Some(b) if a < b => min = Some(a),
                            None => min = Some(a),
                            _ => {}, 
                        };
                    }
                }
            }
            w += &format!("{}", min.unwrap());
            let mut s_next = Vec::new();

            for &u in s_last.iter() {
                for (&a, &v) in self.delta[u].iter() {
                    if a != min.unwrap() {
                        continue;
                    }
                    if i == n-1 {
                        if self.f.contains(&v) {
                            s_next.push(v);
                        }
                        continue;
                    }

                    let m_power = self.m.get(n-i-2).unwrap();
                    for (f, &val) in m_power.slice(s![v, ..]).iter().enumerate() {
                        if val != 0 && self.f.contains(&f) {
                            s_next.push(v);
                        }
                    }
                }
            }
            if i != n - 1 {
                self.s.push(s_next);
            }
        }
        Some(w)
    }

    fn is_complete(&self, i: usize, q: usize) -> bool {
        if i == 0 {
            return self.f.contains(&q)
        }
        else {
            let m_power = self.m.get(i-1).unwrap();
            for (f, &val) in m_power.slice(s![q, ..]).iter().enumerate() {
                if val != 0 && self.f.contains(&f) {
                    return true;
                }
            }
        }
        false
    }

    fn next_word(&mut self, w: &String) -> Option<String> {
        let w_vec: Vec<char> = w.chars().collect();
        let n = w.len();
        for i in (1..=n).rev() {
            let s_last = self.s.pop().unwrap();
            let mut r = HashSet::new();
            for &p in s_last.iter() {
                for (&a, &q) in self.delta[p].iter() {
                    if self.is_complete(n-i, q) {
                        r.insert((a,q));
                    }
                }
            }
            let mut min = None;
            for (a, _) in r.iter() {
                if *a <= w_vec[i-1] {
                    continue;
                }
                match min {
                    None => min = Some(*a),
                    Some(b) if *a < b => min = Some(*a),
                    _ => {},
                };
            }
            if let Some(b_i) = min {
                let mut s_next = Vec::new();
                for &p in s_last.iter() {
                    for (&a, &q) in self.delta[p].iter() {
                        if a != b_i {
                            continue;
                        }
                        if self.is_complete(n-1, q) {
                            s_next.push(q);
                        }
                    }
                }
                self.s.push(s_last);
                if i-1 != n {
                    self.s.push(s_next);
                }
                let w_rest = self.min_word(n-i).unwrap();
                return Some(w[0..i-1].to_string() + &String::from(b_i) + &w_rest);
            }
        }
        None
    }

    pub fn enum_cross_section(&mut self, n: usize) {
        self.s.push(vec![0]);
        let mut w = self.min_word(n);

        while let Some(v) = w {
            println!("{}", v);
            w = self.next_word(&v);
        }
    }
}
