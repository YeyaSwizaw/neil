pub trait Problem {
    type State;

    fn energy(&self, state: &Self::State) -> f64;
    fn new_state(&self, state: &Self::State) -> Self::State;
}
