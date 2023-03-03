use blockstack_lib::address::b58;
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
        //TODO: Replace random scalar generation with appropriate function calls in wtfrost when data is exposed
        let mut rnd = OsRng::default();
        let private_key = b58::encode_slice(Scalar::random(&mut rnd).to_string().as_bytes());
        if let Some(filepath) = self.filepath {
            info!(
                "Writing private key to provided output file: {}",
                filepath.to_string_lossy()
            );
            let mut file = File::create(filepath)?;
            file.write_all(private_key.as_bytes())?;
            info!("Private key written succesfully.");
        } else {
            println!("{}", private_key);
        }
        Ok(())
    }
}
