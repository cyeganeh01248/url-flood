use clap::Parser;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Instant;

/// A tool for testing http flooding.
#[derive(Parser, Debug)]
struct Args {
    /// The full url to test.
    url: String,

    /// The number of threads to spin off.
    /// Ex: https://www.google.com/
    #[arg(short = 'j', long, default_value = "32")]
    num_threads: u16,

    /// The total number of requests to send.
    #[arg(short, long, default_value = "1000")]
    num_requests: u32,

    /// Whether to run endlessly or not.
    #[arg(short, long)]
    continuous: bool,
}

fn main() {
    let args = Args::parse();
    println!("Running URL-Flood with {args:?}");

    let count = Arc::new(Mutex::new(args.num_requests));
    let (tx, rx) = channel();

    let time_start = Instant::now();
    let mut handles = vec![];
    for _tid in 0..args.num_threads {
        let thread_url = args.url.clone();
        let thread_infinite = args.continuous;
        let thread_count = Arc::clone(&count);
        let thread_tx = tx.clone();
        handles.push(spawn(move || {
            let client = reqwest::blocking::Client::new();
            loop {
                if !thread_infinite {
                    let mut locked_count = thread_count.lock().unwrap();
                    if *locked_count == 0 {
                        break;
                    }
                    *locked_count -= 1;
                    drop(locked_count);
                }

                let resp = client.get(&thread_url).send();
                thread_tx.send(resp).expect("Unable to collect result.");
            }
        }));
    }
    drop(tx);
    let bar = ProgressBar::new(args.num_requests as u64);
    let mut results = HashMap::new();
    for r in rx {
        bar.inc(1);
        if args.continuous && bar.position() >= (args.num_requests as u64) {
            bar.reset()
        }
        let key = match r {
            Ok(resp) => resp.status().as_u16().to_string(),
            Err(_) => "err".to_string(),
        };
        let e = results.entry(key);
        e.and_modify(|v| *v += 1u32).or_insert_with(|| 1u32);
    }
    bar.finish();
    let dur = time_start.elapsed();
    println!(
        "Finished flooding {} with {} requests using {} threads in {:?} or {:?} per request.",
        args.url,
        args.num_requests,
        args.num_threads,
        dur,
        dur / args.num_requests
    );
    println!("Result Status Codes: {results:#?}");
    for handle in handles {
        let _r = handle.join().unwrap();
    }
}
