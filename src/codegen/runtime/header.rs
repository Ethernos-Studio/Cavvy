//! IR 头部生成
//!
//! 生成外部函数声明和全局常量

use crate::codegen::context::IRGenerator;

/// 发射IR头部（外部声明和运行时函数）
pub fn emit(gen: &mut IRGenerator) {
    gen.emit_raw("; EOL (Ethernos Object Language) Generated LLVM IR");
    gen.emit_raw("target triple = \"x86_64-w64-mingw32\"");
    gen.emit_raw("");

    // 声明外部函数 (printf 和标准C库函数)
    gen.emit_raw("declare i32 @printf(i8*, ...)");
    gen.emit_raw("declare i32 @scanf(i8*, ...)");
    gen.emit_raw("declare void @SetConsoleOutputCP(i32)");
    gen.emit_raw("declare i64 @strlen(i8*)");
    gen.emit_raw("declare i8* @calloc(i64, i64)");
    gen.emit_raw("declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg)");
    gen.emit_raw("declare i32 @snprintf(i8*, i64, i8*, ...)");
    gen.emit_raw("@.str.float_fmt = private unnamed_addr constant [3 x i8] c\"%f\\00\", align 1");
    gen.emit_raw("@.str.int_fmt = private unnamed_addr constant [5 x i8] c\"%lld\\00\", align 1");
    gen.emit_raw("@.str.true_str = private unnamed_addr constant [5 x i8] c\"true\\00\", align 1");
    gen.emit_raw("@.str.false_str = private unnamed_addr constant [6 x i8] c\"false\\00\", align 1");
    gen.emit_raw("");

    // 空字符串常量（用于 null 安全）
    gen.emit_raw("@.eol_empty_str = private unnamed_addr constant [1 x i8] c\"\\00\", align 1");
    gen.emit_raw("");

    // 生成运行时函数
    gen.emit_string_concat_runtime();
    gen.emit_float_to_string_runtime();
    gen.emit_int_to_string_runtime();
    gen.emit_bool_to_string_runtime();
    gen.emit_char_to_string_runtime();
    gen.emit_string_length_runtime();
    gen.emit_string_substring_runtime();
    gen.emit_string_indexof_runtime();
    gen.emit_string_charat_runtime();
    gen.emit_string_replace_runtime();
}
