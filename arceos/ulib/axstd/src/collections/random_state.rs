// Implement RandomState to provide hashing for HashMap
use arceos_api::modules::axhal::misc::random;
use core::hash::BuildHasher;
use core::hash::Hasher;

/// A hasher that uses arceos random implementation
#[derive(Default)]
pub struct RandomHasher {
    state: u64,
}

impl Hasher for RandomHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut state = self.state;
        for &b in bytes {
            state = state.wrapping_mul(31).wrapping_add(b as u64);
        }
        self.state = state;
    }
}

/// A random state that uses arceos random implementation
#[derive(Clone, Default)]
pub struct RandomState {
    seed: u64,
}

impl RandomState {
    /// Creates a new `RandomState` that is initialized with a random seed.
    pub fn new() -> Self {
        Self {
            seed: random() as u64,
        }
    }
}

impl BuildHasher for RandomState {
    type Hasher = RandomHasher;

    fn build_hasher(&self) -> RandomHasher {
        let mut hasher = RandomHasher::default();
        hasher.state = self.seed;
        hasher
    }
}
