extern crate macro_test;

use macro_test::*;

#[cfg(test)]
fn cero() { assert!(true); }
#[cfg(test)]
fn one(arg1: i32) { assert_eq!(arg1, arg1); }
#[cfg(test)]
fn two(arg1: i32, arg2: i32) { assert_eq!(arg1, arg2); }
#[cfg(test)]
fn tree(arg1: i32, arg2: i32, arg3: i32) {
	assert_eq!(arg1, arg3);
	assert_eq!(arg2, arg3);
}

macro_tests!(
	ft = cero,
	(cero_1_1)
);
macro_tests!(
	ft = cero,
	(cero_1_2,),
);
macro_tests!(
	ft = cero,
	(cero_1_3),
	(cero_2_3,),
);
macro_tests!(
	ft = cero,
	(cero_1_4),
	(cero_2_4,)
);

macro_tests!(
	ft = one,
	(one_1_1, 1),
);
macro_tests!(
	ft = one,
	(one_1_2, 1,),
);
macro_tests!(
	ft = one,
	(one_1_3, 1),
	(one_2_3, 2,),
);
macro_tests!(
	ft = one,
	(one_1_4, 1),
	(one_2_4, 2,)
);

macro_tests!(
	ft = two,
	(two_1_1, 1, 1),
);
macro_tests!(
	ft = two,
	(two_1_2, 1, 1,),
);
macro_tests!(
	ft = two,
	(two_1_3, 1, 1),
	(two_2_3, 2, 2,),
);
macro_tests!(
	ft = two,
	(two_1_4, 1, 1),
	(two_2_4, 2, 2,)
);

macro_tests!(
	ft = tree,
	(tree_1_1, 1, 1, 1),
);
macro_tests!(
	ft = tree,
	(tree_1_2, 1, 1, 1,),
);
macro_tests!(
	ft = tree,
	(tree_1_3, 1, 1, 1),
	(tree_2_3, 2, 2, 2,),
);
macro_tests!(
	ft = tree,
	(tree_1_4, 1, 1, 1),
	(tree_2_4, 2, 2, 2,)
);

macro_tests!(
	attrs = [
		#[test]
	],
	ft = cero,
	(cero_1_5)
);
macro_tests!(
	attrs = [
		#[test]
	],
	ft = cero,
	(cero_1_6,),
);
macro_tests!(
	attrs = [
		#[test]
	],
	ft = cero,
	(cero_1_7),
	(cero_2_7,),
);
macro_tests!(
	attrs = [
		#[test]
	],
	ft = cero,
	(cero_1_8),
	(cero_2_8,)
);

macro_tests!(
	attrs = [
		#[test],
		#[cfg(test)]
	],
	ft = cero,
	(cero_1_9)
);
macro_tests!(
	attrs = [
		#[test],
		#[cfg(test)]
	],
	ft = cero,
	(cero_1_10,),
);
macro_tests!(
	attrs = [
		#[test],
		#[cfg(test)]
	],
	ft = cero,
	(cero_1_11),
	(cero_2_11,),
);
macro_tests!(
	attrs = [
		#[test],
		#[cfg(test)]
	],
	ft = cero,
	(cero_1_12),
	(cero_2_12,)
);
