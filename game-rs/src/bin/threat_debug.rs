// cargo run --bin threat_debug | less -r

use game_rs::lookup::{StonesMask, STONES_COUNT, THREAT_LOOKUP_TABLE};

fn print_stones(stones: StonesMask) {
	let mut i = STONES_COUNT;
	while 0 < i {
		i -= 1;
		let stone = (stones >> (i * 2)) & 0b11;
		print!("{}{:02b} ", match stone {
			0b00 => "\x1b[93m",
			0b01 => "\x1b[91m",
			0b10 => "\x1b[92m",
			0b11 => "\x1b[90m",
			_ => unreachable!(),
		}, stone);
	}
	print!("\x1b[0m");
}

// fn print_threat_mask(threat_mask: BitMask) {
// 	let mut i = STONES_COUNT;
// 	while 0 < i {
// 		i -= 1;
// 		let threat = (threat_mask >> i) & 0b1;
// 		print!("{}", match threat {
// 			0b0 => "\x1b[0m",
// 			0b1 => "\x1b[91m",
// 			_ => unreachable!(),
// 		});
// 		print!("{:01b} ", threat);
// 	}
// 	print!("\x1b[0m");
// }

fn main() {
	for (i, threat_mask) in THREAT_LOOKUP_TABLE.iter().enumerate() {
		print_stones(i);
		// print!("   ");
		// print_threat_mask(threat_mask.threat_mask);
		print!("   ");
		for score in &threat_mask.threat_score {
			print!("{:?} ", score);
		}
		println!();
	}
}
