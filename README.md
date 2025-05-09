# rust-macro-test
This macro simplifies running multiple tests with less boilerplate code.

# Examples
```rust
use macro_test::macro_tests;

#[test]
fn my_test_function(arg1: i32, arg2: i32) {
    assert_eq!(arg1, arg2);
}

macro_tests!(my_test_function,
    (test_1, 1, 1),
    (test_2, 1 + 1 - 2 + 2, 2)
);
```

The previous code is equivalent to the following:
```rust
#[test]
fn my_test_function(arg1: i32, arg2: i32) {
    assert_eq!(arg1, arg2);
}

#[test]
fn test_1() {
    my_test_function(1, 1);
}

#[test]
fn test_2() {
    my_test_function(1 + 1 - 2 + 2, 2);
}
```
