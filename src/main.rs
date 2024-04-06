use std::{thread::sleep, time::Duration};

use rand::{thread_rng, Rng};

fn main() {
  tracing_subscriber::fmt::init();

  let mut rng = thread_rng();

  loop {
    let num = rng.gen_range(0..3);

    match num {
      0 => tracing::info!("this is an INFO log ✅"),
      1 => tracing::warn!("this is a WARN log 🛑"),
      2 => tracing::error!("this is an ERROR log 🚨"),
      _ => unreachable!(),
    }
		
    sleep(Duration::from_secs(5));
  }
}
