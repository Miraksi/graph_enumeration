use std::{collections::{HashMap, HashSet}, usize};

pub struct MaekinenEnum {
    delta: Vec<HashMap<char,usize>>,
    f: HashSet<usize>,
    s: Vec<Vec<usize>>,
    q_min: Vec<Vec<Option<(char,usize)>>>,
}
impl MaekinenEnum {
    pub fn new(delta: Vec<HashMap<char,usize>>, f: HashSet<usize>) -> Self {
        let len = delta.len();
        Self {
            delta,
            f,
            s: Vec::new(),
            q_min: vec![vec![None]; len],
        }
    }

    // former min_word
    fn init_q_min(&mut self, n: usize) {
        for (p, edges) in self.delta.iter().enumerate() {
            self.q_min[p].push(None);
            for (&a, &q) in edges.iter() {
                if !self.f.contains(&q) {
                    continue;
                }
                match self.q_min[p][1] {
                    Some((b, _)) if a >= b => {},
                    _ => self.q_min[p][1] = Some((a, q)),
                };
            }
        }
        for i in 2..=n {
            for (p, edges) in self.delta.iter().enumerate() {
                let mut min: Option<(char,usize)> = None;
                for (&a, &q) in edges.iter() {
                    if self.q_min[q][i - 1] == None {
                        continue;
                    }
                    match min {
                        None => min = Some((a,q)),
                        Some((b,_)) if a < b => min = Some((a,q)),
                        _ => {},
                    };
                }
                self.q_min[p].push(min);
            }
        }
    }

    // currently only for DFA
    fn next_word(&mut self, w: &String) -> Option<String> {
        let w_vec: Vec<char> = w.chars().collect();
        let n = w.len();
        for i in (1..=n).rev() {
            let s_last = self.s.pop().unwrap();
            let mut r = HashSet::new();
            for &p in s_last.iter() {
                for (&a, &q) in self.delta[p].iter() {
                    if let Some(_) = self.q_min[q][n-i] {
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
                        if let Some(_) = self.q_min[q][n-i] {
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

    fn min_word(&mut self, mut i: usize) -> Option<String> {
        let mut w = String::new();
        let mut s_last = self.s.pop().unwrap();
        let mut current_state = s_last[0];
        while i > 0 {
            let (a, q) = self.q_min[current_state][i].unwrap();
            let mut s_next = Vec::new();
            for &p in s_last.iter() {
                for (&b, &q) in self.delta[p].iter() {
                    if b != a {
                        continue;
                    }
                    if let Some(_) = self.q_min[q][i-1] {
                        s_next.push(q);
                    }
                    else if i == 1 && self.f.contains(&q) {
                        s_next.push(q);
                    }
                }
            }
            self.s.push(s_last);
            s_last = s_next;

            w += &String::from(a);
            current_state = q;
            i -= 1;
        }
        self.s.push(s_last);
        if w.len() == 0 {
            return None;
        }
        Some(w)
    }

    pub fn enum_cross_section(&mut self, n: usize) {
        self.init_q_min(n);
        self.s.push(vec![0]);
        let mut w = self.min_word(n);

        while let Some(v) = w {
            println!("{}", v);
            w = self.next_word(&v);
        }
    }
}