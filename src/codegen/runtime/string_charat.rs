//! 字符串字符获取运行时函数

use crate::codegen::context::IRGenerator;

impl IRGenerator {
    /// 生成字符串字符获取运行时函数
    pub(super) fn emit_string_charat_runtime(&mut self) {
        self.emit_raw("define i8 @__eol_string_charat(i8* %str, i32 %index) {");
        self.emit_raw("entry:");
        self.emit_raw("  ; 空指针安全检查");
        self.emit_raw("  %is_null = icmp eq i8* %str, null");
        self.emit_raw("  br i1 %is_null, label %out_of_bounds, label %check_bounds");
        self.emit_raw("");
        self.emit_raw("check_bounds:");
        self.emit_raw("  %len = call i64 @strlen(i8* %str)");
        self.emit_raw("  %len_i32 = trunc i64 %len to i32");
        self.emit_raw("  %index_neg = icmp slt i32 %index, 0");
        self.emit_raw("  %index_too_large = icmp sge i32 %index, %len_i32");
        self.emit_raw("  %out_of_range = or i1 %index_neg, %index_too_large");
        self.emit_raw("  br i1 %out_of_range, label %out_of_bounds, label %get_char");
        self.emit_raw("");
        self.emit_raw("out_of_bounds:");
        self.emit_raw("  ret i8 0");
        self.emit_raw("");
        self.emit_raw("get_char:");
        self.emit_raw("  %idx_i64 = sext i32 %index to i64");
        self.emit_raw("  %char_ptr = getelementptr i8, i8* %str, i64 %idx_i64");
        self.emit_raw("  %char_val = load i8, i8* %char_ptr");
        self.emit_raw("  ret i8 %char_val");
        self.emit_raw("}");
        self.emit_raw("");
    }
}
