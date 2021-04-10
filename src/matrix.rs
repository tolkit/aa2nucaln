pub mod matrix {

    use crate::codons::codons;
    // https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
    // would be nice to avoid clone but whatever.

    pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        assert!(!v.is_empty());
        (0..v[0].len())
            .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
            .collect()
    }

    pub fn create_matrix(aa: &[u8]) -> Vec<Vec<&str>> {
        let mut vec = Vec::new();
        for a in aa {
            vec.push(codons::get_codon_string(*a));
        }
        vec
    }
}
