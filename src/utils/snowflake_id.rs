use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use idgenerator::{IdGeneratorOptions, IdInstance};
use lazy_static::lazy_static;

pub struct SnowflakeIdGenerator {
    worker_id: u16,
    sequence: u16,
    last_timestamp: u64,
}

impl SnowflakeIdGenerator {
    pub fn new(worker_id: u16) -> Self {
        SnowflakeIdGenerator {
            worker_id,
            sequence: 0,
            last_timestamp: 0,
        }
    }

    pub fn generate_id(&mut self) -> Result<u64, &'static str> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "SystemTime before UNIX EPOCH!")?
            .as_millis() as u64;

        if timestamp < self.last_timestamp {
            return Err("Clock moved backwards!");
        }

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) % 4096;
            if self.sequence == 0 {
                // 等待下一毫秒
                while timestamp <= self.last_timestamp {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|_| "SystemTime before UNIX EPOCH!")?
                        .as_millis() as u64;
                    if now > timestamp {
                        break;
                    }
                }
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        let id = ((timestamp - 1609459200000) << 22)
            | ((self.worker_id as u64) << 12)
            | (self.sequence as u64);
        Ok(id)
    }
}

lazy_static! {
    static ref ID_GENERATOR: Mutex<SnowflakeIdGenerator> = Mutex::new(SnowflakeIdGenerator::new(1));
}

pub fn generate_unique_id() -> Result<u64, &'static str> {
    let mut id_generator = ID_GENERATOR.lock().unwrap();
    id_generator.generate_id()
}


pub fn generate_snowflake_id() -> u64 {
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6).seq_bit_len(14);
    let _ = IdInstance::init(options);
    let new_id = IdInstance::next_id();
    return new_id as u64;
}