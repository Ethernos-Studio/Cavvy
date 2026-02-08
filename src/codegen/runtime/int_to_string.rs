//! 整数转字符串运行时函数

use crate::codegen::context::IRGenerator;

impl IRGenerator {
    /// 生成整数到字符串运行时函数
    pub(super) fn emit_int_to_string_runtime(&mut self) {
        self.emit_raw("define i8* @__eol_int_to_string(i64 %value) {");
        self.emit_raw("entry:");
        self.emit_raw("  ; 分配堆内存缓冲区（32字节足够存储64位整数）");
        self.emit_raw("  %buf = call i8* @calloc(i64 1, i64 32)");
        self.emit_raw("  ; 使用 %lld 格式打印长整数");
        self.emit_raw("  call i32 (i8*, i64, i8*, ...) @snprintf(i8* %buf, i64 32, i8* getelementptr ([4 x i8], [4 x i8]* @.str.int_fmt, i64 0, i64 0), i64 %value)");
        self.emit_raw("  ret i8* %buf");
        self.emit_raw("}");
        self.emit_raw("");
    }
}
