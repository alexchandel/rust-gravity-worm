extern crate ncurses;

use std::iter::count;
use std::collections::ringbuf::RingBuf;
use std::collections::Deque;
use std::io::timer;

struct Game {
	worm_height: Box<RingBuf<i32>>,
	cave_above_height: Box<RingBuf<i32>>,
	cave_ahead_height: Box<RingBuf<i32>>,
	gap: i32,
	cave_incr: bool,
	worm_decr: bool,
	max_y: i32,
	max_x: i32,
	worm_len: uint,
	score: i64
}

impl Game {
	fn with_size(y: i32, x: i32) -> Game {
		let cave_init = y/8;
		let worm_init = y/4;
		let worm_len  = x/2+1;
		let ahead_len = x-worm_len;

		let mut worm_ring  = box RingBuf::with_capacity(worm_len as uint);
		let mut above_ring = box RingBuf::with_capacity(worm_len as uint);
		let mut ahead_ring = box RingBuf::with_capacity(ahead_len as uint);

		worm_ring.extend( count(worm_init, 0).take(worm_len as uint));
		above_ring.extend(count(cave_init, 0).take(worm_len as uint));
		ahead_ring.extend(count(cave_init, 0).take(ahead_len as uint));

		return Game{
			worm_height: worm_ring,
			cave_above_height: above_ring,
			cave_ahead_height: ahead_ring,
			gap: cave_init*7,
			cave_incr: false,
			worm_decr: false,
			max_y: y,
			max_x: x,
			worm_len: worm_len as uint,
			score: 0
		}
	}

	/* Whether the worm has collided with the ceiling or floor */
	fn worm_alive(&self) -> bool {
		let ceil = *self.cave_above_height.back().unwrap();
		let worm = *self.worm_height.back().unwrap();
		return (ceil < worm) && (worm < ceil + self.gap)
	}

	fn advance_one_step(&mut self) {
		/* Check if wall reversal needed */
		let back: i32 = *self.cave_ahead_height.back().unwrap();
		if (back + self.gap) >= self.max_y {
			self.cave_incr = false;
			if self.gap > 1 {
				self.gap -= 1;
			}
		} else if back <= 0 {
			self.cave_incr = true;
		}

		/* Advance walls */
		let _ = self.cave_above_height.pop_front();
		self.cave_above_height.push_back(self.cave_ahead_height.pop_front().unwrap());
		self.cave_ahead_height.push_back(back + if self.cave_incr {1} else {-1});

		/* Advance worm */
		let head = *self.worm_height.back().unwrap();
		let _ = self.worm_height.pop_front();
		self.worm_height.push_back(head + if self.worm_decr {-1} else {1});

	}
}

fn main() {

	println!("\nHello world!");
	println!("Welcome to Rust Gravity Worm for ncurses, by alex!");
	println!("\nControls:");
	println!("\nspace:\t\tUp");
	println!("any other key:\tDown")
	println!("\nPress any key to begin...")

	/* Start ncurses */
	ncurses::initscr();
	ncurses::cbreak(); // don't wait for newlines in input

	/* Print intro + controls */
	ncurses::clear();
	ncurses::mvprintw(1, 4, "Hello world!");
	ncurses::mvprintw(2, 4, "Welcome to Rust Gravity Worm for ncurses, by alex!");
	ncurses::mvprintw(4, 4, "Controls:");
	ncurses::mvprintw(6, 4, "space:\t\tUp");
	ncurses::mvprintw(7, 4, "any other key:\tDown");
	ncurses::mvprintw(9, 4, "Press any key to begin...");
	ncurses::refresh();
	timer::sleep(500);
	ncurses::getch();

	/* Keyboard */
	ncurses::nodelay(ncurses::stdscr, true); // don't block for input
	ncurses::noecho(); // don't echo input
	// ncurses::keypad(ncurses::stdscr, true); // accept special keys
	ncurses::curs_set(ncurses::CURSOR_INVISIBLE); // don't display cursor

	/* Get screen bounds */
	let mut _max_y = 0;
	let mut _max_x = 0;
	ncurses::getmaxyx(ncurses::stdscr, &mut _max_y, &mut _max_x);
	/* Initialize game */
	let mut g = Game::with_size(_max_y, _max_x);
	let mut refresher = timer::Timer::new().unwrap();
	let painter = refresher.periodic(100); // repaint every 100ms

	loop {
		painter.recv();

		/* Update worm direction from input */
		loop {
			let chr = ncurses::getch();
			if chr != ncurses::ERR {
				g.worm_decr = (chr as u8 as char) == ' ';
			} else {
				break;
			}
		}

		/* Run actions */
		g.advance_one_step();

		/* Draw current frame */
		ncurses::clear();
		for (num, cave) in g.cave_above_height.iter().enumerate() {
			ncurses::mvprintw(*cave as i32, num as i32, "x"); // Cave top
			ncurses::mvprintw((*cave + g.gap) as i32, num as i32, "x"); // Cave bottom
		}
		for (num, cave) in g.cave_ahead_height.iter().enumerate() {
			ncurses::mvprintw(*cave as i32, (num + g.worm_len) as i32, "x"); // Cave top
			ncurses::mvprintw((*cave + g.gap) as i32, (num + g.worm_len) as i32, "x"); // Cave bottom
		}
		for (num, worm) in g.worm_height.iter().enumerate() {
			ncurses::mvprintw(*worm as i32, num as i32, "="); // Worm
		}
		ncurses::mvprintw(0, 0, format!("Score: {0}", g.score).as_slice());
		/* Check for game over */
		if !g.worm_alive() {
			ncurses::mvprintw(1, 0, "GAME OVER!");
			ncurses::mvprintw(2, 0, "Press enter to finish...");
			break;
		}
		ncurses::refresh();

		g.score += 1;
	}

	/* Display final sceen until key is pressed */
	ncurses::refresh();
	ncurses::nocbreak();
	ncurses::nodelay(ncurses::stdscr, false);
	ncurses::getch();
	ncurses::endwin(); // Restore normal terminal behavior
}
