use std::fmt::{
    self,
    Display,
    Formatter,
    Write,
};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

#[derive(Debug, Copy, Clone)]
pub struct NodeID {
    value: usize,
}

impl NodeID {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

const CHARSET: &[u8] =
    b"!#%()*+,-./:;=?@[]^_`{|}~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
impl Display for NodeID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let n = self.value;
        write!(f, "\"")?;
        // if n != 0 {
        //     n -= 1;
        //     if n == 0 {
        //         f.write_char(CHARSET[0] as char)?;
        //     } else {
        //         while n > 0 {
        //             f.write_char(CHARSET[n % CHARSET.len()] as char)?;
        //             n /= CHARSET.len();
        //         }
        //     }
        // }

        // 种子为 n
        let mut rng = SmallRng::seed_from_u64(n as u64);
        for _ in 0..20 {
            f.write_char(CHARSET[rng.random_range(0..CHARSET.len())] as char)?;
        }

        write!(f, "\"")
    }
}
