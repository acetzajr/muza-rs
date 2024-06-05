use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

type Sender = mpsc::Sender<SenderMessage>;
type Receiver = mpsc::Receiver<ReceiverMessage>;
type Awaker = mpsc::Sender<()>;
type Waiter = mpsc::Receiver<()>;

const KEYS: usize = 128;

struct Processor {
    handle: JoinHandle<()>,
}

struct Syn0x0 {
    scale: Scale,
    states: [KeyState; KEYS],
}

#[derive(Default, Clone, Copy)]
enum Phase {
    #[default]
    Idle,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
}

#[derive(Default, Clone, Copy)]
struct KeyState {
    phase: Phase,
    frequency: f64,
}

enum ReceiverMessage {
    Return,
    NoteOn { key: u8, velocity: u8 },
    NoteOff { key: u8, velocity: u8 },
    PedalOn,
    PedalOff,
    Process { frames: usize },
}
enum SenderMessage {}

struct Scale {
    rations: [f64; 12],
    base: f64,
}

impl Scale {
    fn equal_tempered_fn(note: f64, base: f64) -> f64 {
        base * 2f64.powf(note / 12.)
    }

    fn acetza(base: f64) -> Self {
        Self {
            base,
            rations: [
                1.,
                256. / 243.,
                9. / 8.,
                32. / 27.,
                81. / 64.,
                4. / 3.,
                Scale::equal_tempered_fn(6., 1.),
                3. / 2.,
                128. / 81.,
                27. / 16.,
                16. / 9.,
                243. / 128.,
            ],
        }
    }

    fn frequency(&self, note: usize) -> f64 {
        let index = note % 12;
        self.base * self.rations[index] * 2usize.pow(index as u32) as f64
    }
}

impl Syn0x0 {
    fn new(sender: &Sender) -> Self {
        let scale = Scale::acetza(1.);
        let mut states: [KeyState; KEYS] = [KeyState::default(); KEYS];
        for (key, state) in states.iter_mut().enumerate() {
            state.frequency = scale.frequency(key);
        }
        Self { scale, states }
    }

    fn run(receiver: Receiver, sender: Sender) {
        let mut synth = Syn0x0::new(&sender);
        let (awaker, waiter) = mpsc::channel();
        let mut processors = Vec::with_capacity(16);
        for _ in 0..processors.capacity() {
            let awaker = awaker.clone();
            let sender = sender.clone();
            processors.push(Processor {
                handle: thread::spawn(move || processor(sender, awaker)),
            })
        }
        'handling: for message in receiver {
            match message {
                ReceiverMessage::Return => break 'handling,
                ReceiverMessage::NoteOn { key, velocity } => todo!(),
                ReceiverMessage::NoteOff { key, velocity } => todo!(),
                ReceiverMessage::PedalOn => todo!(),
                ReceiverMessage::PedalOff => todo!(),
                ReceiverMessage::Process { frames } => synth.process(frames),
            }
        }
    }

    fn process(&mut self, frames: usize, waiter: Waiter) {
        for state in &self.states {
            if let Phase::Idle = state.phase {
                continue;
            }
        }
    }
}

fn processor(sender: Sender, awaker: Awaker) {}
