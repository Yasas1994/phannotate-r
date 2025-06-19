use needletail::Sequence;

pub fn revcomp_seq(seq: &[u8]) {
    let rc = seq.reverse_complement();
    println!("RevComp: {}", String::from_utf8_lossy(&rc));
}

pub fn revcomp_base(base: &char) -> char {
    match base.to_ascii_lowercase() {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => 'N', // fallback for ambiguous/invalid bases
    }
}

pub fn revcomp_kmer(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|base| revcomp_base(&base))
        .collect()
}