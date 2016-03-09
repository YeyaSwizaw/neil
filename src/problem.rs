//////////////////////////////////////////////////////////////////////////////
//  File: neil/problem.rs
//////////////////////////////////////////////////////////////////////////////
//  Copyright 2016 Samuel Sleight
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//////////////////////////////////////////////////////////////////////////////

/**
 * A problem represents something to be solved using simulated
 * annealing, and provides methods to calculate the energy of a
 * state and generate new states.
 */
pub trait Problem {
    type State;

    /**
     * This function should generate an initial state for the problem.
     */
    fn initial_state(&self) -> Self::State;

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
