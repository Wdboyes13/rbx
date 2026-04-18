use crate::scramble::Seq;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Solve {
    pub scramble: Seq,
    pub solve_time: f64,
    pub recorded_at: u64
}

impl Solve {
    pub fn new(scramble: &Seq, solve_time: f64) -> Self {
        Self {
            scramble: scramble.clone(),
            solve_time,
            recorded_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SolveStore {
    pub nsolves: u64,
    pub solves: Vec<Solve>
}

impl SolveStore {
    pub fn to_binary(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_stdvec(self)
    }

    pub fn from_binary(data: Vec<u8>) -> Result<Self, postcard::Error> {
        postcard::from_bytes::<SolveStore>(&data)
    }

    pub fn min_max(&self, min_idx: &mut usize, max_idx: &mut usize) {
        *min_idx = 0;
        *max_idx = 0;

        for i in 0..self.solves.len() {
            if self.solves[i].solve_time > self.solves[*max_idx].solve_time {
                *max_idx = i;
            }
            if self.solves[i].solve_time < self.solves[*min_idx].solve_time {
                *min_idx = i;
            }
        }
    }

    pub fn avg(&self, out_of: usize, weighted: bool) -> Option<f64> {
        if self.solves.len() < out_of {
            None
        } else {
            let l5s = &self.solves[self.solves.len()-out_of-1..];
            let mut l5s_times = Vec::<f64>::new();
            for i in l5s.iter() {
                l5s_times.push(i.solve_time);
            }

            if weighted {
                let mut max_idx: usize = 0;
                let mut min_idx: usize = 0;

                self.min_max(&mut min_idx, &mut max_idx);

                l5s_times.remove(max_idx);
                l5s_times.remove(min_idx);
            }

            let mut avg: f64 = 0.0;
            for i in l5s_times.iter() {
                avg += i;
            }

            Some(avg / l5s_times.len() as f64)
        }
    }

    pub fn best_time(&self) -> f64 {
        if self.solves.is_empty() {
            return 0.0;
        }

        let mut max_idx: usize = 0;
        let mut min_idx: usize = 0;
        self.min_max(&mut min_idx, &mut max_idx);
        self.solves[min_idx].solve_time
    }

    pub fn worst_time(&self) -> f64 {
        if self.solves.is_empty() {
            return 0.0;
        }

        let mut max_idx: usize = 0;
        let mut min_idx: usize = 0;
        self.min_max(&mut min_idx, &mut max_idx);
        self.solves[max_idx].solve_time
    }
}