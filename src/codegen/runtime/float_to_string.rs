//! 浮点数转字符串运行时函数

use crate::codegen::context::IRGenerator;

impl IRGenerator {
    /// 生成浮点数转字符串运行时函数
    pub(super) fn emit_float_to_string_runtime(&mut self) {
        // 使用一个包装函数来确保正确的调用约定
        // 注意：使用 calloc 分配堆内存（自动零初始化），而不是 alloca 分配栈内存
        self.emit_raw("define i8* @__eol_float_to_string(double %value) {");
        self.emit_raw("entry:");
        self.emit_raw("  ; 分配堆内存缓冲区（64字节，8字节对齐，使用 calloc 自动零初始化）");
        self.emit_raw("  %buf = call i8* @calloc(i64 1, i64 64)");
        self.emit_raw("  %fmt_ptr = getelementptr [3 x i8], [3 x i8]* @.str.float_fmt, i64 0, i64 0");
        self.emit_raw("  ; 调用 snprintf（指定缓冲区大小）");
        self.emit_raw("  call i32 (i8*, i64, i8*, ...) @snprintf(i8* %buf, i64 64, i8* %fmt_ptr, double %value)");
        self.emit_raw("  ret i8* %buf");
        self.emit_raw("}");
        self.emit_raw("");
    }
}
