extern crate macro_test;

use macro_test::*;

#[cfg(test)]
fn my_test_function(arg1: i32, arg2: i32) {
	assert_eq!(arg1, arg2);
}

macro_tests!(
	attrs = [#[test]],
	ft = my_test_function
);
macro_tests!(
	attrs = [
		#[test]
	],
	ft = my_test_function
);
macro_tests!(
	attrs = [
		#[test]
	],
	ft = my_test_function,
);
macro_tests!(
	attrs = [#[test] #[cfg(test)]],
	ft = my_test_function
);
macro_tests!(
	attrs = [
		#[test]
		#[cfg(test)]
	],
	ft = my_test_function
);
macro_tests!(
	attrs = [
		#[test]
		#[cfg(test)]
		#[not(not(cfg(test)))]
	],
	ft = my_test_function
);
macro_tests!(
	attrs = [
		#[test]
		#[cfg(test)]
		#[not(not(cfg(test)))]
	],
	ft = my_test_function,
);

#[cfg(test)]
fn eq(a: i32, b: i32) { assert_eq!(a, b); }

macro_tests!{
	ft = eq
}

macro_tests!{
	attrs = [ ],
	ft = eq
}

macro_tests!{
	attrs = [ #[test] ],
	ft = eq
}

macro_tests!{
	attrs = [ #[test], #[cfg(test)] ],
	ft = eq
}

macro_tests!{
	attrs = [ #[test], #[cfg(test)], #[cfg(test)] ],
	ft = eq
}
