//! 字符串长度运行时函数

use crate::codegen::context::IRGenerator;

impl IRGenerator {
    /// 生成字符串长度运行时函数
    pub(super) fn emit_string_length_runtime(&mut self) {
        self.emit_raw("define i32 @__cay_string_length(i8* %str) {");
        self.emit_raw("entry:");
        self.emit_raw("  ; 空指针安全检查");
        self.emit_raw("  %is_null = icmp eq i8* %str, null");
        self.emit_raw("  br i1 %is_null, label %null_case, label %normal_case");
        self.emit_raw("");
        self.emit_raw("null_case:");
        self.emit_raw("  ret i32 0");
        self.emit_raw("");
        self.emit_raw("normal_case:");
        self.emit_raw("  %len = call i64 @strlen(i8* %str)");
        self.emit_raw("  %len_i32 = trunc i64 %len to i32");
        self.emit_raw("  ret i32 %len_i32");
        self.emit_raw("}");
        self.emit_raw("");
    }
}
