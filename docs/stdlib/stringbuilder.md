# StringBuilder - 可变字符串构建器

## 版本信息

- **版本**: 0.5.1.0
- **文件**: `caylibs/StringBuilder.cay`
- **依赖**: 外部辅助函数 (`__cay_memcpy_byte`, `__cay_memset_byte`)

## 概述

StringBuilder 提供了高效的字符串拼接操作，避免了字符串不可变性带来的性能问题。使用动态数组作为内部缓冲区，支持自动扩容和链式调用。

**核心特性**:
- 高效的字符串拼接（O(n) 而非 O(n²)）
- 自动容量管理
- 链式 API 设计
- 支持多种数据类型
- 内存安全（RAII）

**时间复杂度**: 追加操作 O(1) amortized
**空间复杂度**: O(n) n=字符串长度

## 核心类

### StringBuilder

#### 构造函数

```cay
// 默认构造函数（初始容量16）
StringBuilder sb = new StringBuilder();

// 指定初始容量
StringBuilder sb = new StringBuilder(256);

// 从初始字符串构造
StringBuilder sb = new StringBuilder("Hello");
```

#### 追加操作

```cay
// 追加字符串
sb.append("Hello")
  .append(" ")
  .append("World");

// 追加字符
sb.append('!');

// 追加整数
sb.append(42);

// 追加长整数
sb.append(1234567890L);

// 追加布尔值
sb.append(true);

// 追加字符数组
char[] chars = {'a', 'b', 'c'};
sb.append(chars);

// 追加换行符
sb.appendln();

// 追加字符串并换行
sb.appendln("New line");
```

#### 插入和删除

```cay
// 在指定位置插入字符串
sb.insert(5, " Beautiful");

// 删除指定范围的字符
sb.delete(5, 15);

// 删除单个字符（通过范围）
sb.delete(5, 6);
```

#### 替换和反转

```cay
// 替换所有出现的子串
sb.replace("old", "new");

// 反转字符串
sb.reverse();
```

#### 查询操作

```cay
// 获取长度
int len = sb.length();

// 获取容量
int cap = sb.capacity();

// 检查是否为空
bool empty = sb.isEmpty();

// 获取指定位置的字符
char c = sb.charAt(5);

// 查找子串位置
int index = sb.indexOf("search");
```

#### 子串操作

```cay
// 截取子串（从位置到末尾）
String sub = sb.substring(5);

// 截取子串（指定范围）
String sub = sb.substring(5, 10);
```

#### 修改操作

```cay
// 清空
sb.clear();

// 设置长度（截断或扩展）
sb.setLength(10);
```

#### 转换操作

```cay
// 转换为字符串
String result = sb.toString();

// 获取C字符串指针（用于FFI）
long ptr = sb.c_str();

// 从C字符串创建
StringBuilder sb2 = StringBuilder.fromCString(ptr);
```

## 使用示例

### 基本字符串构建

```cay
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        
        sb.append("Hello")
          .append(" ")
          .append("World")
          .append("!");
        
        String result = sb.toString();
        println(result);  // 输出: Hello World!
    }
}
```

### 构建复杂字符串

```cay
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        
        // 构建HTML
        sb.append("<html>").appendln();
        sb.append("  <head>").appendln();
        sb.append("    <title>").append("My Page").append("</title>").appendln();
        sb.append("  </head>").appendln();
        sb.append("  <body>").appendln();
        sb.append("    <h1>").append("Welcome").append("</h1>").appendln();
        sb.append("  </body>").appendln();
        sb.append("</html>");
        
        println(sb.toString());
    }
}
```

### 格式化输出

```cay
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        
        String name = "Alice";
        int age = 25;
        float score = 95.5;
        
        sb.append("Name: ").append(name).appendln();
        sb.append("Age: ").append(age).appendln();
        sb.append("Score: ").append(score).appendln();
        
        println(sb.toString());
        // 输出:
        // Name: Alice
        // Age: 25
        // Score: 95.5
        
    }
}
```

### 循环构建字符串

```cay
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        
        // 构建CSV数据
        sb.append("id,name,value").appendln();
        
        int i = 0;
        while (i < 100) {
            sb.append(i).append(",")
              .append("Item").append(i).append(",")
              .append(i * 10)
              .appendln();
            i = i + 1;
        }
        
        // 比字符串拼接高效得多
        String csv = sb.toString();
    }
}
```

### 字符串修改

```cay
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder("Hello World");
        
        // 插入
        sb.insert(5, " Beautiful");
        println(sb.toString());  // Hello Beautiful World
        
        // 替换
        sb.replace("Beautiful", "Amazing");
        println(sb.toString());  // Hello Amazing World
        
        // 删除
        sb.delete(5, 13);
        println(sb.toString());  // Hello World
        
        // 反转
        sb.reverse();
        println(sb.toString());  // dlroW olleH
    }
}
```

### 性能对比示例

```cay
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        int n = 10000;
        
        // 方法1: 字符串拼接（慢，O(n²)）
        // String result = "";
        // int i = 0;
        // while (i < n) {
        //     result = result + i;  // 每次创建新字符串
        //     i = i + 1;
        // }
        
        // 方法2: StringBuilder（快，O(n)）
        StringBuilder sb = new StringBuilder();
        int i = 0;
        while (i < n) {
            sb.append(i);
            i = i + 1;
        }
        String result = sb.toString();
        
        println("完成，长度: " + result.length());
    }
}
```

### 预分配容量优化

```cay
#include <StringBuilder.cay>

public class Main {
    public static void main(String[] args) {
        // 预估需要的容量，避免多次扩容
        int estimatedSize = 10000;
        StringBuilder sb = new StringBuilder(estimatedSize);
        
        int i = 0;
        while (i < 1000) {
            sb.append("This is line ").append(i).appendln();
            i = i + 1;
        }
        
        println(sb.toString());
    }
}
```

## 性能优化

### 时间复杂度分析

| 操作 | 时间复杂度 | 说明 |
|------|-----------|------|
| append(String) | O(n) amortized | n=追加长度 |
| append(char) | O(1) amortized | 常数时间 |
| insert | O(n) | 需要移动元素 |
| delete | O(n) | 需要移动元素 |
| reverse | O(n) | 遍历一半元素 |
| replace | O(n+m) | n=原串, m=新串 |
| toString | O(n) | 复制数据 |
| indexOf | O(n*m) | n=长度, m=子串长度 |

### 空间复杂度分析

| 操作 | 空间复杂度 | 说明 |
|------|-----------|------|
| 构造函数 | O(capacity) | 初始容量 |
| append | O(n) | 总空间需求 |
| 扩容 | O(n) | 临时需要双倍空间 |

### 扩容策略

- **初始容量**: 16 字符
- **增长因子**: 2x
- **最小增长**: 确保满足最小需求

### 最佳实践

1. **预估容量**: 如果知道大致大小，使用带容量的构造函数
2. **批量追加**: 减少方法调用开销
3. **避免频繁toString**: 只在最后转换一次
4. **及时释放**: 大StringBuilder用完及时置null

## 与String的对比

| 特性 | String | StringBuilder |
|------|--------|---------------|
| 可变性 | 不可变 | 可变 |
| 拼接性能 | O(n²) | O(n) |
| 线程安全 | 安全 | 不安全 |
| 内存占用 | 较多（每次创建新对象） | 较少（复用缓冲区） |
| 适用场景 | 静态字符串 | 动态构建 |

## 注意事项

1. **非线程安全**: 多线程环境需要外部同步
2. **内存泄漏**: 长时间持有大量StringBuilder会占用内存
3. **容量规划**: 频繁扩容会影响性能
4. **FFI使用**: c_str() 返回的指针在StringBuilder修改后失效
5. **null处理**: append(null) 会追加 "null" 字符串
