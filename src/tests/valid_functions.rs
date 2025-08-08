extern crate macro_test;

use macro_test::*;

#[cfg(test)]
fn my_test_function(arg1: i32, arg2: i32) {
	assert_eq!(arg1, arg2);
}

macro_tests!(
	attrs = [ #[test] ],
	ft = my_test_function,
	(test_1, 1, 1),
	(test_2, 1 + 1 - 2 + 2, 2)
);
macro_tests!(
	attrs = [
		#[test]
	],
	ft = my_test_function,
	(test_3, 1, 1),
	(test_4, 1 + 1 - 2 + 2, 2)
);

#[cfg(test)]
fn eq(arg1: i32, arg2: i32) {
	assert_eq!(arg1, arg2);
}

macro_tests!{
	ft = eq,
	(test_0_1, 1, 1)
}

macro_tests!{
	attrs = [ ],
	ft = eq,
	(test_1_1, 1, 1)
}

macro_tests!{
	attrs = [ #[test] ],
	ft = eq,
	(test_2_1, 1, 1)
}

macro_tests!{
	attrs = [ #[test], #[cfg(test)] ],
	ft = eq,
	(test_3_1, 1, 1)
}

macro_tests!{
	attrs = [ #[test], #[cfg(test)], #[cfg(test)] ],
	ft = eq,
	(test_4_1, 1, 1)
}
