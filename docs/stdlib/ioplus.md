# IOPlus - 输入输出增强工具类

## 版本信息

- **版本**: 0.5.0.0
- **文件**: `caylibs/IOPlus.cay`
- **依赖**: 无（使用 Cavvy 内置 IO 函数）

## 概述

IOPlus 提供了类似 Python 的便捷打印功能，支持可变参数、多种数据类型和格式化输出。所有方法均为静态，无需实例化。

**核心特性**:
- 类似 Python print 的可变参数打印
- 支持多种数据类型（字符串、整数、浮点数）
- 自定义分隔符
- 输入读取辅助函数
- 格式化输出工具

**时间复杂度**: O(n) n=输出总长度
**空间复杂度**: O(1)

## 静态方法

### 字符串打印

#### prints(String...) - 打印多个字符串

接受多个字符串参数，用空格分隔，最后输出换行。

```cay
// 基本用法
IOPlus.prints("Hello", "World");
// 输出: Hello World\n
// 多个参数
IOPlus.prints("The", "quick", "brown", "fox");
// 输出: The quick brown fox\n
// 空参数
IOPlus.prints();
// 输出: \n
```

#### printsNoLn(String...) - 打印不换行

类似 prints 但不输出换行。

```cay
IOPlus.printsNoLn("Loading");
IOPlus.printsNoLn("...");
IOPlus.println();
// 输出: Loading...\n
// 用于构建同一行输出
IOPlus.printsNoLn("Progress:");
IOPlus.printsNoLn(" 50%");
// 输出: Progress: 50%
```

#### printsSep(String, String...) - 指定分隔符

使用指定分隔符打印多个字符串。

```cay
// 使用逗号分隔
IOPlus.printsSep(", ", "apple", "banana", "cherry");
// 输出: apple, banana, cherry\n
// 使用自定义分隔符
IOPlus.printsSep(" | ", "ERROR", "File not found", "code: 404");
// 输出: ERROR | File not found | code: 404\n
// 使用无分隔符
IOPlus.printsSep("", "H", "e", "l", "l", "o");
// 输出: Hello\n
// 使用换行分隔
IOPlus.printsSep("\n", "Line1", "Line2", "Line3");
// 输出:
// Line1
// Line2
// Line3
```

#### printsSepNoLn(String, String...) - 指定分隔符不换行

使用指定分隔符打印，不换行。

```cay
IOPlus.printsSepNoLn(" -> ", "Start", "Middle");
IOPlus.printsSep(" -> ", "End");
// 输出: Start -> Middle -> End\n
// 用于构建表格
IOPlus.printsSepNoLn(" | ", "Name", "Age", "City");
IOPlus.printsSep(" | ", "----", "---", "----");
// 输出:
// Name | Age | City
// ---- | --- | ----
```

### 整数打印

#### printi(int...) - 打印多个整数

打印多个整数，用空格分隔，最后换行。

```cay
// 基本用法
IOPlus.printi(1, 2, 3);
// 输出: 1 2 3\n
// 混合正负数
IOPlus.printi(-10, 0, 10, 100);
// 输出: -10 0 10 100\n
// 空参数
IOPlus.printi();
// 输出: \n
```

#### printiNoLn(int...) - 打印整数不换行

```cay
IOPlus.printiNoLn(1, 2);
IOPlus.printi(3, 4);
// 输出: 1 2 3 4\n
// 用于构建数字序列
IOPlus.printiNoLn(1);
IOPlus.printsNoLn(",");
IOPlus.printiNoLn(2);
IOPlus.printsNoLn(",");
IOPlus.printi(3);
// 输出: 1,2,3\n```

### 浮点数打印

#### printfl(float...) - 打印多个浮点数

打印多个 float 类型数值，用空格分隔，最后换行。

```cay
// 基本用法
IOPlus.printfl(3.14, 2.71, 1.41);
// 输出: 3.14 2.71 1.41\n
// 科学计数法
IOPlus.printfl(1.5e10, 2.5e-5);
// 输出: 1.5e10 2.5e-5\n```

#### printdb(double...) - 打印多个双精度浮点数

打印多个 double 类型数值，用空格分隔，最后换行。

```cay
// 高精度数值
IOPlus.printdb(3.141592653589793, 2.718281828459045);
// 输出: 3.141592653589793 2.718281828459045\n
// 混合整数和小数
IOPlus.printdb(1.0, 2.5, 3.14159);
// 输出: 1.0 2.5 3.14159\n```

### 混合类型打印

#### printsi(String, int) - 字符串和整数

```cay
IOPlus.printsi("Count:", 42);
// 输出: Count: 42\n
IOPlus.printsi("Age:", 25);
// 输出: Age: 25\n
IOPlus.printsi("Score:", -100);
// 输出: Score: -100\n```

#### printsf(String, float) - 字符串和浮点数

```cay
IOPlus.printsf("Pi:", 3.14);
// 输出: Pi: 3.14\n
IOPlus.printsf("Temperature:", 36.5);
// 输出: Temperature: 36.5\n```

#### printsis(String, int, String) - 字符串、整数、字符串

```cay
IOPlus.printsis("User", 123, "logged in");
// 输出: User 123 logged in\n
IOPlus.printsis("Item", 5, "added to cart");
// 输出: Item 5 added to cart\n```

#### printssi(String, String, int) - 两个字符串和一个整数

```cay
IOPlus.printssi("Error", "in line", 42);
// 输出: Error in line 42\n
IOPlus.printssi("Page", "number:", 1);
// 输出: Page number: 1\n```

### 输入方法

#### input(String) - 带提示的输入

显示提示信息并读取用户输入。

```cay
// 基本用法
String name = IOPlus.input("Enter your name: ");
// 输出: Enter your name: （等待输入）

// 数字输入
String ageStr = IOPlus.input("Enter your age: ");
int age = parseInt(ageStr);

// 确认提示
String confirm = IOPlus.input("Continue? (y/n): ");
```

#### input() - 无提示输入

读取用户输入（无提示）。

```cay
println("Enter command:");
String command = IOPlus.input();

// 或配合 printsNoLn
IOPlus.printsNoLn("> ");
String command = IOPlus.input();
// 输出: > （等待输入，光标在同一行）
```

#### inputInt(String) - 读取整数输入

显示提示并读取整数。

```cay
// 基本用法
int age = IOPlus.inputInt("Enter your age: ");
// 输出: Enter your age: （等待输入，返回整数）

// 用于菜单选择
int choice = IOPlus.inputInt("Select option (1-5): ");

// 用于数量输入
int quantity = IOPlus.inputInt("Enter quantity: ");
```

#### inputFloat(String) - 读取浮点数输入

显示提示并读取浮点数。

```cay
// 基本用法
float price = IOPlus.inputFloat("Enter price: ");
// 输出: Enter price: （等待输入，返回浮点数）

// 用于测量值
float temperature = IOPlus.inputFloat("Enter temperature: ");

// 用于百分比
float discount = IOPlus.inputFloat("Enter discount (%): ");
```

### 辅助方法

#### println() - 打印空行

```cay
IOPlus.println("Header");
IOPlus.println();           // 空行
IOPlus.println("Content");
// 输出:
// Header
//
// Content
```

#### repeat(String, int) - 重复打印

重复打印某个字符串多次。

```cay
// 重复字符
IOPlus.repeat("*", 10);
// 输出: **********

// 重复字符串
IOPlus.repeat("Na", 8);
// 输出: NaNaNaNaNaNaNaNa

// 用于进度条
IOPlus.printsNoLn("[");
IOPlus.repeat("=", 50);
IOPlus.printsNoLn("]");
// 输出: [==================================================]
```

#### repeatLn(String, int) - 重复打印并换行

```cay
IOPlus.repeatLn("-", 40);
// 输出: ----------------------------------------\n
// 用于分隔线
IOPlus.prints("Section 1");
IOPlus.repeatLn("=", 40);
IOPlus.prints("Content...");
// 输出:
// Section 1
// ========================================
// Content...
```

#### divider(int) - 水平分割线

打印指定长度的水平分割线（使用 `-`）。

```cay
IOPlus.divider(40);
// 输出: ----------------------------------------\n
// 用于UI分隔
IOPlus.prints("Menu");
IOPlus.divider(20);
IOPlus.prints("1. Option 1");
IOPlus.prints("2. Option 2");
IOPlus.divider(20);
// 输出:
// Menu
// --------------------
// 1. Option 1
// 2. Option 2
// --------------------
```

#### divider(int, String) - 自定义分割线

使用指定字符打印分割线。

```cay
// 使用等号
IOPlus.divider(30, "=");
// 输出: ==============================\n
// 使用星号
IOPlus.divider(20, "*");
// 输出: ********************\n
// 使用波浪线
IOPlus.divider(25, "~");
// 输出: ~~~~~~~~~~~~~~~~~~~~~~~~~\n
// 用于标题装饰
IOPlus.divider(30, "=");
IOPlus.prints("WELCOME");
IOPlus.divider(30, "=");
// 输出:
// ==============================
// WELCOME
// ==============================
```

## 使用示例

### 交互式菜单

```cay
#include <IOPlus.cay>

public class Main {
    public static void main(String[] args) {
        IOPlus.divider(30, "=");
        IOPlus.prints("MAIN MENU");
        IOPlus.divider(30, "=");
        IOPlus.prints("1. View profile");
        IOPlus.prints("2. Edit settings");
        IOPlus.prints("3. Logout");
        IOPlus.divider(30, "=");
        
        int choice = IOPlus.inputInt("Select option: ");
        IOPlus.printsi("You selected:", choice);
    }
}
```

### 数据表格输出

```cay
#include <IOPlus.cay>

public class Main {
    public static void main(String[] args) {
        // 表头
        IOPlus.printsSepNoLn(" | ", "ID", "Name", "Score");
        IOPlus.prints();
        IOPlus.printsSep(" | ", "--", "----", "-----");
        
        // 数据行
        IOPlus.printsSep(" | ", "1", "Alice", "95");
        IOPlus.printsSep(" | ", "2", "Bob", "87");
        IOPlus.printsSep(" | ", "3", "Charlie", "92");
    }
}
```

### 进度显示

```cay
#include <IOPlus.cay>

public class Main {
    public static void main(String[] args) {
        IOPlus.printsNoLn("[");
        
        int i = 0;
        while (i <= 10) {
            IOPlus.printsNoLn("=");
            i = i + 1;
        }
        
        IOPlus.printsNoLn("] 100%");
        IOPlus.println();
        // 输出: [==========] 100%
    }
}
```

### 日志输出

```cay
#include <IOPlus.cay>

public class Logger {
    public static void info(String message) {
        IOPlus.printsSep(" ", "[INFO]", message);
    }
    
    public static void error(String message, int code) {
        IOPlus.printssi("[ERROR]", message, code);
    }
    
    public static void debug(String context, String message) {
        IOPlus.printsSep(": ", "[DEBUG]" + context, message);
    }
}

public class Main {
    public static void main(String[] args) {
        Logger.info("Application started");
        Logger.error("Connection failed", 500);
        Logger.debug("Database", "Query executed in 12ms");
    }
}
```

### 用户注册表单

```cay
#include <IOPlus.cay>

public class Main {
    public static void main(String[] args) {
        IOPlus.divider(40, "=");
        IOPlus.prints("USER REGISTRATION");
        IOPlus.divider(40, "=");
        
        String username = IOPlus.input("Username: ");
        String email = IOPlus.input("Email: ");
        int age = IOPlus.inputInt("Age: ");
        
        IOPlus.divider(40, "-");
        IOPlus.prints("Registration Summary:");
        IOPlus.printsis("Username:", username, "");
        IOPlus.printsis("Email:", email, "");
        IOPlus.printsi("Age:", age);
        IOPlus.divider(40, "-");
    }
}
```

### 数学计算展示

```cay
#include <IOPlus.cay>

public class Main {
    public static void main(String[] args) {
        IOPlus.prints("Mathematical Constants:");
        IOPlus.divider(30);
        
        IOPlus.printsf("Pi", 3.14159);
        IOPlus.printsf("Euler's number", 2.71828);
        IOPlus.printsf("Golden ratio", 1.61803);
        
        IOPlus.divider(30);
        IOPlus.prints("Sample Calculations:");
        
        int a = 10;
        int b = 3;
        IOPlus.printsi("a =", a);
        IOPlus.printsi("b =", b);
        IOPlus.printsi("a + b =", a + b);
        IOPlus.printsi("a - b =", a - b);
        IOPlus.printsi("a * b =", a * b);
        IOPlus.printsi("a / b =", a / b);
        IOPlus.printsi("a % b =", a % b);
    }
}
```

## 性能优化

### 时间复杂度

| 方法 | 时间复杂度 | 说明 |
|------|-----------|------|
| prints | O(n) | n=总输出长度 |
| printi | O(n) | n=数字位数 |
| printfl | O(n) | n=数字位数 |
| repeat | O(n*k) | n=重复次数, k=字符串长度 |
| divider | O(n) | n=长度 |

### 最佳实践

1. **批量输出**: 减少系统调用次数
2. **使用NoLn方法**: 构建复杂行输出
3. **合理分隔**: 使用divider增强可读性
4. **输入验证**: inputInt/inputFloat需要额外验证

## 注意事项

1. **类型限制**: 每种方法有固定参数类型，不能混用
2. **换行控制**: 注意带Ln和不带Ln方法的区别
3. **输入阻塞**: input方法会阻塞等待用户输入
4. **格式化**: 当前版本不支持printf风格的格式化
5. **性能**: 大量输出建议使用StringBuilder构建后一次性输出
