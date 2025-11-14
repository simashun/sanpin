mod cli;
use clap::Parser;
use cli::Args;
mod executor;
mod sound;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let args = Args::parse();

    println!("- Ping start -");
    println!("target: {}", args.target);
    if args.continuous {
        println!("count: continuous (-t)");
    } else {
        println!("count: {}", args.count);
    }
    println!("timeout: {}", args.timeout);

    // let mess = executor::run_ping(&args);
    // println!("{:?}", &mess);

    // match run_ping(&args) {
    //     Ok(stdout) => println!("{}",stdout),
    //     Err(e) => println!("{}", e)
    // }

    // Set up Ctrl-C handler: mark a flag so parent process won't print an extra error
    let running = Arc::new(AtomicBool::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            // on Ctrl-C, mark false; allow child process to handle termination
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    match executor::run_ping_realtime(&args) {
        Ok(_output) => {
            println!("\n- Ping end -");
        }

        Err(e) => {
            // If Ctrl-C was pressed, suppress printing the child-exit diagnostic
            if running.load(Ordering::SeqCst) {
                println!("\n--- エラー ---\n{}", e)
            }
        }
    }
}
