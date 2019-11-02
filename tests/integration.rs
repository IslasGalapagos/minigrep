use std::fs;

use assert_cli;

mod common;

#[test]
fn calling_without_args() {
    assert_cli::Assert::main_binary()
      .fails()
      .and()
      .stderr().contains("Didn't get a query string")
      .unwrap();
}

#[test]
fn calling_without_first_arg() {
    assert_cli::Assert::main_binary()
      .with_args(&["x"])
      .fails()
      .and()
      .stderr().contains("Didn't get a file name")
      .unwrap();
}

#[test]
fn calling_with_all_args() {
    let file_name = "x.txt";
    let content = "abc";

    common::create_and_write_file(file_name, content).unwrap();

    assert_cli::Assert::main_binary()
      .with_args(&[content, file_name])
      .unwrap();

    fs::remove_file(file_name).unwrap();
}