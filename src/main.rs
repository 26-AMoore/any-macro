use std::{
	collections::HashMap,
	sync::mpsc::{self, Receiver, Sender},
	thread::{self, sleep},
	time::{Duration, Instant},
};

use inputbot::KeybdKey;

fn main() {
	let (sender, receiver): (Sender<KeyPress>, Receiver<KeyPress>) = mpsc::channel();
	let control_key = KeybdKey::RBracketKey;
	let replay_key = KeybdKey::LBracketKey;
	//                   Name       Start    List of presses
	let mut map: HashMap<KeybdKey, (Instant, Vec<KeyPress>)> = HashMap::new();

	inputbot::KeybdKey::bind_all(move |key: inputbot::KeybdKey| {
		let sender = &sender;
		let start_time_millis = Instant::now();

		while key.is_pressed() {
			sleep(Duration::from_millis(1));
		}

		let press_duration = Instant::now().duration_since(start_time_millis);

		println!("Key: {:?}, Time: {:?}", key, press_duration);
		// sends a KeyPress to the mpsc
		match sender.send(KeyPress {
			start_time: start_time_millis,
			press_duration,
			key,
		}) {
			Ok(_) => {}
			Err(e) => todo!("{:?}", e),
		}
	});

	let mut log_next = true;

	// main loop
	for event in receiver {
		println!("{:?}", event);
		if event.key.eq(&control_key) {
			log_next = true;
		}

		println!("{:?}", event);
		let mut record = Vec::new();
		if log_next {
			map.insert(
				control_key,
				(Instant::now(), {
					println!("{:?}", event);
					record.push(event);
					record
				}),
			);
			log_next = false;
		}
		println!("{:?}", map);

		if !log_next && event.key.eq(&replay_key) {
			replay(
				map.get(&control_key).unwrap().1.clone(),
				map.get(&control_key).unwrap().0,
			);
		}
	}

	// make a new thread because `handle_input_events()` is blocking
	let input_handler_thread = thread::spawn(|| {
		inputbot::handle_input_events(false);
	});

	input_handler_thread.join().unwrap();
}

fn replay(codes: Vec<KeyPress>, start: Instant) {
	for key in codes {
		let key = key.clone();
		thread::spawn(move || {
			sleep(key.start_time.duration_since(start));
			key.key.press();
			sleep(key.press_duration);
			key.key.release();
		});
	}
}

fn record(receiver: &Receiver<KeyPress>, control_key: KeybdKey) -> Vec<KeyPress> {
	let mut record: Vec<KeyPress> = Vec::new();
	for event in receiver.iter() {
		println!("pressed: {}", event.key);
		if event.key.eq(&control_key) {
			break;
		}
		record.push(event);
	}
	record
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct KeyPress {
	start_time: Instant,
	press_duration: Duration,
	key: KeybdKey,
}

/*
1pressed: NumPad1
2pressed: Down
3pressed: NumPad3
4pressed: Left
5pressed: NumPad5
6pressed: Right
7pressed: Home
8pressed: Up
9pressed: NumPad9
*/
