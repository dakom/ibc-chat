use std::future::Future;
use std::path::PathBuf;
use anyhow::{bail, Result};
use tokio::sync::mpsc;

use notify::{Event, Config, EventKind, RecommendedWatcher, Watcher};

pub async fn watch_files<F, A>(files: Vec<PathBuf>, on_change: F) -> Result<()> 
where 
    F: Fn(Event) -> A,
    A: Future<Output = Result<()>>,

{
    let (tx, mut rx) = mpsc::channel(256);

    // https://github.com/notify-rs/notify/issues/380#issuecomment-1250468496
    let mut watcher = RecommendedWatcher::new(move |res| {
        tx.blocking_send(res).unwrap();
    }, Config::default())?;

    for file in files {
        // we already know the exact path, so no need for additional recursion
        // and in fact recursion wouldn't help us since we needed exact paths
        // built from local deps
        watcher.watch(file.as_path(), notify::RecursiveMode::NonRecursive)?;
    }

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                        on_change(event).await?;
                    } 
                    _ => println!("Other event: {:?}", event.paths),
                }
            },
            Err(e) => bail!("Watcher error: {:?}", e),
        }
    }

    println!("finished watching");
    Ok(())
}