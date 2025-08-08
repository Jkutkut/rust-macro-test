use std::process::{
	Command,
	Output,
};

use crate::*;

fn ensure_library_exits() {
	println!("Compiling library...");
	// TODO: prevent recompilation from here
	std::fs::create_dir_all("target").unwrap();
	Command::new("rustc")
		.args(&[
			"--crate-type=lib", "src/lib.rs",
			"-o", "target/lib.rlib",
		])
		.output()
		.expect("Failed to compile library");
}

fn compile(src_file: &str) -> (String, Output) {
	println!("Compiling test {}...", src_file);
	let src_file_path = std::path::PathBuf::from(src_file);
	let filename = src_file_path.file_name().unwrap();

	let test_output_path = std::path::PathBuf::from("target/custom_tests");
	let out_file_path = test_output_path.join(filename);

	let out_file_path = out_file_path.to_str().unwrap();
	let test_output_path = test_output_path.to_str().unwrap();

	println!("target_directory: {}", test_output_path);
	println!("out file: {}", out_file_path);
	println!("src file: {}", src_file);
	std::fs::create_dir_all(test_output_path).unwrap();
	(
		out_file_path.to_string(),
		Command::new("rustc")
			.args(&[
				"--test", src_file,
				"--extern", "macro_test=target/lib.rlib",
				"-o", &out_file_path,
			])
			.output()
			.expect("Failed to compile test")
	)
}

fn run_crate_test(crate_path: &str, expected_outputs: &[&str], expected_warnings: &[&str]) {
	let output = Command::new("cargo")
		.args(&["test"])
		.current_dir(crate_path)
		.output()
		.expect("Failed to run test");
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	println!(
		"###### Output crate {} ######\n### stdout ### \n{}\n### stderr ###\n{}####################\n",
		crate_path,
		stdout, stderr
	);
	assert!(output.status.success());
	for output in expected_outputs {
		println!("Output must contain: {}", output);
		assert!(stdout.contains(output));
	}
	for warning in expected_warnings {
		println!("Warning must contain: {}", warning);
		assert!(stderr.contains(warning));
	}
}

fn test_invalid_compilation(filename: &str, expected_errors: &[&str]) {
	ensure_library_exits();
	let (_, output) = compile(filename);
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	println!(
		"###### Output {} ######\n### stdout ### \n{}\n### stderr ###\n{}####################\n",
		filename,
		stdout, stderr
	);
	assert!(!output.status.success());
	for error in expected_errors {
		println!("Error must contain: {}", error);
		assert!(stderr.contains(error));
	}
}

fn test_valid_compilation(filename: &str, expected_outputs: &[&str], expected_warnings: &[&str]) {
	ensure_library_exits();
	let (filename, output) = compile(filename);
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	println!(
		"###### Output {} ######\n### stdout ### \n{}\n### stderr ###\n{}####################\n",
		filename,
		stdout, stderr
	);
	assert!(output.status.success());

	let output = Command::new(&filename)
		.output()
		.expect("Failed to run test");

	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	println!(
		"###### Output {} ######\n### stdout ### \n{}\n### stderr ###\n{}####################\n",
		filename,
		stdout, stderr
	);
	assert!(output.status.success());
	for output in expected_outputs {
		println!("Output must contain: {}", output);
		assert!(stdout.contains(output));
	}
	for warning in expected_warnings {
		println!("Warning must contain: {}", warning);
		assert!(stderr.contains(warning));
	}
}

fn arr2test_results(arr: &[i32]) -> String {
	let tests_results_order = [
		"passed", "failed",
		"ignored", "measured",
		"filtered"
	];
	tests_results_order.iter().enumerate()
		.map(|(i, v)| format!(
			"{} {}",
			if i < arr.len() { arr[i] } else { 0 },
			v
		))
		.collect::<Vec<String>>()
		.join("; ")
}

macro_tests!(
	ft = test_invalid_compilation,
	(invalid_dupped_test, "src/tests/invalid_dupped_test.rs", &[ "`test_1`", "defined multiple times"]),
	(invalid_attr, "src/tests/invalid_attr.rs", &[ "`foo`", "attribute", "in this scope"]),
);

macro_tests!(
	ft = test_valid_compilation,
	(valid_functions, "src/tests/valid_functions.rs", &[&arr2test_results(&[9]), "test_1 ... ok", "test_2 ... ok"], &[]),
	(valid_functions_default, "src/tests/valid_functions_default.rs", &[&arr2test_results(&[2]), "test_1 ... ok", "test_2 ... ok"], &[]),
	(valid_multiple_attr, "src/tests/valid_multiple_attr.rs", &[&arr2test_results(&[4])], &[]),
	(valid_empty, "src/tests/valid_empty.rs", &[&arr2test_results(&[])], &[]),
	(valid_empty_default, "src/tests/valid_empty_default.rs", &[&arr2test_results(&[])], &[]),
	(valid_ft_args, "src/tests/valid_ft_args.rs", &[&arr2test_results(&[36])], &[]),
	(valid_empty_attrs, "src/tests/valid_empty_attrs.rs", &[&arr2test_results(&[])], &[]),
);

macro_tests!(
	ft = run_crate_test,
	(crate_async, "src/tests/async", &[], &[]),
);
