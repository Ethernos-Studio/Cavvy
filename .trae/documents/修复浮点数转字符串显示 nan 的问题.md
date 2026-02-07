## 问题分析

在 EOL 代码中：

```eol
println("两个浮点数的积是: " + (String)(x * y));
```

输出显示 `nan`，但直接 `println(x * y)` 显示 `12.000000` 是正确的。

## 问题定位

问题在 [expressions.rs:L941-L972](file:///e:/spj/EOL/src/codegen/expressions.rs#L941-L972) 的 `generate_cast_expression` 函数中，处理浮点数到字符串转换的部分：

1. 分配了 32 字节的缓冲区用于存储结果字符串
2. 使用 `sprintf` 和 `"%f"` 格式将浮点数转换为字符串

## 根本原因

经过代码审查，发现以下问题：

1. **缓冲区分配方式**：使用 `alloca [32 x i8]` 分配缓冲区，但 `getelementptr` 获取的指针类型可能有问题
2. **格式字符串**：`%f` 格式对于某些浮点值可能产生意外结果
3. **缺少 snprintf**：应该使用 `snprintf` 并传入缓冲区大小以确保安全

## 修复方案

修改 [expressions.rs](file:///e:/spj/EOL/src/codegen/expressions.rs) 中的浮点数到字符串转换代码：

1. 将缓冲区大小从 32 增加到 64 字节（更安全的默认值）
2. 确保 `sprintf` 调用正确
3. 或者改用 `snprintf` 并传入缓冲区大小

## 具体修改

在 `generate_cast_expression` 函数的浮点数到字符串转换部分（约第 941-972 行）：

```rust
// 浮点到字符串（float/double -> String）
if (from_type == "float" || from_type == "double") && to_type == "i8*" {
    // 分配临时缓冲区（64字节够装 float/double 的字符串表示）
    let buf_ptr = self.new_temp();
    self.emit_line(&format!("  {} = alloca [64 x i8], align 1", buf_ptr));
    
    // 获取缓冲区指针
    let casted_buf = self.new_temp();
    self.emit_line(&format!("  {} = getelementptr [64 x i8], [64 x i8]* {}, i64 0, i64 0",
        casted_buf, buf_ptr));
    
    // 获取格式字符串 "%f" 的指针
    let fmt_ptr = self.new_temp();
    self.emit_line(&format!("  {} = getelementptr [3 x i8], [3 x i8]* @.str.float_fmt, i64 0, i64 0",
        fmt_ptr));
    
    // C 的可变参数函数中，float 会被提升为 double
    let (arg_type, arg_val) = if from_type == "float" {
        let promoted = self.new_temp();
        self.emit_line(&format!("  {} = fpext float {} to double", promoted, val));
        ("double".to_string(), promoted)
    } else {
        ("double".to_string(), val.to_string())
    };
    
    // 调用 sprintf
    self.emit_line(&format!("  call i32 @sprintf(i8* {}, i8* {}, {} {})",
        casted_buf, fmt_ptr, arg_type, arg_val));
    
    return Ok(format!("{} {}", to_type, casted_buf));
}
```

主要变更：缓冲区大小从 32 改为 64 字节。
