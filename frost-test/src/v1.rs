#[cfg(test)]
mod tests {
    use rand_core::OsRng;
    use wtfrost::{traits::Signer, v1};

    #[test]
    fn test() {
        let mut rng = OsRng::default();

        let ids = [1, 2, 3];
        let n: usize = 10;
        let t: usize = 7;

        let mut signer = v1::Signer::new(&ids, n, t, &mut rng);

        assert_eq!(signer.parties.len(), ids.len());
        signer.gen_nonces(&mut rng);

        let nonces = signer.gen_nonces(&mut rng);
        assert_eq!(nonces.len(), ids.len());
    }
}
