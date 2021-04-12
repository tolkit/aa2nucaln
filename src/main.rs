// A rust implementation of https://academic.oup.com/bioinformatics/article/31/11/1836/2365396
// Convert an amino acid alignment to a nucleotide alignment for HMM training.
// Prints to stdout only, nothing fancy.

use aa2nucaln::writer::writer;
use bio::io::fasta;
use clap::{value_t, App, Arg};

fn main() {
    // command line options
    let matches = App::new("aa2nucaln")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("Convert an amino acid alignment to a nucelotide alignment.\nFor more info, see: https://academic.oup.com/bioinformatics/article/31/11/1836/2365396")
        .arg(
            Arg::with_name("fasta")
                .short("f")
                .long("fasta")
                .takes_value(true)
                .required(true)
                .help("The reference amino acid alignment file in fasta format."),
        )
        .arg(
            Arg::with_name("ignore")
                .short("i")
                .long("ignore")
                .takes_value(true)
                .required(false)
                .default_value("true")
                .help("Should the lower-case letters in an amino acid alignment be ignored?"),
        )
        .get_matches();
    // parse command line options
    let input_fasta = matches.value_of("fasta").unwrap();
    let ignore = value_t!(matches.value_of("ignore"), bool).unwrap_or_else(|e| e.exit());

    // read in the fasta.
    let fasta_reader = fasta::Reader::from_file(input_fasta).expect("[-]\tPath invalid.");
    writer::write_faa(fasta_reader, ignore)
}
