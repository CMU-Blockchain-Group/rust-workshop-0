
/// takes two references, `a` and `b`, and swaps the values they point to
fn swap_two_ints(a: &mut i32, b: &mut i32) {
	// TODO
	unimplemented!()
}

struct Rectangle {
	// TODO
}

/// given two rectangles, returns whether or not they overlap
/// you'll need to define `Rectangle` above
fn rect_overlaps(a: &Rectangle, b: &Rectangle) -> bool {
	// TODO
	unimplemented!()
}

enum RockPaperScissors {
	Rock,
	Paper,
	Scissors
}

/// given a Rock Paper Scissors move, return the move that beats it
/// use a match statement instead of an if statement
fn rps_weakness(play: RockPaperScissors) -> RockPaperScissors {
	// TODO
	unimplemented!()
}

/// given two slices of integers `a` and `b`, compute their dot product
/// e.g. dot_product(&[1, 2, 3, 4], &[4, 3, 2, 1]) = 1 * 4 + 2 * 3 + 3 * 2 + 4 * 1`
fn dot_product(a: &[i32], b: &[i32]) -> i32 {
	// TODO
	unimplemented!()
}

/// collect the first n fibonnaci numbers into a vector and return the result
fn first_n_fib(n: usize) -> Vec<u32> {
	// TODO
	unimplemented!()
}

/// given a sorted slice of integers `a` and an element `x`, find the index of `x`. If it exists, return `Some(index)` otherwise return `None`
/// this function should be recursive and you should not make any helper functions with hi/lo parameters (hint: use slices)
/// e.g. a = `[1, 2, 3, 4, 5]`. After calling `multiply_slice(&mut a, 1, 3)`, the array should be `[1, 3, 2, 4, 5]`
fn recursive_binary_search(a: &[i32]) -> Option<usize> {
	// TODO
	unimplemented!()
}

/// given a slice of slices of strings, append all of the strings together
/// since we didn't talk about the string type, you may want to read about it 
fn flatten_slice_of_slices(strs: &[&[&str]]) -> String {
	// TODO
	unimplemented!();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_swap_two_ints() {
		let mut a = 1;
		let mut b = 2;
		swap_two_ints(&mut a, &mut b);
		assert_eq!(a, 2);
		assert_eq!(b, 1);
	}

	// TODO Your tests here!
	// Define a function with the #[test] attribute macro to define it as a test.
	// Then run `cargo test` to run it
}