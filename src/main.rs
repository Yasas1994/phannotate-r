mod edges;
mod node;
pub mod orfs;
pub mod gcframe;
pub mod seq;
mod misc;
use needletail::parse_fastx_file;
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;
use std::collections::HashMap;
use rust_decimal::Decimal;

/// Valid output formats
#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Tabular,
    Genbank,
    Fasta,
}

/// Argument parser struct
#[derive(Parser, Debug)]
#[command(name = "phanotate", version = "1.0", 
          about = "PHANOTATE: A phage genome annotator",
          author = "Katelyn McNair")]
struct Args {
    /// Input FASTA file
    infile: String,

    /// Output file [default: stdout]
    #[arg(short, long, default_value = "-", value_parser = clap::value_parser!(String))]
    outfile: String,

    /// Output format
    #[arg(short = 'f', long, default_value = "tabular")]
    format: OutputFormat,

    /// Start codons and weights (e.g., "atg:0.85,gtg:0.10,ttg:0.05")
    #[arg(short = 's', long, default_value = "ATG:0.85,GTG:0.10,TTG:0.05")]
    start_codons: String,

    /// Stop codons (e.g., "tag,tga,taa")
    #[arg(short = 'e', long, default_value = "TAG,TGA,TAA")]
    stop_codons: String,

    /// Minimum ORF length
    #[arg(short = 'l', long, default_value_t = 90)]
    minlen: usize,

    /// Dump intermediate results
    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue)]
    dump: bool,
}

fn parse_start_codons(raw: &str) -> HashMap<String, Decimal> {
    let mut codons = HashMap::new();
    for pair in raw.split(',') {
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() == 2 {
            let codon = parts[0].to_uppercase();
            let weight = Decimal::from_str(parts[1]).unwrap();
            codons.insert(codon, weight);
        }
    }
    // Normalize by max
    if let Some(max) = codons.values().cloned().max() {
        for v in codons.values_mut() {
            *v /= max;
        }
    }
    codons
}

fn parse_stop_codons(raw: &str) -> Vec<String> {
    raw.split(',').map(|s| s.to_uppercase()).collect()
}

fn main() {
    let args = Args::parse();

    // Open output file or default to stdout
    let mut writer: Box<dyn Write> = if args.outfile == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(&args.outfile).expect("Unable to create output file"))
    };

    let start_codons = parse_start_codons(&args.start_codons);
    let stop_codons = parse_stop_codons(&args.stop_codons);

    // Example debug print
    writeln!(writer, "Parsed args:\n{:?}", args).unwrap();
    writeln!(writer, "Start codons: {:?}", start_codons).unwrap();
    writeln!(writer, "Stop codons: {:?}", stop_codons).unwrap();
    // let mut reader = parse_fastx_file(&args.infile).expect("Failed to open FASTA file");
    
    // while let Some(record) = reader.next() {
    //     let seqrec = record.expect("Failed to read record");
    //     println!(">{}_{}", String::from_utf8_lossy(seqrec.id()), seqrec.seq().len());
    //     println!("{:?}", seqrec.seq());
    // }
    let overlap_score = misc::score_gap(1203, false, 0.4);
    println!("Overlap score: {}", overlap_score);
}
