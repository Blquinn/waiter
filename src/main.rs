use std::net::TcpStream;
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "waiter", about = "Wait for a tcp port to open")]
struct Options {
    #[structopt()]
    /// The address to wait for.
    addr: String,

    #[structopt(short = "t", long = "timeout")]
    /// Optional timeout to stop waiting after in seconds.
    timeout: Option<u64>,

    #[structopt(short = "s", long = "success-message")]
    /// Optional message to display upon successful connection.
    success_message: Option<String>,

    #[structopt(short = "p", long = "poll-interval")]
    /// Optional poll interval in ms to check the addr.
    poll_interval: Option<u64>,
}

fn main() {
    let opts = Options::from_args();
    let start = Instant::now();
    let timeout = Duration::from_secs(opts.timeout.unwrap_or(0));
    let interval = Duration::from_millis(opts.poll_interval.unwrap_or(100));

    if interval > timeout {
        println!("You can't set poll-interval greater than timeout.");
        exit(1);
    }

    let mut opened = true;
    while TcpStream::connect(&opts.addr).is_err() {
        if opts.timeout.is_some() && start.elapsed() > timeout {
            opened = false;
            break;
        }
        sleep(interval);
    }

    if opened {
        println!("Connected to {} after {:#?}", opts.addr, start.elapsed());
        if let Some(msg) = opts.success_message {
            println!("{}", msg);
        }
    } else {
        println!("Addr {} didn't open after {:#?}", opts.addr, timeout);
        exit(3);
    }
}
