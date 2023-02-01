#[cfg(test)]
mod tests {
    use std::time;

    use frost::v1::Party;
    use hashbrown::HashMap;
    use rand_core::OsRng;

    #[test]
    fn pure_frost() {
        // let num_sigs = 7;
        // let num_nonces = 5;
        let N: usize = 10;
        let T = (N * 2) / 3;


        let mut rng = rand_core::OsRng::default();

        //
        let party = Party::new(1, N, T, &mut rng);
    }
}
