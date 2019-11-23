# turing-rs
Simple turing machine implementation I wrote as part of a school assignment on turing machine architecture.

This turing machine features an infinitely growing tape, but only in the positive direction.
If you try to go left from cell 0, you will enter the error-halting state.
This doesn't theoretically limit what can be computed, but it does mean many programs from the internet will not work sadly.


## Example of creating a program
```rust
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
```
