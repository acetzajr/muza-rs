pub fn callback(message: &[u8]) {
    if message.len() < 3 {
        return;
    }
    match message[0] >> 4 {
        8 => {
            println!("Note off");
        }
        9 => {
            println!("Note on");
        }
        11 => {
            if message[1] != 64 {
                return;
            }
            if message[2] < 64 {
                println!("Pedal off");
            } else {
                println!("Pedal on");
            }
        }
        _ => {}
    }
}
