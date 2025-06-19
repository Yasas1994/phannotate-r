use crate::orfs::Orf;
use crate::orfs::Orfs;
use crate::gcframe::GCFrame;
use crate::seq;
use std::collections::HashMap;

pub fn score_overlap(len: i64, direction: bool, pstop: f64) -> f64 {
    // same direction - True else False
    let o: f64 = 1.0 - pstop;
    let s: f64 = 0.05;
    let mut score: f64 = 1.0 / (o * len as f64);
    if direction == false {
        score += 1.0 / s;
    }
    score
}
    
pub fn score_gap(length: i32, direction: bool, pgap: f64) -> f64 {
    let g = 1.0 - pgap;
    let s = 0.05;

    if length > 300 {
        return g.powf(100.0) + length as f64;
    }

    let mut score = 1.0 / g.powf((length as f64) / 3.0);

    if direction == false {
        score += 1.0 / s;
    }

    score
}

fn average(vec: &Vec<f64>) -> Option<f64> {
    if vec.is_empty() {
        None
    } else {
        Some(vec.iter().sum::<f64>() / vec.len() as f64)
    }
}

pub fn score_rbs(seq: &str) -> u32 {
    let s: Vec<u8> = seq.as_bytes().iter().rev().copied().collect();

    let scan = |patterns: &[&[u8]], ranges: &[(usize, usize)]| {
        for &(start, end) in ranges {
            if end > s.len() { continue; }
            for pat in patterns {
                if &s[start..end] == *pat {
                    return true;
                }
            }
        }
        false
    };

    let p = |s: &str| s.to_ascii_uppercase().as_bytes().to_vec();

    if scan(&[&p("GGAGGA")], &[(5,11), (6,12), (7,13), (8,14), (9,15), (10,16)]) { return 27; }
    else if scan(&[&p("GGAGGA")], &[(3,9), (4,10)]) { return 26; }
    else if scan(&[&p("GGAGGA")], &[(11,17), (12,18)]) { return 25; }
    else if scan(&[&p("GGAGG")], &[(5,10), (6,11), (7,12), (8,13), (9,14), (10,15)]) { return 24; }
    else if scan(&[&p("GGAGG")], &[(3,8), (4,9)]) { return 23; }
    else if scan(&[&p("GAGGA")], &[(5,10), (6,11), (7,12), (8,13), (9,14), (10,15)]) { return 22; }
    else if scan(&[&p("GAGGA")], &[(3,8), (4,9)]) { return 21; }
    else if scan(&[&p("GAGGA"), &p("GGAGG")], &[(11,16), (12,17)]) { return 20; }
    else if scan(&[&p("GGACGA"), &p("GGATGA"), &p("GGAAGA"), &p("GGCGGA"), &p("GGGGGA"), &p("GGTGGA")], &[(5,11), (6,12), (7,13), (8,14), (9,15), (10,16)]) { return 19; }
    else if scan(&[&p("GGAAGA"), &p("GGATGA"), &p("GGACGA"), &p("GGTGGA"), &p("GGGGGA"), &p("GGCGGA")], &[(3,9), (4,10)]) { return 18; }
    else if scan(&[&p("GGAAGA"), &p("GGATGA"), &p("GGACGA"), &p("GGTGGA"), &p("GGGGGA"), &p("GGCGGA")], &[(11,17), (12,18)]) { return 17; }
    else if scan(&[&p("GGAG"), &p("GAGG")], &[(5,9), (6,10), (7,11), (8,12), (9,13), (10,14)]) { return 16; }
    else if scan(&[&p("AGGA")], &[(5,9), (6,10), (7,11), (8,12), (9,13), (10,14)]) { return 15; }
    else if scan(&[&p("GGTGG"), &p("GGGGG"), &p("GGCGG")], &[(5,10), (6,11), (7,12), (8,13), (9,14), (10,15)]) { return 14; }
    else if scan(&[&p("AGG"), &p("GAG"), &p("GGA")], &[(5,8), (6,9), (7,10), (8,11), (9,12), (10,13)]) { return 13; }
    else if scan(&[&p("AGGA"), &p("GAGG"), &p("GGAG")], &[(11,15), (12,16)]) { return 12; }
    else if scan(&[&p("AGGA"), &p("GAGG"), &p("GGAG")], &[(3,7), (4,8)]) { return 11; }
    else if scan(&[&p("GAGGA"), &p("GGAGG")], &[(13,18), (14,19), (15,20)]) || scan(&[&p("GGAGGA")], &[(13,19), (14,20), (15,21)]) { return 10; }
    else if scan(&[&p("GAAGA"), &p("GATGA"), &p("GACGA")], &[(5,10), (6,11), (7,12), (8,13), (9,14), (10,15)]) { return 9; }
    else if scan(&[&p("GGTGG"), &p("GGGGG"), &p("GGCGG")], &[(3,8), (4,9)]) { return 8; }
    else if scan(&[&p("GGTGG"), &p("GGGGG"), &p("GGCGG")], &[(11,16), (12,17)]) { return 7; }
    else if scan(&[&p("AGG"), &p("GAG"), &p("GGA")], &[(11,14), (12,15)]) { return 6; }
    else if scan(&[&p("GAAGA"), &p("GATGA"), &p("GACGA")], &[(3,8), (4,9)]) { return 5; }
    else if scan(&[&p("GAAGA"), &p("GATGA"), &p("GACGA")], &[(11,16), (12,17)]) { return 4; }
    else if scan(&[&p("AGGA"), &p("GAGG"), &p("GGAG")], &[(13,17), (14,18), (15,19)]) { return 3; }
    else if scan(&[&p("AGG"), &p("GAG"), &p("GGA")], &[(13,16), (14,17), (15,18)]) { return 2; }
    else if scan(&[&p("GGAAGA"), &p("GGATGA"), &p("GGACGA")], &[(13,19), (14,20), (15,21)]) { return 2; }
    else if scan(&[&p("GGTGG"), &p("GGGGG"), &p("GGCGG")], &[(13,18), (14,19), (15,20)]) { return 2; }
    else if scan(&[&p("AGG"), &p("GAG"), &p("GGA")], &[(3,6), (4,7)]) { return 1; }

    0
}


pub fn get_orfs<'a>(
    seq: &'a [u8],
    start_codons: Option<&'a [&'a str]>,
    stop_codons: Option<&'a [&'a str]>,
) -> Orfs {
    let start = start_codons.unwrap_or(&["ATG", "GTG", "TTG"]);
    let stop = stop_codons.unwrap_or(&["TAG", "TGA", "TAA"]);
    let mut open_reading_frames : Orfs;
    open_reading_frames.seq = String::from_utf8_lossy(seq).to_string();
    open_reading_frames.contig_length = seq.len();


    
    // use `start` and `stop`...
   open_reading_frames
}

fn process_dna(dna: &str) -> (HashMap<char, u32>, Vec<f64>) {
    let mut frequency: HashMap<char, u32> = HashMap::from([
        ('A', 0), ('T', 0), ('C', 0), ('G', 0),
    ]);
    let mut background_rbs = vec![1.0; 28];
    let mut training_rbs = vec![1.0; 28]; // unused in the Python version
    let mut frame_plot = GCFrame::new(dna.len());

    let dna_chars: Vec<char> = dna.chars().collect();

    for (i, &base_raw) in dna_chars.iter().enumerate() {
        let mut base = match base_raw.to_ascii_uppercase() {
            'A' | 'T' | 'C' | 'G' => base_raw,
            'S' | 'B' | 'V' => 'G',
            _ => 'A',
        };

        // Nucleotide frequency
        *frequency.entry(base).or_insert(0) += 1;
        *frequency.entry(seq::revcomp_base(&base)).or_insert(0) += 1;

        // RBS scoring with 21-mer
        if i + 21 <= dna.len() {
            let kmer: String = dna_chars[i..i+21].iter().collect();
            let rev_kmer = seq::revcomp_kmer(&kmer);

            let idx = score_rbs(&kmer);
            if idx < background_rbs.len() {
                background_rbs[idx] += 1.0;
            }

            let idx_rc = score_rbs(&rev_kmer);
            if idx_rc < background_rbs.len() {
                background_rbs[idx_rc] += 1.0;
            }
        }

        // GC frame plot
        frame_plot.add_base(base);
    }

    let gc_pos_freq = frame_plot.get();

    (frequency, gc_pos_freq)
}