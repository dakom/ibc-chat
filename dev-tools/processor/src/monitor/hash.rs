use std::path::{Path, PathBuf};
use ring::digest::{Context, SHA256};
use tokio::fs;
use tokio::io::AsyncReadExt;
use anyhow::Result;

pub async fn hash_files(paths: &[PathBuf]) -> Result<String> {
    // create a new context for the hash computation
    let mut context = Context::new(&SHA256);

    // for each path, add the file's contents to the hash
    for path in paths {
        hash_file(&path, &mut context).await?;
    }

    // Finalize the hash computation and get the digest.
    let digest = context.finish();

    Ok(hex::encode(digest.as_ref()))
}

async fn hash_file<P: AsRef<Path>>(path: P, context: &mut Context) -> Result<()> {
    // Open the file asynchronously.
    let mut file = fs::File::open(path).await?;
    // Create a new SHA256 context to compute the hash.
    // Buffer to hold chunks of file data.
    let mut buffer = [0; 4096];
    
    // Read the file in a loop until all data is processed.
    loop {
        // Read a chunk of the file into the buffer.
        let count = file.read(&mut buffer).await?;
        // If count is 0, it means we've reached the end of the file.
        if count == 0 {
            return Ok(());
        }
        // Update the hash context with the data read.
        context.update(&buffer[..count]);
    }
}