type NoteFn = fn(key: u8, velocity: u8);

pub fn callback(message: &[u8]) {
    if message.len() < 3 {
        return;
    }
    if get_channel(message) != 0 {
        return;
    }
    match message[0] >> 4 {
        8 => {
            note(message, note_off);
        }
        9 => {
            note(message, note_on);
        }
        11 => {
            if message[1] != 64 {
                return;
            }
            if message[2] < 64 {
                pedal_off();
            } else {
                pedal_on();
            }
        }
        _ => {}
    }
}

fn get_channel(message: &[u8]) -> u8 {
    message[0] & 0b1111
}

fn note(message: &[u8], notefn: NoteFn) {
    notefn(message[1], message[2]);
}

fn note_off(key: u8, velocity: u8) {
    println!("Note off -> key {}, velocity {}", key, velocity);
}

fn note_on(key: u8, velocity: u8) {
    println!("Note on -> key {}, velocity {}", key, velocity);
}

fn pedal_off() {
    println!("Pedal off");
}

fn pedal_on() {
    println!("Pedal on");
}
