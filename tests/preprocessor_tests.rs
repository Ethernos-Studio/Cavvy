//! Cavvy 语言预处理器集成测试
//!
//! 测试 #include、#pragma once 等预处理器功能

mod common;
use common::{compile_and_run_eol, compile_eol_expect_error};

// ==================== 0.3.5.0 预处理器 #include 测试 ====================

#[test]
fn test_include_basic() {
    let output = compile_and_run_eol("examples/test_include_basic.cay").expect("include basic should compile and run");
    assert!(output.contains("Version test"),
            "Should show version test message, got: {}", output);
    assert!(output.contains("Addition test"),
            "Should show addition test message, got: {}", output);
    assert!(output.contains("Include test PASSED!"),
            "Include basic test should pass, got: {}", output);
}

#[test]
fn test_include_nested() {
    let output = compile_and_run_eol("examples/test_include_nested.cay").expect("include nested should compile and run");
    assert!(output.contains("Nested include test PASSED!"),
            "Nested include test should pass, got: {}", output);
}

#[test]
fn test_include_pragma_once() {
    let output = compile_and_run_eol("examples/test_include_pragma_once.cay").expect("include pragma once should compile and run");
    assert!(output.contains("Pragma once test PASSED!"),
            "Pragma once test should pass (multiple includes handled correctly), got: {}", output);
}

#[test]
fn test_error_include_cycle() {
    let error = compile_eol_expect_error("examples/errors/error_include_cycle.cay")
        .expect("cyclic include should fail to compile");
    assert!(
        error.contains("循环包含") || error.contains("cyclic") || error.contains("circular") || error.contains("include"),
        "Should report cyclic include error, got: {}",
        error
    );
}
