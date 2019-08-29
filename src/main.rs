


mod turing;
use turing::prelude::*;
use turing::prelude::enum_values::*;

fn main() {

	// Binary NOT
	/*
	let program = ProgramBuilder::with_states(vec![
		State::with_transitions("State 0", vec![
			Transition::new(Blank, Blank, Right, "Accept"),
			Transition::new(0, 1, Right, "State 0"),
			Transition::new(1, 0, Right, "State 0"),
		]),
	]).build();
	*/

	let program = ProgramBuilder::with_states(vec![
		// Go to the right most digit
		State::with_transitions("State 0", vec![
			Transition::new(Blank, Blank, Left, "State 1"),
			Transition::new(0, 0, Right, "State 0"),
			Transition::new(1, 1, Right, "State 0"),
		]),
		State::with_transitions("State 1", vec![
			Transition::new(Blank, 1, Right, "Accept"),
			Transition::new(0, 1, Right, "State 0"),
			Transition::new(1, 0, Left, "State 1"),
		])
	]).build();

	let mut machine = Machine::with_program(program, vec![
		0, 1, 1, 0
	]);


	match machine.run_to_end() {
		true => {
			println!("Program ran successfully: \n\n");
			dbg!(machine.tape);
		},
		false => {
			println!("Program exited in reject state: \n\n");
			dbg!(machine.tape);
		}
	}
}
