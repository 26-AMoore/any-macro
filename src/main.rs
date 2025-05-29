use std::{
	sync::mpsc::{self, Receiver, Sender},
	thread::{self, sleep},
	time::{Duration, Instant},
};

use inputbot::KeybdKey;

fn main() {
	let (sender, receiver): (Sender<KeyPress>, Receiver<KeyPress>) = mpsc::channel();
	let control_key = KeybdKey::PeriodKey;

	inputbot::KeybdKey::bind_all(move |key: inputbot::KeybdKey| {
		let sender = &sender;
		let start_time_millis = Instant::now();

		while key.is_pressed() {
			sleep(Duration::from_millis(1));
		}
		let press_duration = Instant::now().duration_since(start_time_millis);

		match sender.send(KeyPress {
			start_time: start_time_millis,
			press_duration,
			key,
		}) {
			Ok(_) => {}
			Err(e) => todo!("{:?}", e),
		}
	});

	let input_handler_thread = thread::spawn(|| {
		inputbot::handle_input_events(false);
	});

	//Lsuper = 125 || 0x7D

	let mut recorded_keys: Vec<KeyPress> = Vec::new();
	let mut logging = false;

	for event in receiver.iter() {
		if event.key.eq(&control_key) {
			logging = !logging;
			println!("Logging: {}", logging);
			if logging {
				recorded_keys = Vec::new();
			} else {
				for key in &recorded_keys {
					if key.key.eq(&control_key) {
						break;
					}
					//sleep(key.start_time.duration_since(start_time));
					key.key.press();
					sleep(key.press_duration);
					key.key.release();
				}
			}
		} else {
			if logging {
				recorded_keys.push(event);
			}
		}
	}

	input_handler_thread.join().unwrap();
}

#[derive(Clone, Copy, Debug)]
struct KeyPress {
	start_time: Instant,
	press_duration: Duration,
	key: KeybdKey,
}
