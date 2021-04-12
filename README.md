# aa2nucaln

Catchy name there. Rust implementation of the amino acid alignment to a nucleotide alignment described here: https://academic.oup.com/bioinformatics/article/31/11/1836/2365396.

For use with hmmer.

## Usage

`aa2nucaln ./fastas/TERT_thumb.faa > TERT_thumb.fna`

Output is to stdout, so capture with `>`.

I also provide a bash script for reproducibility in the creation of the hmm files for use in <a href="https://github.com/tolkit/telomeric-identifier">tidk</a>