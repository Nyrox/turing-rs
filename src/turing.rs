const BLOCK_SIZE: usize = 16;
pub type Alphabet = u8;
pub const Blank: Alphabet = 255;


mod internal {
	use super::{Alphabet, HeadAction, BLOCK_SIZE, Blank};

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
		pub fn with_states(states: Vec<State>) -> Self {
			Program {
				states,
				current_state: 2,
			}
		}
	}
#[derive(Debug)]
	pub struct Tape {
		head: usize,
		cells: Vec<Block>,
	}

	impl Tape {
		pub fn from_input(input: Vec<Alphabet>) -> Self {
			Tape {
				head: 0,
				cells: input.windows(BLOCK_SIZE).map(|cells| {
					let mut block = Block { cells: [Blank; BLOCK_SIZE] };

					for (i, e) in cells.iter().enumerate() {
						block.cells[i] = *e;
					}

					block
				}).collect()
			}
		}
	}

	#[derive(Debug)]
	pub struct Machine {
		program: Program,
		tape: Tape,
	}

	impl Machine {
		pub fn with_program(program: Program, input: Vec<Alphabet>) -> Self {
			Machine {
				program,
				tape: Tape::from_input(input),
			}
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
								self.states.iter().position(|s| s.name == name).expect("failed to find state")
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
