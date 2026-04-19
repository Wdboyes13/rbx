mod default;
mod helpers;
mod ui;

use crate::solve_store::SolveStore;
use crate::scramble::Seq;
use std::fs::File;
use stopwatch2::Stopwatch;

#[derive(Copy, Clone)]
enum View {
    Main,
    SolveViewer,
    Timer
}

pub struct RbxApp {
    current_scramble: Seq,
    sparm_len: usize,
    sparm_cx: i32,
    stime_buf: String,
    store_file: File,
    solve_store: SolveStore,
    scramble_vis: bool,
    ao5: Option<f64>,
    ao12: Option<f64>,
    best: f64,
    worst: f64,
    view: View,
    last_view: Option<View>,
    swatch: Stopwatch,
}