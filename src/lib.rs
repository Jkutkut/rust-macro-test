#![doc = include_str!("../README.md")]
#[macro_export]
macro_rules! macro_tests {
	(
		$(
			#[$attr:meta]
		)+
		$ft:ident
		$(,
			$(($test_name:ident, $($ex:expr),* $(,)? )),* $(,)?
		)?
	) => {
		$(
			$(
				#[$attr]
				fn $test_name() {
					$ft($($ex),*);
				}
			)*
		)?
	};
	(
		$ft:ident
		$(,
			$(($test_name:ident, $($ex:expr),* $(,)? )),* $(,)?
		)?
	) => {
		$(
			$(
				#[test]
				fn $test_name() {
					$ft($($ex),*);
				}
			)*
		)?
	};
}
