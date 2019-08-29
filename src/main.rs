


mod turing;
use turing::prelude::*;
use turing::prelude::enum_values::*;

fn main() {

	let program = ProgramBuilder::with_states(vec![
		State::with_transitions("State 0", vec![
			Transition::new(Blank, Blank, Right, "State 1"),
			Transition::new(0, 0, Left, "State 0"),
			Transition::new(1, 1, Left, "State 0"),
		]),
		State::with_transitions("State 1", vec![
			Transition::new(Blank, 1, Left, "State 2"),
			Transition::new(0, 1, Right, "State 2"),
			Transition::new(1, 0, Right, "State 1"),
		]),
		State::with_transitions("State 2", vec![
			Transition::new(Blank, Blank, Right, "Accept"),
			Transition::new(0, 0, Left, "State 2"),
			Transition::new(1, 1, Left, "State 2"),
		])
	]).build();

	let machine = Machine::with_program(program, vec![
		1, 0, 1, 1
	]);

	dbg!(machine);
}
