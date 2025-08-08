#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! macro_tests {
	(
		ft = $ft:ident
		$(,(
			$test_name:ident
			$(,$ex:expr)* $(,)?
		))* $(,)?
	) => {
		macro_tests!{
			attrs = [],
			ft = $ft
			$(,(
				$test_name
				$(,$ex)*
			))*
		}
	};
	(
		attrs = $attrs:tt,
		ft = $ft:ident
		$(,(
			$test_name:ident
			$(,$ex:expr)* $(,)?
		))* $(,)?
	) => {
		$(
			macro_single_test!{
				attrs = $attrs,
				ft = $ft,
				$test_name
				$(,$ex)*
			}
		)*
	};
	(
		ft = async $ft:ident
		$(,(
			$test_name:ident
			$(,$ex:expr)* $(,)?
		))* $(,)?
	) => {
		macro_tests!{
			attrs = [],
			ft = async $ft
			$(,(
				$test_name
				$(,$ex)*
			))*
		}
	};
	(
		attrs = $attrs:tt,
		ft = async $ft:ident
		$(,(
			$test_name:ident
			$(,$ex:expr)* $(,)?
		))* $(,)?
	) => {
		$(
			macro_single_test!{
				attrs = $attrs,
				ft = async $ft,
				$test_name
				$(,$ex)*
			}
		)*
	};
}

#[macro_export]
macro_rules! macro_single_test {
	(
		attrs = [],
		ft = $ft:ident,
		$test_name:ident
		$(,$ex:expr)* $(,)?
	) => {
		macro_single_test!{
			attrs = [#[test]],
			ft = $ft,
			$test_name
			$(,$ex)*
		}
	};
	(
		attrs = [
			$(#[$attr:meta]),+
		],
		ft = $ft:ident,
		$test_name:ident
		$(,$ex:expr)* $(,)?
	) => {
		$(
			#[$attr]
		)*
		fn $test_name() {
			$ft($($ex),*);
		}
	};
	(
		attrs = [],
		ft = async $ft:ident,
		$test_name:ident
		$(,$ex:expr)* $(,)?
	) => {
		macro_single_test!{
			attrs = [#[test]],
			ft = async $ft,
			$test_name
			$(,$ex)*
		}
	};
	(
		attrs = [
			$(#[$attr:meta]),+
		],
		ft = async $ft:ident,
		$test_name:ident
		$(,$ex:expr)* $(,)?
	) => {
		$(
			#[$attr]
		)*
		async fn $test_name() {
			$ft($($ex),*).await;
		}
	};
}

#[cfg(test)]
mod tests;
