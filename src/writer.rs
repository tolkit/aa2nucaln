pub mod writer {
    use crate::matrix::matrix;
    use std::io::Read;

    pub fn write_faa<R: Read>(faa: bio::io::fasta::Reader<R>) {
        for record in faa.records() {
            let fasta_record = record.expect("[-]\tError during fasta record parsing.");
            let matrix = matrix::create_matrix(fasta_record.seq());
            let t_mat = matrix::transpose(matrix);
            for (i, seq) in t_mat.iter().enumerate() {
                println!(">{}:{}\n{}", fasta_record.id(), i + 1usize, seq.join(""));
            }
        }
    }
}
