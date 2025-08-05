extern crate macro_test;

use macro_test::macro_tests;

#[cfg(test)]
fn my_test_function(arg1: i32, arg2: i32) {
	assert_eq!(arg1, arg2);
}

macro_tests!(
	attrs = [ #[test] ],
	my_test_function,
	(test_1, 1, 1),
	(test_2, 1 + 1 - 2 + 2, 2)
);
macro_tests!(
	attrs = [
		#[test]
	],
	my_test_function,
	(test_3, 1, 1),
	(test_4, 1 + 1 - 2 + 2, 2)
);
