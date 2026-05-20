# Math - 数学计算标准库

## 版本信息

- **版本**: 0.5.3.0
- **文件**: `caylibs/Math.cay`
- **依赖**: `std/ffi.cay`

## 概述

Math 标准库提供全面的数学计算功能，包括：

- **Math 类**: 静态数学工具，提供三角函数、对数、指数等
- **Random 类**: 随机数生成器
- **Vector2/Vector3 类**: 2D/3D 向量运算
- **数学常量**: PI、E 等常用常量

**核心特性**:
- 完整的三角函数支持（sin, cos, tan, asin, acos, atan, atan2）
- 双曲函数（sinh, cosh, tanh）
- 指数和对数函数（exp, log, log10, log2）
- 幂函数（pow, sqrt, cbrt, sqr）
- 取整函数（ceil, floor, round, trunc）
- 最值函数（max, min, clamp）
- 插值函数（lerp, smoothStep）
- GCD 和 LCM 计算
- 随机数生成（均匀分布、正态分布）
- 2D/3D 向量运算

**时间复杂度**: 大多数函数 O(1)，GCD/LCM O(log n)
**空间复杂度**: O(1)

## 数学常量

| 常量 | 值 | 说明 |
|------|-----|------|
| `MATH_PI` | 3.141592653589793 | 圆周率 |
| `MATH_E` | 2.718281828459045 | 自然对数底 |
| `MATH_LN2` | 0.693147180559945 | ln(2) |
| `MATH_LN10` | 2.302585092994046 | ln(10) |
| `MATH_LOG2E` | 1.442695040888963 | log2(e) |
| `MATH_LOG10E` | 0.434294481903252 | log10(e) |
| `MATH_SQRT2` | 1.414213562373095 | √2 |
| `MATH_SQRT1_2` | 0.707106781186548 | 1/√2 |
| `MATH_DEG_TO_RAD` | 0.017453292519943 | 度转弧度系数 |
| `MATH_RAD_TO_DEG` | 57.29577951308232 | 弧度转度系数 |
| `MATH_EPSILON` | 1e-10 | 浮点精度容差 |

## Math 类 - 静态数学工具

### 三角函数

#### sin(double x) - 正弦函数

```cay
// 计算正弦值（参数为弧度）
double result = Math.sin(Math.PI / 2);  // 1.0
double result2 = Math.sin(0);            // 0.0

// 度转弧度后计算
double degrees = 30.0;
double radians = Math.toRadians(degrees);
double sinValue = Math.sin(radians);     // 0.5
```

**时间复杂度**: O(1)

#### cos(double x) - 余弦函数

```cay
double result = Math.cos(0);             // 1.0
double result2 = Math.cos(Math.PI);      // -1.0
```

#### tan(double x) - 正切函数

```cay
double result = Math.tan(Math.PI / 4);   // 1.0
double result2 = Math.tan(0);            // 0.0
```

#### asin(double x) - 反正弦函数

```cay
// 输入值必须在 [-1, 1] 范围内
double result = Math.asin(1.0);          // 1.570796... (π/2)
double result2 = Math.asin(0.5);         // 0.523599... (π/6)
```

#### acos(double x) - 反余弦函数

```cay
double result = Math.acos(1.0);          // 0.0
double result2 = Math.acos(0.0);         // 1.570796... (π/2)
```

#### atan(double x) - 反正切函数

```cay
double result = Math.atan(1.0);          // 0.785398... (π/4)
double result2 = Math.atan(0.0);         // 0.0
```

#### atan2(double y, double x) - 双参数反正切

```cay
// 返回 [-π, π] 范围内的角度，正确处理所有象限
double angle1 = Math.atan2(1.0, 1.0);    // π/4 (45度)
double angle2 = Math.atan2(1.0, -1.0);   // 3π/4 (135度)
double angle3 = Math.atan2(-1.0, -1.0);  // -3π/4 (-135度)
double angle4 = Math.atan2(-1.0, 1.0);   // -π/4 (-45度)

// 从坐标计算角度
double x = 3.0;
double y = 4.0;
double angle = Math.atan2(y, x);         // 0.927295... (约53.13度)
```

### 双曲函数

#### sinh(double x) - 双曲正弦

```cay
double result = Math.sinh(1.0);          // 1.175201...
```

#### cosh(double x) - 双曲余弦

```cay
double result = Math.cosh(0.0);          // 1.0
```

#### tanh(double x) - 双曲正切

```cay
double result = Math.tanh(1.0);          // 0.761594...
```

### 指数和对数

#### exp(double x) - 指数函数 e^x

```cay
double result = Math.exp(1.0);           // 2.718281... (e)
double result2 = Math.exp(0.0);          // 1.0
double result3 = Math.exp(2.0);          // 7.389056... (e²)
```

#### log(double x) - 自然对数 ln(x)

```cay
double result = Math.log(Math.E);        // 1.0
double result2 = Math.log(1.0);          // 0.0
double result3 = Math.log(10.0);         // 2.302585...
```

**参数**: x > 0

#### log10(double x) - 常用对数 log10(x)

```cay
double result = Math.log10(100.0);       // 2.0
double result2 = Math.log10(1000.0);     // 3.0
double result3 = Math.log10(10.0);       // 1.0
```

#### log2(double x) - 以2为底的对数

```cay
double result = Math.log2(8.0);          // 3.0
double result2 = Math.log2(256.0);       // 8.0
```

#### logBase(double x, double base) - 任意底数对数

```cay
// log₃(27) = 3
double result = Math.logBase(27.0, 3.0);

// log₅(125) = 3
double result2 = Math.logBase(125.0, 5.0);
```

### 幂函数

#### pow(double x, double y) - 幂函数 x^y

```cay
double result = Math.pow(2.0, 10.0);     // 1024.0
double result2 = Math.pow(10.0, 3.0);    // 1000.0
double result3 = Math.pow(2.0, -1.0);    // 0.5
double result4 = Math.pow(4.0, 0.5);     // 2.0 (平方根)
```

#### sqrt(double x) - 平方根

```cay
double result = Math.sqrt(16.0);         // 4.0
double result2 = Math.sqrt(2.0);         // 1.414213...
```

**参数**: x >= 0

#### cbrt(double x) - 立方根

```cay
double result = Math.cbrt(27.0);         // 3.0
double result2 = Math.cbrt(-8.0);        // -2.0
double result3 = Math.cbrt(0.0);         // 0.0
```

#### sqr(double x) - 平方

```cay
double result = Math.sqr(5.0);           // 25.0
double result2 = Math.sqr(-3.0);         // 9.0
```

### 取整函数

#### ceil(double x) - 向上取整

```cay
double result = Math.ceil(3.2);          // 4.0
double result2 = Math.ceil(3.9);         // 4.0
double result3 = Math.ceil(-3.2);        // -3.0
```

#### floor(double x) - 向下取整

```cay
double result = Math.floor(3.9);         // 3.0
double result2 = Math.floor(3.1);        // 3.0
double result3 = Math.floor(-3.2);       // -4.0
```

#### round(double x) - 四舍五入

```cay
double result = Math.round(3.4);         // 3.0
double result2 = Math.round(3.5);        // 4.0
double result3 = Math.round(3.6);        // 4.0
double result4 = Math.round(-3.5);       // -4.0
```

#### trunc(double x) - 向零取整

```cay
double result = Math.trunc(3.9);         // 3.0
double result2 = Math.trunc(-3.9);       // -3.0
double result3 = Math.trunc(3.1);        // 3.0
```

#### frac(double x) - 取小数部分

```cay
double result = Math.frac(3.14159);      // 0.14159
double result2 = Math.frac(-3.14159);    // -0.14159
```

### 绝对值和符号

#### abs(double x) - 绝对值（double）

```cay
double result = Math.abs(-3.14);         // 3.14
double result2 = Math.abs(3.14);         // 3.14
```

#### abs(int x) - 绝对值（int）

```cay
int result = Math.abs(-42);              // 42
int result2 = Math.abs(42);              // 42
```

#### abs(long x) - 绝对值（long）

```cay
long result = Math.abs(-9999999999L);    // 9999999999
```

#### sign(double x) - 符号函数

```cay
int result = Math.sign(-5.0);            // -1
int result2 = Math.sign(5.0);            // 1
int result3 = Math.sign(0.0);            // 0
```

#### fmod(double x, double y) - 浮点取模

```cay
double result = Math.fmod(10.5, 3.0);    // 1.5
double result2 = Math.fmod(-10.5, 3.0);  // -1.5
```

### 角度转换

#### toRadians(double degrees) - 度转弧度

```cay
double radians = Math.toRadians(180.0);  // 3.14159... (π)
double radians2 = Math.toRadians(90.0);  // 1.57079... (π/2)
double radians3 = Math.toRadians(45.0);  // 0.78539... (π/4)
```

#### toDegrees(double radians) - 弧度转度

```cay
double degrees = Math.toDegrees(Math.PI);        // 180.0
double degrees2 = Math.toDegrees(Math.PI / 2);   // 90.0
double degrees3 = Math.toDegrees(Math.PI / 4);   // 45.0
```

### 最值函数

#### max/min - 最大值/最小值

```cay
// int 版本
int maxVal = Math.max(10, 20);           // 20
int minVal = Math.min(10, 20);           // 10

// double 版本
double maxVal2 = Math.max(3.14, 2.71);   // 3.14
double minVal2 = Math.min(3.14, 2.71);   // 2.71

// long 版本
long maxVal3 = Math.max(1000000L, 2000000L);  // 2000000
```

#### clamp - 钳制函数

```cay
// int 版本
int clamped = Math.clamp(150, 0, 100);   // 100 (超过上限)
int clamped2 = Math.clamp(-50, 0, 100);  // 0 (低于下限)
int clamped3 = Math.clamp(50, 0, 100);   // 50 (在范围内)

// double 版本
double clamped4 = Math.clamp(3.14, 0.0, 1.0);  // 1.0
```

### 比较函数

#### approxEqual - 近似相等

```cay
// 使用默认容差 (1e-10)
bool equal = Math.approxEqual(0.1 + 0.2, 0.3);  // true

// 使用自定义容差
bool equal2 = Math.approxEqual(1.0, 1.001, 0.01);  // true
bool equal3 = Math.approxEqual(1.0, 1.001, 0.0001); // false
```

### 插值函数

#### lerp - 线性插值

```cay
// double 版本
double result = Math.lerp(0.0, 100.0, 0.5);   // 50.0
double result2 = Math.lerp(10.0, 20.0, 0.3);  // 13.0

// int 版本
int result3 = Math.lerp(0, 100, 0.5);         // 50
int result4 = Math.lerp(0, 100, 0.3);         // 30
```

#### smoothStep - 平滑插值

```cay
// 使用 Hermite 插值，在边界处平滑
double result = Math.smoothStep(0.0, 1.0, 0.5);  // 0.5
double result2 = Math.smoothStep(0.0, 1.0, 0.0); // 0.0
double result3 = Math.smoothStep(0.0, 1.0, 1.0); // 1.0
```

### GCD 和 LCM

#### gcd - 最大公约数

```cay
int result = Math.gcd(48, 18);           // 6
int result2 = Math.gcd(100, 35);         // 5
int result3 = Math.gcd(17, 13);          // 1 (互质)
```

**时间复杂度**: O(log(min(a,b)))

#### lcm - 最小公倍数

```cay
int result = Math.lcm(4, 6);             // 12
int result2 = Math.lcm(21, 6);           // 42
```

## Random 类 - 随机数生成器

### init() - 初始化随机数生成器

```cay
// 使用当前时间作为种子初始化
Random.init();

// 初始化是自动的，首次调用 nextInt() 等函数时会自动初始化
```

### setSeed(int seed) - 设置随机种子

```cay
// 设置固定种子以获得可重复的随机序列
Random.setSeed(12345);
int a = Random.nextInt(100);  // 总是产生相同结果

Random.setSeed(12345);
int b = Random.nextInt(100);  // 与 a 相同
```

### nextInt() - 生成随机整数

```cay
// 生成 [0, RAND_MAX] 范围内的随机整数
int value = Random.nextInt();
```

### nextInt(int bound) - 生成有界随机整数

```cay
// 生成 [0, bound) 范围内的随机整数
int dice = Random.nextInt(6) + 1;        // 1-6 (骰子)
int coin = Random.nextInt(2);            // 0 或 1 (硬币)
int percent = Random.nextInt(101);       // 0-100 (百分比)
```

### nextInt(int min, int max) - 生成范围内随机整数

```cay
// 生成 [min, max] 范围内的随机整数
int age = Random.nextInt(18, 65);        // 18-65 岁
int temperature = Random.nextInt(-10, 40); // -10 到 40 度
```

### nextDouble() - 生成随机浮点数

```cay
// 生成 [0.0, 1.0) 范围内的随机浮点数
double value = Random.nextDouble();
```

### nextDouble(double min, double max) - 生成范围内随机浮点数

```cay
// 生成 [min, max) 范围内的随机浮点数
double price = Random.nextDouble(10.0, 100.0);
double angle = Random.nextDouble(0.0, 2.0 * Math.PI);
```

### nextBool() - 生成随机布尔值

```cay
bool flag = Random.nextBool();           // true 或 false

// 用于随机决策
if (Random.nextBool()) {
    println("Heads");
} else {
    println("Tails");
}
```

### nextGaussian(double mean, double stdDev) - 正态分布随机数

```cay
// 生成符合正态分布的随机数
// 均值为 0，标准差为 1 的标准正态分布
double stdNormal = Random.nextGaussian(0.0, 1.0);

// 均值为 100，标准差为 15 的 IQ 分数分布
double iq = Random.nextGaussian(100.0, 15.0);

// 均值为 50，标准差为 10 的考试成绩分布
double score = Random.nextGaussian(50.0, 10.0);
```

## Vector2 类 - 2D 向量

### 构造函数

```cay
// 默认构造函数（零向量）
Vector2 v1 = new Vector2();              // (0, 0)

// 带参数构造函数
Vector2 v2 = new Vector2(3.0, 4.0);      // (3, 4)
```

### 基本运算

```cay
Vector2 a = new Vector2(1.0, 2.0);
Vector2 b = new Vector2(3.0, 4.0);

// 加法
Vector2 sum = a.add(b);                  // (4, 6)

// 减法
Vector2 diff = a.sub(b);                 // (-2, -2)

// 数乘
Vector2 scaled = a.mul(2.0);             // (2, 4)

// 除法
Vector2 divided = a.div(2.0);            // (0.5, 1.0)
```

### 点积和叉积

```cay
Vector2 a = new Vector2(1.0, 2.0);
Vector2 b = new Vector2(3.0, 4.0);

// 点积
double dot = a.dot(b);                   // 1*3 + 2*4 = 11

// 叉积（标量）
double cross = a.cross(b);               // 1*4 - 2*3 = -2
```

### 长度和距离

```cay
Vector2 v = new Vector2(3.0, 4.0);

// 向量长度
double len = v.length();                 // 5.0

// 长度平方（避免开方运算，更快）
double lenSq = v.lengthSquared();        // 25.0

// 归一化（单位向量）
Vector2 normalized = v.normalize();      // (0.6, 0.8)

// 向量距离
Vector2 a = new Vector2(1.0, 2.0);
Vector2 b = new Vector2(4.0, 6.0);
double dist = a.distance(b);             // 5.0
double distSq = a.distanceSquared(b);    // 25.0
```

### 插值和旋转

```cay
Vector2 a = new Vector2(0.0, 0.0);
Vector2 b = new Vector2(10.0, 10.0);

// 线性插值
Vector2 mid = a.lerp(b, 0.5);            // (5, 5)
Vector2 quarter = a.lerp(b, 0.25);       // (2.5, 2.5)

// 旋转向量
Vector2 v = new Vector2(1.0, 0.0);
Vector2 rotated = v.rotate(Math.PI / 2); // (0, 1) 逆时针旋转90度
```

### 反射

```cay
// 向量反射（用于碰撞检测）
Vector2 velocity = new Vector2(1.0, -1.0);
Vector2 normal = new Vector2(0.0, 1.0);  // 水平面法向量
Vector2 reflected = velocity.reflect(normal);
// 结果: (1.0, 1.0)
```

### 静态工厂方法

```cay
// 零向量
Vector2 zero = Vector2.zero();           // (0, 0)

// 右向量
Vector2 right = Vector2.right();         // (1, 0)

// 上向量
Vector2 up = Vector2.up();               // (0, 1)
```

## Vector3 类 - 3D 向量

### 构造函数

```cay
// 默认构造函数（零向量）
Vector3 v1 = new Vector3();              // (0, 0, 0)

// 带参数构造函数
Vector3 v2 = new Vector3(1.0, 2.0, 3.0); // (1, 2, 3)
```

### 基本运算

```cay
Vector3 a = new Vector3(1.0, 2.0, 3.0);
Vector3 b = new Vector3(4.0, 5.0, 6.0);

// 加法
Vector3 sum = a.add(b);                  // (5, 7, 9)

// 减法
Vector3 diff = a.sub(b);                 // (-3, -3, -3)

// 数乘
Vector3 scaled = a.mul(2.0);             // (2, 4, 6)

// 除法
Vector3 divided = a.div(2.0);            // (0.5, 1.0, 1.5)
```

### 点积和叉积

```cay
Vector3 a = new Vector3(1.0, 2.0, 3.0);
Vector3 b = new Vector3(4.0, 5.0, 6.0);

// 点积
double dot = a.dot(b);                   // 1*4 + 2*5 + 3*6 = 32

// 叉积
Vector3 cross = a.cross(b);              // (-3, 6, -3)
// cross.x = 2*6 - 3*5 = -3
// cross.y = 3*4 - 1*6 = 6
// cross.z = 1*5 - 2*4 = -3
```

### 长度和归一化

```cay
Vector3 v = new Vector3(1.0, 2.0, 2.0);

// 向量长度
double len = v.length();                 // 3.0

// 长度平方
double lenSq = v.lengthSquared();        // 9.0

// 归一化
Vector3 normalized = v.normalize();      // (0.333..., 0.666..., 0.666...)
```

### 静态工厂方法

```cay
// 零向量
Vector3 zero = Vector3.zero();           // (0, 0, 0)

// 右向量
Vector3 right = Vector3.right();         // (1, 0, 0)

// 上向量
Vector3 up = Vector3.up();               // (0, 1, 0)

// 前向量
Vector3 forward = Vector3.forward();     // (0, 0, 1)
```

## 使用示例

### 计算两点间距离

```cay
#include <Math.cay>

public class Main {
    public static void main(String[] args) {
        // 2D 距离
        Vector2 p1 = new Vector2(0.0, 0.0);
        Vector2 p2 = new Vector2(3.0, 4.0);
        double distance2D = p1.distance(p2);
        println("2D Distance: " + (String)distance2D);  // 5.0

        // 3D 距离
        Vector3 p3 = new Vector3(1.0, 2.0, 2.0);
        Vector3 p4 = new Vector3(4.0, 6.0, 2.0);
        double distance3D = p3.distance(p4);
        println("3D Distance: " + (String)distance3D);  // 5.0
    }
}
```

### 随机数生成示例

```cay
#include <Math.cay>

public class Main {
    public static void main(String[] args) {
        // 初始化随机数生成器
        Random.init();

        // 生成随机整数
        println("Random int [0, 100): " + (String)Random.nextInt(100));

        // 生成骰子点数
        int dice = Random.nextInt(1, 6);
        println("Dice roll: " + (String)dice);

        // 生成随机浮点数
        double price = Random.nextDouble(10.0, 100.0);
        println("Random price: " + (String)price);

        // 生成正态分布随机数
        double measurement = Random.nextGaussian(50.0, 5.0);
        println("Measurement: " + (String)measurement);
    }
}
```

### 三角函数应用

```cay
#include <Math.cay>

public class Main {
    public static void main(String[] args) {
        // 计算圆上点的坐标
        double radius = 10.0;
        double angle = Math.toRadians(45.0);  // 45度转弧度

        double x = radius * Math.cos(angle);
        double y = radius * Math.sin(angle);

        println("Circle point at 45 degrees:");
        println("x: " + (String)x);  // 7.071...
        println("y: " + (String)y);  // 7.071...

        // 计算两点间角度
        Vector2 a = new Vector2(0.0, 0.0);
        Vector2 b = new Vector2(1.0, 1.0);
        double angleRad = Math.atan2(b.y - a.y, b.x - a.x);
        double angleDeg = Math.toDegrees(angleRad);
        println("Angle: " + (String)angleDeg + " degrees");  // 45.0
    }
}
```

### 向量物理模拟

```cay
#include <Math.cay>

public class Ball {
    private Vector2 position;
    private Vector2 velocity;
    private double radius;

    public Ball(double x, double y, double vx, double vy, double r) {
        this.position = new Vector2(x, y);
        this.velocity = new Vector2(vx, vy);
        this.radius = r;
    }

    public void update(double dt) {
        // position += velocity * dt
        this.position = this.position.add(this.velocity.mul(dt));
    }

    public void bounce(Vector2 normal) {
        // velocity = reflect(velocity, normal)
        this.velocity = this.velocity.reflect(normal);
    }

    public void printPosition() {
        println("Ball at: " + this.position.toString());
    }
}

public class Main {
    public static void main(String[] args) {
        Ball ball = new Ball(0.0, 0.0, 5.0, 3.0, 1.0);

        // 模拟几帧
        int frame = 0;
        while (frame < 5) {
            ball.update(0.016);  // 16ms 一帧
            ball.printPosition();
            frame = frame + 1;
        }

        // 碰到墙壁反弹
        Vector2 wallNormal = new Vector2(-1.0, 0.0);
        ball.bounce(wallNormal);
        println("After bounce:");
        ball.printPosition();
    }
}
```

### 数值计算

```cay
#include <Math.cay>

public class Main {
    public static void main(String[] args) {
        // 计算 GCD 和 LCM
        int a = 48;
        int b = 18;
        println("GCD(" + (String)a + ", " + (String)b + ") = " + (String)Math.gcd(a, b));
        println("LCM(" + (String)a + ", " + (String)b + ") = " + (String)Math.lcm(a, b));

        // 钳制数值
        int health = 150;
        int clampedHealth = Math.clamp(health, 0, 100);
        println("Clamped health: " + (String)clampedHealth);  // 100

        // 线性插值
        double start = 0.0;
        double end = 100.0;
        double progress = 0.75;
        double current = Math.lerp(start, end, progress);
        println("Lerped value: " + (String)current);  // 75.0

        // 浮点数比较
        double x = 0.1 + 0.2;
        double y = 0.3;
        if (Math.approxEqual(x, y)) {
            println("0.1 + 0.2 ≈ 0.3 (within epsilon)");
        }
    }
}
```

## 性能优化

### 时间复杂度

| 操作 | 时间复杂度 | 说明 |
|------|-----------|------|
| 三角函数 | O(1) | 硬件加速 |
| 指数/对数 | O(1) | 硬件加速 |
| 幂函数 | O(1) | 硬件加速 |
| 向量运算 | O(1) | 固定操作数 |
| GCD | O(log(min(a,b))) | 欧几里得算法 |
| LCM | O(log(min(a,b))) | 基于 GCD |

### 最佳实践

1. **避免重复计算**: 缓存常用值如 sin/cos 结果
2. **使用 lengthSquared**: 比较距离时避免开方
3. **批量随机数**: 减少 Random 初始化开销
4. **预计算常量**: 将角度转换为弧度后缓存

## 注意事项

1. **三角函数参数**: 使用弧度而非角度
2. **sqrt 参数**: 必须 >= 0，否则结果未定义
3. **log 参数**: 必须 > 0
4. **asin/acos 参数**: 必须在 [-1, 1] 范围内
5. **随机种子**: 设置相同种子产生相同序列
6. **向量除法**: 除数为 0 时返回零向量
