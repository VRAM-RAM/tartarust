use crate::generation::Pepper;


/// Contains tartarus parameters :
/// 
/// - memory (in MiB)
/// - iterations 
/// - pepper
pub struct TartarusParams {
    pub memory: u32,
    pub iterations: u32,
    pub pepper: Pepper,
}

impl TartarusParams {
    pub fn new(memory: u32, iterations: u32, pepper: Pepper) -> Self {
        Self { memory, iterations, pepper }
    }

    /// The reocmmended parameters. 
    /// Warning : the parameters aren't audited.
    pub fn recommended() -> Self {
        Self {
            memory: 64,
            iterations: 3,
            pepper: Pepper::generate(),
        }
    }

    /// Parameters for test.
    pub fn test() -> Self {
       Self {
            memory: 12,
            iterations: 1,
            pepper: Pepper::generate(),
       }
    }

    /// Inserts a custom pepper in the parameters. For example :
    /// 
    /// ```rust
    /// use tartarust_lib::generation::Pepper;
    /// use tartarust_lib::params::TartarusParams;
    /// 
    /// let mut params = TartarusParams::recommended();
    /// params.with_pepper(Pepper::from("my pepper"));
    /// ```
    pub fn with_pepper(&mut self, pepper: Pepper) {
        self.pepper = pepper
    }
}

