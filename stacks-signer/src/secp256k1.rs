use clap::Args;
use rand_core::OsRng;
use std::{fs::File, io::prelude::*, path::PathBuf};
use tracing::info;
use wtfrost::Scalar;

#[derive(Args)]
pub struct Secp256k1 {
    #[arg(short, long)]
    /// Path to output generated private Secp256k1 key
    filepath: Option<PathBuf>,
}

impl Secp256k1 {
    /// Generate a random Secp256k1 private key
    pub fn generate_private_key(self) -> std::io::Result<()> {
        info!("Generating a new private key.");
        let mut rnd = OsRng::default();
        let private_key = Scalar::random(&mut rnd);
        if let Some(filepath) = self.filepath {
            info!(
                "Writing private key to provided output file: {}",
                filepath.to_string_lossy()
            );
            let mut file = File::create(filepath)?;
            file.write_all(private_key.to_string().as_bytes())?;
            info!("Private key written successfully.");
        } else {
            println!("{}", private_key);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::secp256k1::Secp256k1;
    use testdir::testdir;

    #[test]
    fn generate_private_key() {
        let mut filepath = testdir!();
        filepath.push(".priv_key");
        assert!(!filepath.exists());

        let secp256k1 = Secp256k1 {
            filepath: Some(filepath.clone()),
        };
        secp256k1.generate_private_key().unwrap();
        assert!(filepath.exists());
    }
}
