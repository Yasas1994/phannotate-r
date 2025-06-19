use std::collections::{HashMap, VecDeque};

pub struct GCFrame {
    window: usize,
    states: Box<dyn Iterator<Item = usize>>,
    bases: [Option<VecDeque<char>>; 4],
    frequency: [Option<HashMap<char, usize>>; 4],
    total: [VecDeque<usize>; 4],
    freq: Vec<[usize; 3]>,
}

impl GCFrame {
    pub fn new(window: usize) -> Self {
        let window = window / 3;

        let mut bases: [Option<VecDeque<char>>; 4] = Default::default();
        let mut frequency: [Option<HashMap<char, usize>>; 4] = Default::default();

        for frame in 1..=3 {
            bases[frame] = Some(VecDeque::from(vec!['-'; window]));
            let mut freq_map = HashMap::new();
            for &base in ['A', 'T', 'C', 'G', '-'].iter() {
                freq_map.insert(base, 0);
            }
            frequency[frame] = Some(freq_map);
        }

        let states = Box::new((1..=3).cycle());

        GCFrame {
            window,
            states,
            bases,
            frequency,
            total: Default::default(),
            freq: Vec::new(),
        }
    }

    pub fn add_base(&mut self, base: char) {
        let frame = self.states.next().unwrap();
        if let (Some(bases_frame), Some(freq_map)) =
            (&mut self.bases[frame], &mut self.frequency[frame])
        {
            bases_frame.push_back(base);
            *freq_map.entry(base).or_insert(0) += 1;

            let removed = bases_frame.pop_front().unwrap();
            *freq_map.entry(removed).or_insert(0) -= 1;

            let gc_count = freq_map.get(&'G').unwrap_or(&0) + freq_map.get(&'C').unwrap_or(&0);
            self.total[frame].push_back(gc_count);
        }
    }

    fn close(&mut self) {
        for _ in 0..(self.window / 2) {
            for frame in 1..=3 {
                if let (Some(bases_frame), Some(freq_map)) =
                    (&mut self.bases[frame], &mut self.frequency[frame])
                {
                    bases_frame.pop_front();
                    let removed = bases_frame.pop_front().unwrap_or('-');
                    *freq_map.entry(removed).or_insert(0) -= 1;
                    let gc_count = freq_map.get(&'G').unwrap_or(&0) + freq_map.get(&'C').unwrap_or(&0);
                    self.total[frame].push_back(gc_count);
                }
            }
        }
    }

    pub fn get(&mut self) -> &Vec<[usize; 3]> {
        self.close();
        self.freq.push([20, 20, 20]);

        let len = self.total[3].len().saturating_sub(1);
        for i in 0..len {
            let t1 = self.total[1][i];
            let t2 = self.total[2][i];
            let t3 = self.total[3][i];
            let t1n = self.total[1].get(i + 1).copied().unwrap_or(0);
            let t2n = self.total[2].get(i + 1).copied().unwrap_or(0);

            self.freq.push([t1, t2, t3]);
            self.freq.push([t2, t3, t1n]);
            self.freq.push([t3, t1n, t2n]);
        }

        if let Some(i) = self.total[3].len().checked_sub(1) {
            let t1 = self.total[1][i];
            let t2 = self.total[2][i];
            let t3 = self.total[3][i];
            self.freq.push([t1, t2, t3]);

            if let Some(t1n) = self.total[1].get(i + 1) {
                self.freq.push([t2, t3, *t1n]);
            }

            if let Some(t2n) = self.total[2].get(i + 1) {
                self.freq.push([t3, self.total[1].get(i + 1).copied().unwrap_or(0), *t2n]);
            }
        }

        &self.freq
    }
}

pub fn max_idx(a: usize, b: usize, c: usize) -> usize {
    if a > b {
        if a > c {
            1
        } else {
            3
        }
    } else {
        if b > c {
            2
        } else {
            3
        }
    }
}

pub fn min_idx(a: usize, b: usize, c: usize) -> usize {
    if a > b {
        if b > c {
            3
        } else {
            2
        }
    } else {
        if a > c {
            3
        } else {
            1
        }
    }
}
