use argh::FromArgs;
use std::{
    sync::mpsc,
    thread,
    error::Error,
    time::{Duration, Instant},
    io::{ Stderr, stderr },
};
use crossterm::{
    execute,
    terminal::{
        EnterAlternateScreen,
        enable_raw_mode,
        disable_raw_mode
    },
};
use tui::{
    Terminal, 
    backend::{CrosstermBackend, Backend},
};

#[argh(description = "...")]
#[derive(Debug, FromArgs)]
struct Cli {

    #[argh(positional)]
    path: Option<String>,

    #[argh(option, default = "250", short = 't', description = "...")]
    tick_rate: u64,

    #[argh(switch, short = 'k', description = "...")]
    keep: bool,

    #[argh(switch, short = 'd', description = "...")]
    dirs: bool,
}

fn out() -> Stderr { stderr() }

fn main() {
    let cli: Cli = argh::from_env();

    enable_raw_mode();
    let mut out = out();
    execute!(out, EnterAlternateScreen);

    let backend = CrosstermBackend::new(out);
    let terminal = CrosstermBackend::new(backend);

    let (tx, rx) = mpsc::channel();
    let (tx_stop_thread, rx_stop_thread) = mpsc::channel();

    println!("{:?}", cli)
}