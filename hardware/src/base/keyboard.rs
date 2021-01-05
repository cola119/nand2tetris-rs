use super::logic::{bit::O, Word};
use serde_json::Value;
use std::sync::mpsc::Receiver;

pub struct Keyboard {
    receiver: Option<Receiver<String>>,
    // 本来はRegisterを使って実装する.
    code: Option<Word>,
}

impl Keyboard {
    pub fn new(receiver: Option<Receiver<String>>) -> Self {
        Self {
            receiver,
            code: None,
        }
    }

    pub fn input(&mut self) {
        if let Some(ref rx) = self.receiver {
            if let Ok(str) = rx.try_recv() {
                let v: Value = serde_json::from_str(&str).unwrap();
                if v["down"].as_bool().unwrap() == false {
                    self.code = None;
                } else {
                    let num = v["key"].as_i64().unwrap();
                    let str: &str = &format!("{:016b}", num);
                    self.code = Some(Word::from(str));
                }
            }
        }
    }

    pub fn output(&self) -> Word {
        match self.code {
            None => Word::new([O; 16]),
            Some(x) => x,
        }
    }
}
