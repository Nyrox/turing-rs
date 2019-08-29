const BLOCK_SIZE: usize = 16;
pub type Alphabet = u8;

pub const Blank: Alphabet = 255;

pub const ACCEPT_STATE: usize = 0;
pub const REJECT_STATE: usize = 1;

mod internal {
	use super::{Alphabet, HeadAction, BLOCK_SIZE, Blank, ACCEPT_STATE, REJECT_STATE};

	#[derive(Debug)]
	pub struct Transition {
		pub cond: Alphabet,
		pub write: Alphabet,
		pub head: HeadAction,
		pub next: usize,
	}

	#[derive(Debug)]
	pub struct State {
		pub transitions: Vec<Transition>,
	}

	impl State {
		pub fn empty() -> Self {
			State {
				transitions: Vec::new(),
			}
		}
	}

	#[derive(Debug)]
	pub struct Block {
		cells: [Alphabet; BLOCK_SIZE],
	}

	#[derive(Debug)]

	pub struct Program {
		states: Vec<State>,
		current_state: usize,
	}

	impl Program {
		pub fn with_states(mut states: Vec<State>) -> Self {
			// marker states
			let mut marker_states = vec![State::empty(), State::empty()];
			marker_states.append(&mut states);

			Program {
				states: marker_states,
				current_state: 2,
			}
		}

		pub fn current_state(&self) -> &State {
			&self.states[self.current_state]
		}

		pub fn reject(&mut self) {
			self.current_state = 1;
		}
	}
#[derive(Debug)]
	pub struct Tape {
		head: usize,
		blocks: Vec<Block>,
	}

	impl Tape {
		pub fn from_input(input: Vec<Alphabet>) -> Self {
			Tape {
				head: BLOCK_SIZE,
				blocks: vec![Block {cells: [Blank; BLOCK_SIZE]}].into_iter().chain(input.chunks(BLOCK_SIZE).map(|cells| {
					let mut block = Block { cells: [Blank; BLOCK_SIZE] };

					for (i, e) in cells.iter().enumerate() {
						block.cells[i] = *e;
					}

					block
				})).collect()
			}
		}

		pub fn current_cell(&mut self) -> &mut Alphabet {
			&mut self.blocks[self.head / BLOCK_SIZE].cells[self.head % BLOCK_SIZE]
		}

		pub fn move_left(&mut self) {
			self.head = if self.head == 0 { 0 } else { self.head - 1 };
		}
		
		pub fn move_right(&mut self) {
			self.head = self.head + 1;
		}
	}

	#[derive(Debug)]
	pub struct Machine {
		pub program: Program,
		pub tape: Tape,
	}

	impl Machine {
		pub fn with_program(program: Program, input: Vec<Alphabet>) -> Self {
			Machine {
				program,
				tape: Tape::from_input(input),
			}
		}

		pub fn run_to_end(&mut self) -> bool {
			loop {
				self.step();

				match self.program.current_state {
					s if s == ACCEPT_STATE => return true,
					s if s == REJECT_STATE => return false,
					_ => ()
				}
			}
		}

		pub fn step(&mut self) {
			let state = self.program.current_state();
			let cell = self.tape.current_cell();

			dbg!(self.program.current_state);
			dbg!(*cell);

			for t in state.transitions.iter() {
				if t.cond == *cell {
					*cell = t.write;
					match t.head {
						HeadAction::Stay => (),
						HeadAction::Left => self.tape.move_left(),
						HeadAction::Right => self.tape.move_right(),
					}

					self.program.current_state = t.next;
					return;
				}
			}

			self.program.reject();
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum HeadAction {
	Stay,
	Left,
	Right,
}

pub struct Transition {
	cond: Alphabet,
	write: Alphabet,
	head: HeadAction,
	next: String,
}

impl Transition {
	pub fn new(cond: Alphabet, write: Alphabet, head: HeadAction, next: &str) -> Self {
		Transition {
			cond,
			write,
			head,
			next: next.to_owned(),
		}
	}
}

pub struct ProgramBuilder {
	states: Vec<State>,
}

impl ProgramBuilder {
	pub fn with_states(states: Vec<State>) -> Self {
		ProgramBuilder { states }
	}

	pub fn build(self) -> internal::Program {
		// We need to resolve our builder-states which use named states to our
		// internal State struct which uses indices
		let states = self.states.iter().map(|s| {
			internal::State {
				transitions: s.transitions.iter().map(|t| {
					internal::Transition {
						cond: t.cond,
						write: t.write,
						head: t.head,
						next: match t.next.as_str() {
							"Accept" => 0,
							"Reject" => 1,
							name => {
								2 + self.states.iter().position(|s| s.name == name).expect("failed to find state")
							}
						}
					}
				}).collect()
			}
		}).collect();

		internal::Program::with_states(states)
	}
}

pub struct State {
	name: String,
	transitions: Vec<Transition>,
}

impl State {
	pub fn with_transitions(name: &str, transitions: Vec<Transition>) -> State {
		State {
			name: name.to_owned(),
			transitions,
		}
	}
}

pub mod prelude {
	pub use super::{HeadAction, ProgramBuilder, State, Transition, Alphabet};
	pub use super::internal::{Machine};

	pub use super::Blank;

	pub mod enum_values {
		pub use super::HeadAction::*;
	}
}
