use ::rand::thread_rng;
use ::rand::distributions::{Range, IndependentSample};

use ::problem::Problem;

#[derive(Debug, Clone)]
pub struct Solver {
    pub iterations: u64,
    pub initial_temperature: f64,
    pub temperature_reduction: f64,
    pub max_attempts: u64,
    pub max_accepts: u64,
    pub max_rejects: u64,
}

impl Solver {
    pub fn new() -> Solver {
        Default::default()
    }

    pub fn build_new<F>(builder: F) -> Solver where F: FnOnce(&mut Solver) {
        let mut solver = Solver::new();
        builder(&mut solver);
        solver
    }

    pub fn solve<P>(&self, problem: &P, initial: P::State) -> P::State where P: Problem {
        let mut rng = thread_rng();
        let range = Range::new(0.0, 1.0);

        let mut energy = problem.energy(&initial);
        let mut temperature = self.initial_temperature;

        let mut attempted = 0;
        let mut accepted = 0;
        let mut rejected = 0;

        let mut state = initial;

        for _ in 0 .. self.iterations {
            state = {
                let next_state = problem.new_state(&state);
                let new_energy = problem.energy(&next_state);

                attempted += 1;

                let de = new_energy - energy;

                if de < 0.0 || range.ind_sample(&mut rng) <= -de / temperature {
                    accepted += 1;
                    energy = new_energy;

                    next_state
                } else {
                    state
                }
            };

            if attempted >= self.max_attempts || accepted >= self.max_accepts {
                if accepted == 0 {
                    rejected += 1;
                } else {
                    rejected = 0;
                }

                attempted = 0;
                accepted = 0;
                temperature *= self.temperature_reduction;

                if rejected >= self.max_rejects {
                    break
                }
            }
        }

        state
    }
}

impl Default for Solver {
    fn default() -> Solver {
        Solver {
            iterations: 1000000,
            initial_temperature: 100.0,
            temperature_reduction: 0.95,
            max_attempts: 50,
            max_accepts: 10,
            max_rejects: 4,
        }
    }
}
