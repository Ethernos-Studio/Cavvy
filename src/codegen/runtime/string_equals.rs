//! String equals 运行时函数
//!
//! 实现 __cay_string_equals 函数，用于比较两个字符串是否相等。

use crate::codegen::context::IRGenerator;

impl IRGenerator {
    /// 生成 string_equals 运行时函数
    /// 比较两个字符串是否相等，返回 i1 (boolean)
    pub(super) fn emit_string_equals_runtime(&mut self) {
        self.emit_raw("; String.equals() 运行时函数");
        self.emit_raw("define i1 @__cay_string_equals(i8* %str1, i8* %str2) {");
        self.emit_raw("entry:");
        
        // 处理 null 情况
        self.emit_raw("  %str1_is_null = icmp eq i8* %str1, null");
        self.emit_raw("  %str2_is_null = icmp eq i8* %str2, null");
        self.emit_raw("  br i1 %str1_is_null, label %str1_null_case, label %str1_not_null");
        
        // str1 是 null 的情况
        self.emit_raw("str1_null_case:");
        self.emit_raw("  ; 如果 str1 是 null，只有当 str2 也是 null 时才相等");
        self.emit_raw("  %both_null = icmp eq i1 %str1_is_null, %str2_is_null");
        self.emit_raw("  ret i1 %both_null");
        
        // str1 不是 null 的情况
        self.emit_raw("str1_not_null:");
        self.emit_raw("  br i1 %str2_is_null, label %str2_null_case, label %both_not_null");
        
        // str2 是 null 但 str1 不是 null
        self.emit_raw("str2_null_case:");
        self.emit_raw("  ret i1 0");
        
        // 两者都不是 null，使用 strcmp 比较
        self.emit_raw("both_not_null:");
        self.emit_raw("  %cmp_result = call i32 @strcmp(i8* %str1, i8* %str2)");
        self.emit_raw("  %is_equal = icmp eq i32 %cmp_result, 0");
        self.emit_raw("  ret i1 %is_equal");
        
        self.emit_raw("}");
        self.emit_raw("");
    }
}
