use std::{
    fmt::Write,
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use notify_rust::Notification;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    seconds: u64,
}

fn main() {
    let args = Args::parse();

    let duration = std::time::Duration::from_secs(args.seconds);
    let pb = init_progress_bar(duration);
    let start_time = Instant::now();

    loop {
        let delta = Instant::now().duration_since(start_time);
        // it's so over
        if delta > duration {
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        pb.inc(1);
    }

    finish(pb);
}

fn init_progress_bar(duration: Duration) -> ProgressBar {
    let pb = ProgressBar::new(duration.as_secs());
    pb.set_style(
        ProgressStyle::with_template(
            "⏰ [{elapsed_precise}] {spinner:.bold.bright.yellow} {bar:40.239/black} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            let elapsed = state.eta().as_secs_f64();
            if elapsed != 0.0 {
                write!(w, "{:.1}s", elapsed).unwrap()
            } else {
                write!(w, "done").unwrap()
            }
        }),
    );
    pb.set_message("Counting down");

    pb
}

fn finish(pb: ProgressBar) {
    pb.finish_with_message("Finished");
    Notification::new()
        .summary("⏰")
        .body("Countdown finished!")
        .show()
        .unwrap();
}
