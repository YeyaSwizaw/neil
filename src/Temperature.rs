#[derive(Debug, Clone)]
pub struct Temperature {
    /**
     * The current step number
     */
    pub step: u64,

    /**
     * The initial temperature of the process.
     */
    pub initial_temperature: f64,

    
    
}


impl Temperature {
	pub fn new() -> Temperature {
        Default::default()
    }
	
    fn exponential_cooling(&self) {}
    
    fn linear_cooling(&self) {}
    
    fn adaptive_cooling(&self) {}
}


impl Default for Temperature {
    fn default() -> Temperature {
        Temperature {
            
        }
    }
}
