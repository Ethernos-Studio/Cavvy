# StringPlus - 字符串增强工具类

## 版本信息

- **版本**: 0.5.0.0
- **文件**: `caylibs/StringPlus.cay`
- **依赖**: 无

## 概述

StringPlus 提供了 Cavvy 标准字符串之外的增强功能，包括字符串分割、格式化等实用工具方法。所有方法均为静态，无需实例化即可使用。

**核心特性**:
- 字符串分割（按空格或自定义分隔符）
- 字符串格式化（位置参数和索引参数）
- 纯静态方法设计
- 零依赖

**时间复杂度**: 各方法标注
**空间复杂度**: O(n) n=结果大小

## 静态方法

### split(String) - 按空格分割

将字符串按空格分割成字符串数组。

**时间复杂度**: O(n) n=字符串长度
**空间复杂度**: O(k) k=分割后的段数

```cay
// 按空格分割
String[] parts = StringPlus.split("Hello World Cavvy");
// 结果: ["Hello", "World", "Cavvy"]

// 空字符串返回空数组
String[] empty = StringPlus.split("");
// 结果: []
```

### split(String, String) - 按指定分隔符分割

将字符串按指定分隔符分割成字符串数组。

**时间复杂度**: O(n*m) n=字符串长度, m=分隔符长度
**空间复杂度**: O(k) k=分割后的段数

```cay
// 按逗号分割
String[] parts = StringPlus.split("apple,banana,cherry", ",");
// 结果: ["apple", "banana", "cherry"]

// 按多字符分隔符分割
String[] lines = StringPlus.split("line1\r\nline2\r\nline3", "\r\n");
// 结果: ["line1", "line2", "line3"]

// 空分隔符退化为空格分割
String[] parts = StringPlus.split("a b c", "");
// 结果: ["a", "b", "c"]
```

### format(String, String...) - 位置参数格式化

使用 `{}` 作为占位符，按顺序替换为参数。

**时间复杂度**: O(n*m) n=模板长度, m=参数总长度
**空间复杂度**: O(n+m)

```cay
// 基本格式化
String result = StringPlus.format("Hello, {}!", "World");
// 结果: "Hello, World!"

// 多个参数
String result = StringPlus.format("{} + {} = {}", "1", "2", "3");
// 结果: "1 + 2 = 3"

// 混合文本
String result = StringPlus.format("Name: {}, Age: {}", "Alice", "25");
// 结果: "Name: Alice, Age: 25"

// 参数多于占位符（多余参数忽略）
String result = StringPlus.format("Hello {}", "World", "Extra");
// 结果: "Hello World"

// 占位符多于参数（未替换的保持原样）
String result = StringPlus.format("{} and {}", "First");
// 结果: "First and {}"
```

### formatIndexed(String, String...) - 索引参数格式化

使用 `{0}`, `{1}`, ... 作为占位符，按索引替换为参数。

**时间复杂度**: O(n*m*k) n=模板长度, m=参数数量, k=平均索引长度
**空间复杂度**: O(n+m)

```cay
// 基本格式化
String result = StringPlus.formatIndexed("Hello, {0}!", "World");
// 结果: "Hello, World!"

// 重复使用参数
String result = StringPlus.formatIndexed("{0} {1} {0}", "A", "B");
// 结果: "A B A"

// 参数可以乱序使用
String result = StringPlus.formatIndexed("{1} comes before {0}", "second", "first");
// 结果: "first comes before second"

// 多个参数
String result = StringPlus.formatIndexed("Name: {0}, Age: {1}, City: {2}", 
    "Alice", "25", "Beijing");
// 结果: "Name: Alice, Age: 25, City: Beijing"
```

## 使用示例

### 命令行参数解析

```cay
#include <StringPlus.cay>

public class Main {
    public static void main(String[] args) {
        String command = "create user --name Alice --age 25";
        
        // 分割命令
        String[] parts = StringPlus.split(command);
        
        // 解析
        String action = parts[0];      // "create"
        String target = parts[1];      // "user"
        
        println("Action: " + action);
        println("Target: " + target);
    }
}
```

### CSV解析

```cay
#include <StringPlus.cay>

public class Main {
    public static void main(String[] args) {
        String csvLine = "Alice,25,Beijing,Engineer";
        
        // 分割CSV行
        String[] fields = StringPlus.split(csvLine, ",");
        
        String name = fields[0];      // "Alice"
        String age = fields[1];       // "25"
        String city = fields[2];      // "Beijing"
        String job = fields[3];       // "Engineer"
        
        println("Name: " + name);
        println("Age: " + age);
        println("City: " + city);
        println("Job: " + job);
    }
}
```

### 日志格式化

```cay
#include <StringPlus.cay>

public class Logger {
    public static void info(String template, String... args) {
        String message = StringPlus.format(template, args);
        println("[INFO] " + message);
    }
    
    public static void error(String template, String... args) {
        String message = StringPlus.format(template, args);
        println("[ERROR] " + message);
    }
}

public class Main {
    public static void main(String[] args) {
        Logger.info("User {} logged in from {}", "Alice", "192.168.1.1");
        // 输出: [INFO] User Alice logged in from 192.168.1.1
        
        Logger.error("Failed to connect to {}:{}", "database", "5432");
        // 输出: [ERROR] Failed to connect to database:5432
    }
}
```

### 模板消息

```cay
#include <StringPlus.cay>

public class Main {
    public static void main(String[] args) {
        // 邮件模板
        String emailTemplate = "Dear {0},\n\n" +
            "Your order #{1} has been {2}.\n\n" +
            "Thank you for your business!\n\n" +
            "Best regards,\n{3}";
        
        String email = StringPlus.formatIndexed(emailTemplate, 
            "Alice",           // {0}
            "12345",           // {1}
            "shipped",         // {2}
            "Customer Service" // {3}
        );
        
        println(email);
        // 输出:
        // Dear Alice,
        //
        // Your order #12345 has been shipped.
        //
        // Thank you for your business!
        //
        // Best regards,
        // Customer Service
    }
}
```

### URL路径解析

```cay
#include <StringPlus.cay>

public class Main {
    public static void main(String[] args) {
        String urlPath = "/api/v1/users/123/posts/456";
        
        // 分割路径
        String[] segments = StringPlus.split(urlPath, "/");
        
        // 解析API端点
        String apiVersion = segments[1];    // "v1"
        String resource = segments[2];      // "users"
        String resourceId = segments[3];    // "123"
        String subResource = segments[4];   // "posts"
        String subResourceId = segments[5]; // "456"
        
        println("API: " + apiVersion);
        println("Resource: " + resource + "/" + resourceId);
        println("SubResource: " + subResource + "/" + subResourceId);
    }
}
```

### 配置文件解析

```cay
#include <StringPlus.cay>

public class ConfigParser {
    public static void parseLine(String line) {
        // 跳过空行和注释
        if (line.isEmpty() || line.startsWith("#")) {
            return;
        }
        
        // 分割键值对
        String[] parts = StringPlus.split(line, "=");
        if (parts.length == 2) {
            String key = parts[0].trim();
            String value = parts[1].trim();
            println("Key: " + key + ", Value: " + value);
        }
    }
}

public class Main {
    public static void main(String[] args) {
        String config = "name=MyApp\nversion=1.0.0\nauthor=Alice";
        String[] lines = StringPlus.split(config, "\n");
        
        int i = 0;
        while (i < lines.length) {
            ConfigParser.parseLine(lines[i]);
            i = i + 1;
        }
    }
}
```

## 性能优化

### 时间复杂度对比

| 操作 | StringPlus | 手动实现 |
|------|-----------|----------|
| split | O(n) | O(n) |
| format | O(n*m) | O(n*m) |

### 最佳实践

1. **选择合适的分割方法**: 简单空格分割用 `split(String)`，复杂分隔符用 `split(String, String)`
2. **format vs formatIndexed**: 顺序替换用 format，乱序/重复用 formatIndexed
3. **避免过度分割**: 大数据量考虑流式处理
4. **缓存模板**: 重复使用的模板字符串建议缓存

## 与StringBuilder的配合使用

```cay
#include <StringPlus.cay>
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        // 先用StringBuilder构建复杂模板
        StringBuilder template = new StringBuilder();
        template.append("User: {0}").appendln();
        template.append("Time: {1}").appendln();
        template.append("Action: {2}");
        
        // 再用StringPlus格式化
        String result = StringPlus.formatIndexed(
            template.toString(),
            "Alice",
            "2024-01-01 12:00:00",
            "LOGIN"
        );
        
        println(result);
    }
}
```

## 注意事项

1. **空字符串处理**: `split("")` 返回空数组而非包含空字符串的数组
2. **分隔符不存在**: `split` 返回包含原字符串的单元素数组
3. **format占位符**: `{}` 必须成对出现，不支持转义
4. **formatIndexed索引**: 索引从 0 开始，越界索引保持原样
5. **性能考虑**: 大量格式化操作建议使用 StringBuilder
