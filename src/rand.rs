use std::{
    time::{SystemTime, UNIX_EPOCH},
    io::{self, Read},
    fs::File
};
// use rand;

fn random_u32() -> u32 {
    let mut rng = File::open("/dev/urandom").unwrap();

    let mut buffer = [0u8; 4];
    rng.read_exact(&mut buffer).unwrap();
    ((buffer[0] as u32) << 24) | ((buffer[1] as u32) << 16) | ((buffer[2] as u32) << 8) | (buffer[3] as u32)
}

fn random_f32() -> f32 {
    let random_u32_number = random_u32();
    random_u32_number as f32 / u32::MAX as f32
}

pub struct Rand {
    curr: f32
}

impl Rand {
    pub fn new() -> Self {
        Self {
            curr: 1.0
        }
    }

    pub fn new_with_seed(seed: f32) -> Self {
        Self {
            curr: seed
        }
    }

    pub fn new_with_nanos() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        Self {
            curr: ((seed as f32 * 19475382.0) % 100000.0) / 1843489234.7342
        }
    }

    pub fn next(&mut self) -> f32 {
        // return random_f32();
        return rand::random::<f32>();
        self.curr = self.curr * 1103515245.0 + 12345.0;
        self.curr = (self.curr / 65536.0) % 32768.0;
        self.curr / 32768.0
    }

    pub fn next_with_range(&mut self, range_min: f32, range_max: f32) -> f32 {
        // let rand_num = random_f32(); 
        // return rand_num * (range_max - range_min) + range_min;

        let rand_num = rand::random::<f32>();
     
        return rand_num * (range_max - range_min) + range_min;
        self.curr = self.curr * 1103515245.0 + 12345.0;
        self.curr = (self.curr / 65536.0) % 32768.0;
        let temp = self.curr / 32768.0;
        temp * (range_max - range_min) + range_min
    }

}
