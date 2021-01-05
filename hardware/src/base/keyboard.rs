use std::sync::mpsc::Receiver;

pub struct Keyboard {
    receiver: Option<Receiver<String>>,
}

impl Keyboard {
    pub fn new(receiver: Option<Receiver<String>>) -> Self {
        Self { receiver }
    }

    pub fn output(&self) {
        if let Some(ref rx) = self.receiver {
            if let Ok(str) = rx.try_recv() {
                println!("hogehoge {}", str);
            }
        }
    }
}
