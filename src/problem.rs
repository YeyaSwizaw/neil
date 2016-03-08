/**
 * A problem represents something to be solved using simulated
 * annealing, and provides methods to calculate the energy of a
 * state and generate new states.
 */
pub trait Problem {
    type State;

    /**
     * This function should calculate the energy of a given state,
     * as a number between 0.0 and 1.0.
     *
     * Lower energy means the state is more optimal - simulated
     * annealing will try to find a state with the lowest energy.
     */
    fn energy(&self, state: &Self::State) -> f64;

    /**
     * This function should provide a new state, given the previous
     * state.
     */
    fn new_state(&self, state: &Self::State) -> Self::State;
}
