use anyhow::Result;
use std::io::{SeekFrom, Seek, Write};
use chrono::{Local, TimeZone, Utc};
use crate::{
    scramble::Seq,
    solve_store::Solve,
    app::RbxApp,
    display_error
};

impl RbxApp {
    pub fn rescramble(&mut self) {
        self.current_scramble = Seq::generate(self.sparm_len, self.sparm_cx);
    }

    pub fn write_store(&mut self) -> Result<()> {
        let data = self.solve_store.to_binary()?;
        self.store_file.seek(SeekFrom::Start(0))?;
        self.store_file.set_len(0)?;
        self.store_file.write_all(data.as_slice())?;
        self.store_file.flush()?;
        Ok(())
    }

    pub fn push_current(&mut self, stime: f64) {
        self.solve_store.nsolves += 1;
        self.solve_store.solves.push(Solve::new(
            &self.current_scramble,
            stime
        ));

        if let Err(e) = self.write_store() {
            display_error("Store error", format!("Failed to write store data: {}", e).as_str());
        }

        self.ao5 = self.solve_store.avg(5, true);
        self.ao12 = self.solve_store.avg(12, true);
        self.best = self.solve_store.best_time();
        self.worst = self.solve_store.worst_time();
    }

    pub fn unixt_to_string(&self, utime: u64) -> String {
        let dtime = Utc.timestamp_opt(utime as i64, 0).unwrap();
        let local = Local.from_utc_datetime(&dtime.naive_utc());
        local.to_rfc3339()
    }
}