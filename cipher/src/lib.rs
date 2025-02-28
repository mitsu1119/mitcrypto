use std::sync::{Arc, Mutex};

use log::{debug, info, Log};

struct TestLogger {
    // log messages
    logs: Arc<Mutex<Vec<String>>>,
}

impl TestLogger {
    fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn logs(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }

    fn clear_logs(&self) {
        self.logs.lock().unwrap().clear();
    }
}

impl Log for TestLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log_message = format!("{}: {}", record.level(), record.args());
            let mut logs = self.logs.lock().unwrap();
            println!("{}", log_message);
            logs.push(log_message);
        }
    }

    fn flush(&self) {
        // do nothing
    }
}

type Block = [u8; AES::BLOCK_SIZE];

pub struct AES {
    key: Block,
}

impl AES {
    const BLOCK_SIZE: usize = 16;
    pub fn new(key: Block) -> Self {
        Self { key }
    }

    pub fn encrypt(&self, plaintext: Block) -> Block {
        debug!("plaintext: {:?}", plaintext);
        *b"testtesttesttest"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_logger() -> Arc<TestLogger> {
        let logger = Arc::new(TestLogger::new());

        log::set_boxed_logger(Box::new(logger.clone())).expect("set logger");
        log::set_max_level(log::LevelFilter::Debug);

        logger
    }

    #[test]
    fn test_aes() {
        let logger = setup_logger();

        let key = *b"testtesttesttest";
        let aes = AES::new(key);
        let plaintext = *b"testtesttesttest";
        let ciphertext = aes.encrypt(plaintext);

        println!("{:?}", logger.logs());

        panic!("ugya");
    }
}
