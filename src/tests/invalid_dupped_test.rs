extern crate macro_test;

use macro_test::*;

#[cfg(test)]
fn my_test_function(arg1: i32, arg2: i32) {
	assert_eq!(arg1, arg2);
}

macro_tests!(
	ft = my_test_function,
	(test_1, 1, 1),
	(test_2, 1 + 1 - 2 + 2, 2)
);

#[test]
fn test_1() {
	// This test already exists
}
