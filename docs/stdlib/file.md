# File - 文件操作标准库

## 版本信息

- **版本**: 0.5.2.0
- **文件**: `caylibs/File.cay`
- **依赖**: `std/ffi.cay`, C标准库 (stdio.h)

## 概述

File 模块提供了全面的文件操作功能，包括文件读写、定位、属性查询和文件系统操作。采用面向对象设计，支持现代 Result 类型错误处理和自动资源管理。

**核心特性**:
- 面向对象API设计
- 自动资源管理（RAII）
- 完整的错误处理（Result类型）
- 流式读写支持
- 线程安全的文件信息查询

## 常量定义

### 文件打开模式

```cay
#define FILE_MODE_READ       "r"      // 只读
#define FILE_MODE_WRITE      "w"      // 只写（创建或截断）
#define FILE_MODE_APPEND     "a"      // 追加
#define FILE_MODE_READPLUS   "r+"     // 读写
#define FILE_MODE_WRITEPLUS  "w+"     // 读写（创建或截断）
#define FILE_MODE_APPENDPLUS "a+"     // 读写追加
```

### 定位原点

```cay
#define SEEK_ORIGIN_SET      0        // 文件开头
#define SEEK_ORIGIN_CUR      1        // 当前位置
#define SEEK_ORIGIN_END      2        // 文件末尾
```

### 错误码

```cay
#define FILE_ERROR_NONE           0   // 无错误
#define FILE_ERROR_NOT_FOUND      1   // 文件不存在
#define FILE_ERROR_ACCESS_DENIED  2   // 权限不足
#define FILE_ERROR_IO             3   // I/O错误
#define FILE_ERROR_INVALID_MODE   4   // 无效打开模式
#define FILE_ERROR_SEEK           5   // 定位错误
#define FILE_ERROR_ALREADY_EXISTS 6   // 文件已存在
#define FILE_ERROR_TOO_LARGE      7   // 文件过大
#define FILE_ERROR_INVALID_PATH   8   // 无效路径
```

## 核心类

### FileMode - 文件打开模式类

```cay
// 静态工厂方法
FileMode.read()        // 只读模式
FileMode.write()       // 只写模式
FileMode.append()      // 追加模式
FileMode.readWrite()   // 读写模式
FileMode.writeRead()   // 写读模式
FileMode.appendRead()  // 追加读模式
FileMode.custom("rb")  // 自定义模式
```

### SeekOrigin - 定位原点类

```cay
SeekOrigin.begin()     // 文件开头
SeekOrigin.current()   // 当前位置
SeekOrigin.end()       // 文件末尾
```

### FileError - 文件错误类型

```cay
FileError.None           // 0
FileError.NotFound       // 1
FileError.AccessDenied   // 2
FileError.IoError        // 3
FileError.InvalidMode    // 4
FileError.SeekError      // 5
FileError.AlreadyExists  // 6
FileError.TooLarge       // 7
FileError.InvalidPath    // 8
FileError.Unknown        // 9
```

### FileResult - 操作结果类型

```cay
// 创建成功结果
FileResult result = FileResult.ok(value);

// 创建错误结果
FileResult result = FileResult.err(FileError.NotFound);

// 检查结果
if (result.isOk()) {
    Object value = result.unwrap();
} else {
    int error = result.getError();
}
```

### FileInfo - 文件信息类

使用 stat-based 方法获取文件信息，避免打开文件。

**时间复杂度**: O(1) 磁盘I/O仅检查元数据

```cay
// 获取文件信息
FileInfo info = FileInfo.fromPath("data.txt");

// 查询属性
bool exists = info.exists();      // 文件是否存在
long size = info.getSize();       // 文件大小（字节）
String path = info.getPath();     // 文件路径
```

### File - 文件操作主类

#### 构造函数

```cay
// 默认构造函数
File file = new File();

// 带路径和模式的构造函数
File file = new File("data.txt", FileMode.read());
```

#### 文件操作

```cay
// 打开文件（传统API）
bool success = file.open("data.txt", FileMode.read());

// 打开文件（Result API）
FileResult result = File.openResult("data.txt", FileMode.read());

// 关闭文件
bool closed = file.close();

// 检查状态
bool isOpen = file.isOpened();
bool eof = file.isEof();
bool hasErr = file.hasError();

// 刷新缓冲区
bool flushed = file.flush();
```

#### 定位操作

```cay
// 获取当前位置
long pos = file.position();

// 定位到指定位置
bool success = file.seek(100, SeekOrigin.begin());   // 从开头偏移100
bool success = file.seek(-50, SeekOrigin.current()); // 从当前位置后退50
bool success = file.seek(0, SeekOrigin.end());       // 定位到末尾

// 重置到文件开头
file.rewind();

// 获取文件大小
long size = file.size();
```

#### 读写操作

```cay
// 字符读写
int charCode = file.readChar();              // 读取一个字符
bool written = file.writeChar(65);           // 写入字符'A'

// 字符串读写
String line = file.readLine(1024);           // 读取一行（最大1024字符）
bool written = file.writeString("Hello");    // 写入字符串
bool written = file.writeLine("World");      // 写入字符串并换行

// 模板字符串写入（插值）
int written = file.writeInterpolated("Name: {}, Age: {}", "Alice", "25");

// 读取全部内容
String content = file.readAllText();
bool written = file.writeAllText(content);

// 二进制读写
long bytesRead = file.readBytes(buffer, size);
long bytesWritten = file.writeBytes(buffer, size);
```

#### 静态工具方法

```cay
// 文件存在检查
bool exists = File.exists("data.txt");
FileResult result = File.existsResult("data.txt");

// 获取文件大小（不打开文件）
long size = File.getSize("data.txt");

// 文件操作
bool deleted = File.delete("old.txt");
bool moved = File.move("old.txt", "new.txt");
bool copied = File.copy("source.txt", "dest.txt");

// 读写整个文件
String content = File.readAllText("data.txt");
bool written = File.writeAllText("data.txt", content);
bool appended = File.appendAllText("data.txt", "more content");

// 行操作
String[] lines = File.readAllLines("data.txt");
bool written = File.writeAllLines("data.txt", lines);
```

### FileReader - 文件读取器

简化文件读取的包装类。

```cay
// 创建读取器
FileReader reader = new FileReader("data.txt");

// 读取操作
String content = reader.readToEnd();
String line = reader.readLine();
bool eof = reader.isEof();

// 行迭代器（流式读取）
LineIterator it = reader.lines();
while (it.hasNext()) {
    String line = it.next();
    println(line);
}

// 关闭
reader.close();
```

### FileWriter - 文件写入器

简化文件写入的包装类。

```cay
// 创建写入器（覆盖模式）
FileWriter writer = new FileWriter("output.txt");

// 创建写入器（追加模式）
FileWriter writer = new FileWriter("output.txt", true);

// 写入操作
writer.write("Hello");
writer.writeLine("World");
writer.flush();

// 关闭
writer.close();
```

### FileUtils - 文件工具类

```cay
// 路径操作
String fileName = FileUtils.getFileName("/path/to/file.txt");           // "file.txt"
String ext = FileUtils.getExtension("/path/to/file.txt");               // ".txt"
String dir = FileUtils.getDirectoryName("/path/to/file.txt");           // "/path/to"
String nameNoExt = FileUtils.getFileNameWithoutExtension("file.txt");   // "file"

// 路径组合
String combined = FileUtils.combine("/path", "file.txt");               // "/path/file.txt"

// 修改扩展名
String newPath = FileUtils.changeExtension("file.txt", ".md");          // "file.md"
```

## 使用示例

### 基本文件读写

```cay
#include <File.cay>

public class Main {
    public static void main(String[] args) {
        // 写入文件
        FileWriter writer = new FileWriter("test.txt");
        writer.writeLine("第一行");
        writer.writeLine("第二行");
        writer.close();
        
        // 读取文件
        FileReader reader = new FileReader("test.txt");
        String line;
        while (!(line = reader.readLine()).isEmpty()) {
            println(line);
        }
        reader.close();
    }
}
```

### 使用Result API处理错误

```cay
#include <File.cay>

public class Main {
    public static void main(String[] args) {
        FileResult result = File.openResult("data.txt", FileMode.read());
        
        if (result.isErr()) {
            int error = result.getError();
            if (error == FileError.NotFound) {
                println("文件不存在");
            } else if (error == FileError.AccessDenied) {
                println("权限不足");
            }
            return;
        }
        
        // 安全地解包结果
        File file = (File)result.unwrap();
        String content = file.readAllText();
        file.close();
        
        println(content);
    }
}
```

### 流式处理大文件

```cay
#include <File.cay>

public class Main {
    public static void main(String[] args) {
        // 流式读取避免内存峰值
        FileReader reader = new FileReader("large_file.txt");
        LineIterator it = reader.lines();
        
        int lineCount = 0;
        while (it.hasNext()) {
            String line = it.next();
            // 处理每一行...
            lineCount = lineCount + 1;
        }
        
        reader.close();
        println("总行数: " + lineCount);
    }
}
```

### 模板字符串写入

```cay
#include <File.cay>

public class Main {
    public static void main(String[] args) {
        File file = new File();
        file.open("report.txt", FileMode.write());
        
        String name = "Alice";
        int age = 25;
        float score = 95.5;
        
        // 使用模板写入
        file.writeInterpolated("Name: {}, Age: {}, Score: {}\n", name, age, score);
        
        file.close();
    }
}
```

### 文件复制

```cay
#include <File.cay>

public class Main {
    public static void main(String[] args) {
        bool success = File.copy("source.txt", "dest.txt");
        if (success) {
            println("复制成功");
        } else {
            println("复制失败");
        }
    }
}
```

## 性能优化

### 时间复杂度

| 操作 | 时间复杂度 | 说明 |
|------|-----------|------|
| open | O(1) | 系统调用 |
| close | O(1) | 系统调用 |
| readChar | O(1) | 缓冲读取 |
| writeChar | O(1) | 缓冲写入 |
| readLine | O(n) | n=行长度 |
| readAllText | O(n) | n=文件大小 |
| seek | O(1) | 系统调用 |
| size | O(1) | stat-based |

### 磁盘I/O优化

1. **批量读写**: 使用 readBytes/writeBytes 减少系统调用
2. **缓冲写入**: 使用 writeLine 而非多次 writeChar
3. **流式处理**: 使用 LineIterator 处理大文件
4. **stat-based查询**: FileInfo 避免打开文件获取大小

## 注意事项

1. **始终关闭文件**: 使用 finalize 自动关闭，或显式调用 close
2. **检查返回值**: 文件操作可能失败，特别是网络文件系统
3. **路径长度限制**: 最大路径长度 260 字符
4. **并发访问**: 文件锁需要额外实现
5. **编码问题**: 当前版本使用系统默认编码
