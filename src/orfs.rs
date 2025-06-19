use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Orf<'a> {
    pub start: usize,
    pub stop: usize,
    pub length: usize,
    pub frame: isize,
    pub seq: &'a [u8],
    pub rbs: String,
    pub rbs_score: f64,
    pub weight: f64,
    pub weight_start: f64,
    pub weight_rbs: f64,
    pub hold: f64,
    pub gcfp_mins: f64,
    pub gcfp_maxs: f64,
    pub start_codons: Vec<String>,
    pub stop_codons: Vec<String>,
}

impl<'a> Orf<'a> {
    pub fn new(start: usize, stop: usize, length: usize, frame: isize, seq: &'a [u8], rbs: String, rbs_score: f64, start_codons: Vec<String>, stop_codons: Vec<String>) -> Self {
        Self {
            start,
            stop,
            length,
            frame,
            seq,
            rbs,
            rbs_score,
            weight: 1.0,
            weight_start: 1.0,
            weight_rbs: 1.0,
            hold: 1.0,
            gcfp_mins: 1.0,
            gcfp_maxs: 1.0,
            start_codons,
            stop_codons,
        }
    }

    pub fn start_codon(&self) -> &str {
        std::str::from_utf8(&self.seq[0..3]).unwrap()
    }

    pub fn stop_codon(&self) -> &str {
        std::str::from_utf8(&self.seq[self.seq.len()-3..]).unwrap()
    }

    pub fn has_start(&self) -> bool {
        self.start_codons.contains(&self.start_codon().to_string())
    }

    pub fn has_stop(&self) -> bool {
        self.stop_codons.contains(&self.stop_codon().to_string())
    }

    pub fn score(&mut self, start_codon_weight: &HashMap<String, f64>) {
        let mut s = 1.0 / self.hold;
        if let Some(w) = start_codon_weight.get(self.start_codon()) {
            s *= w;
        }
        s *= self.weight_rbs;
        self.weight = -s;
    }
}

impl<'a> fmt::Display for Orf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Orf({}, {}, {}, {}, {}, {})",
            self.start, self.stop, self.frame, self.weight_rbs, self.weight_rbs, self.weight)
    }
}

pub struct Orfs<'a> {
    pub data: HashMap<usize, HashMap<usize, Orf<'a>>>,
    pub other_end: HashMap<usize, usize>,
    pub start_codons: Vec<String>,
    pub stop_codons: Vec<String>,
    pub min_orf_len: usize,
    pub contig_length: usize,
    pub seq: String,
}

impl<'a> Orfs<'a> {
    pub fn new(min_orf_len: usize, start_codons: Vec<String>, stop_codons: Vec<String>) -> Self {
        Self {
            data: HashMap::new(),
            other_end: HashMap::new(),
            start_codons,
            stop_codons,
            min_orf_len,
            contig_length: 0,
            seq: String::new(),
        }
    }

    pub fn add_orf(&mut self, start: usize, stop: usize, length: usize, frame: isize, seq: &'a [u8], rbs: String, rbs_score: f64) {
        let orf = Orf::new(start, stop, length, frame, seq, rbs, rbs_score, self.start_codons.clone(), self.stop_codons.clone());
        self.data.entry(stop).or_insert_with(HashMap::new).insert(start, orf);
        self.other_end.insert(start, stop);
        self.other_end.insert(stop, start);
    }

    pub fn iter_orfs(&self) -> impl Iterator<Item = &Orf<'a>> {
        self.data.values().flat_map(|map| map.values())
    }

    pub fn get_orf(&self, start: usize, stop: usize) -> Option<&Orf<'a>> {
        self.data.get(&stop)?.get(&start)
    }
}