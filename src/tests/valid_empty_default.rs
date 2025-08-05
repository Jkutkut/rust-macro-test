extern crate macro_test;

use macro_test::macro_tests;

#[cfg(test)]
fn my_test_function(arg1: i32, arg2: i32) {
	assert_eq!(arg1, arg2);
}

macro_tests!(my_test_function,);
macro_tests!(
	my_test_function,
);
macro_tests!(
	my_test_function
);
