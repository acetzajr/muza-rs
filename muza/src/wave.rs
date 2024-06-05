use std::sync::mpsc::{Receiver, Sender};

use crate::aliases::{Index, Sample};

pub enum ReceiverMessage {
    Return,
    NeedSample,
}

pub struct MPSC {
    messages: Receiver<ReceiverMessage>,
    consumer: Sender<Sample>,
}

pub struct Buffer {
    blocks: Vec<Block>,
    current: Index,
    last: Index,
    first: Index,
    sample: Index,
}

pub struct Block {
    samples: Vec<Sample>,
    ready: bool,
}

impl Buffer {
    fn thread(mut self, mpsc: MPSC) {
        for message in mpsc.messages {
            match message {
                ReceiverMessage::Return => return,
                ReceiverMessage::NeedSample => {
                    mpsc.consumer.send(self.sample()).unwrap();
                }
            }
        }
    }

    fn sample(&mut self) -> Sample {
        if self.sample >= self.blocks[self.current].samples.len() && !self.try_swap() {
            return 0.;
        }
        let sample = self.blocks[self.current].samples[self.sample];
        self.sample += 1;
        sample
    }

    fn try_swap(&mut self) -> bool {
        if !self.blocks[self.last].ready {
            return false;
        }
        self.sample = 0;
        self.blocks[self.current].ready = false;
        self.current = (self.current + 1) % self.blocks.len();
        self.first = (self.first + 1) % self.blocks.len();
        self.last = (self.last + 1) % self.blocks.len();
        true
    }
}
