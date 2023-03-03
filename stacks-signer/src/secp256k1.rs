use blockstack_lib::util::secp256k1::Secp256k1PrivateKey;
use clap::Args;
use std::{fs::File, io::prelude::*, path::PathBuf};
use tracing::info;

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
        // TODO: May be able to directly use wtfrost when the appropriate bytes are exposed
        let private_key = Secp256k1PrivateKey::new().to_hex();
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
