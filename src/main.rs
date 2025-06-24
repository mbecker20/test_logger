use std::time::Duration;

use rand::Rng;
use tokio::signal;
use tracing::info;

async fn app() {
  info!("version: v{}", env!("CARGO_PKG_VERSION"));

  let mut rng = rand::rng();

  loop {
    let num = rng.random_range(0..3);

    match num {
      0 => tracing::info!("this is an INFO log ✅"),
      1 => tracing::warn!("this is a WARN log 🛑"),
      2 => tracing::error!("this is an ERROR log 🚨"),
      _ => unreachable!(),
    }

    tokio::time::sleep(Duration::from_secs(5)).await;
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();

  let mut term_handle = signal::unix::signal(signal::unix::SignalKind::terminate())?;

  tokio::select! {
    _ = app() => {},
    _ = term_handle.recv() => {}
  }

  Ok(())
}
