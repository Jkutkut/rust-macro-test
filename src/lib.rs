#![doc = include_str!("../README.md")]
#[macro_export]
macro_rules! macro_tests {
	(
		$ft:ident $(,)?
	) => {};
	(
		attrs = [ $(#[$attr:meta])+ ],
		$ft:ident $(,)?
	) => {};
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
	(
		attrs = [ #[$attr:meta] ],
		$ft:ident,
		$(($test_name:ident, $($ex:expr),* $(,)? )),+ $(,)?
	) => {
		$(
			#[$attr]
			fn $test_name() {
				$ft($($ex),*);
			}
		)*
	};
	(
		attrs = [ $(#[$attr:meta])+ ],
		$ft:ident,
		$(($test_name:ident, $($ex:expr),* $(,)? )),+ $(,)?
	) => {
		$(
			#[$attr]
			fn $test_name() {
				$ft($($ex),*);
			}
		)*
	}
}

#[cfg(test)]
mod tests;
