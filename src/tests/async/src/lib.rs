extern crate macro_test;

use macro_test::*;

#[cfg(test)]
async fn my_test_function(arg1: i32, arg2: i32) {
	assert_eq!(arg1, arg2);
}

macro_tests!(
	attrs = [
		#[tokio::test]
	],
	ft = async my_test_function,
	(test_1, 1, 1),
	(test_2, 2, 2),
);
