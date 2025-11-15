use std::collections::HashMap;

pub fn shannon_entropy_bits(s: &str) -> f64 {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let len = s.chars().count();

    if len == 0 {
        return 0.0;
    }

    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let len_f = len as f64;
    let mut entropy = 0.0;

    for (_ch, count) in counts {
        let p = count as f64 / len_f;
        entropy -= p * p.log2();
    }

    entropy
}

pub fn entropy_total_bits(s: &str) -> f64 {
    shannon_entropy_bits(s) * (s.chars().count() as f64)
}