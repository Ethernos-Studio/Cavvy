//! 字符串 isEmpty 运行时函数

use crate::codegen::context::IRGenerator;

impl IRGenerator {
    /// 生成字符串 isEmpty 运行时函数
    pub(super) fn emit_string_isempty_runtime(&mut self) {
        self.emit_raw("define i1 @__cay_string_isempty(i8* %str) {");
        self.emit_raw("entry:");
        self.emit_raw("  ; 空指针安全检查");
        self.emit_raw("  %is_null = icmp eq i8* %str, null");
        self.emit_raw("  br i1 %is_null, label %null_case, label %normal_case");
        self.emit_raw("");
        self.emit_raw("null_case:");
        self.emit_raw("  ret i1 1");
        self.emit_raw("");
        self.emit_raw("normal_case:");
        self.emit_raw("  %len = call i64 @strlen(i8* %str)");
        self.emit_raw("  %is_empty = icmp eq i64 %len, 0");
        self.emit_raw("  ret i1 %is_empty");
        self.emit_raw("}");
        self.emit_raw("");
    }
}
