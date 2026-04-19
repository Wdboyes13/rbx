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

    pub fn min_max(&self, min_idx: &mut usize, max_idx: &mut usize, slice: &[Solve]) {
        if slice.is_empty() {
            return;
        }

        *min_idx = 0;
        *max_idx = 0;

        for i in 1..slice.len() {
            if slice[i].solve_time > slice[*max_idx].solve_time {
                *max_idx = i;
            }
            if slice[i].solve_time < slice[*min_idx].solve_time {
                *min_idx = i;
            }
        }
    }

    pub fn avg(&self, out_of: usize, weighted: bool) -> Option<f64> {
        if out_of == 0 || self.solves.len() < out_of {
            None
        } else {
            let l5s = &self.solves[self.solves.len()-out_of..];
            let mut l5s_times = Vec::<f64>::new();
            for i in l5s.iter() {
                l5s_times.push(i.solve_time);
            }

            if weighted {
                if l5s_times.len() <= 2 {
                    return None;
                }

                let mut max_idx: usize = 0;
                let mut min_idx: usize = 0;

                self.min_max(&mut min_idx, &mut max_idx, l5s);

                if max_idx > min_idx {
                    l5s_times.remove(max_idx);
                    l5s_times.remove(min_idx);
                } else {
                    l5s_times.remove(min_idx);
                    l5s_times.remove(max_idx);
                }
            }

            if l5s_times.is_empty() {
                return None;
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
        self.min_max(&mut min_idx, &mut max_idx, &self.solves);
        self.solves[min_idx].solve_time
    }

    pub fn worst_time(&self) -> f64 {
        if self.solves.is_empty() {
            return 0.0;
        }

        let mut max_idx: usize = 0;
        let mut min_idx: usize = 0;
        self.min_max(&mut min_idx, &mut max_idx, &self.solves);
        self.solves[max_idx].solve_time
    }
}