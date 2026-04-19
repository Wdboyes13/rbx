use stopwatch2::Stopwatch;

use crate::app::{RbxApp, View};
use crate::solve_store::SolveStore;
use crate::display_error;
use std::io::Read;
use crate::scramble::Seq;

impl Default for RbxApp {
    fn default() -> Self {
        let home = std::env::home_dir().unwrap();
        let mut store_file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(format!("{}/.rbx_data", home.to_str().unwrap()))
            .unwrap();

        let mut buf = Vec::new();
        if store_file.read_to_end(&mut buf).is_err() {
            display_error("Store error", "Failed to read store");
        }

        let solve_store = if buf.is_empty() {
            SolveStore::default()
        } else {
            match SolveStore::from_binary(buf) {
                Ok(store) => store,
                Err(e) => {
                    display_error("Store error", "Failed to parse store");
                    panic!("{}", e);
                }
            }
        };

        let ao5 = solve_store.avg(5, true);
        let ao12 = solve_store.avg(12, true);
        let best = solve_store.best_time();
        let worst = solve_store.worst_time();

        Self {
            current_scramble: Seq::generate(20, 3),
            sparm_len: 20,
            sparm_cx: 3,
            stime_buf: String::new(),
            store_file,
            solve_store,
            scramble_vis: true,
            ao5,
            ao12,
            best,
            worst,
            view: View::Main,
            last_view: None,
            swatch: Stopwatch::default()
        }
    }
}