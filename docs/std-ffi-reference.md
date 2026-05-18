# Cavvy FFI 标准库参考文档

## 概述

`std/ffi.cay` 是 Cavvy 的 FFI（Foreign Function Interface）标准库，提供了对 C 标准库函数和常用系统函数的绑定。通过此库，Cavvy 程序可以直接调用 C 语言函数，实现与底层系统的交互。

## 使用方法

```cavy
// 导入 FFI 标准库
#include <std/ffi.cay>

public int main() {
    // 使用 C 标准库函数
    printf("Hello, World!\n");
    
    // 使用内存分配
    void_ptr ptr = malloc(1024);
    if (ptr != null) {
        free(ptr);
    }
    
    return 0;
}
```

## 类型别名

### 基本 C 类型

| Cavvy 类型别名 | C 类型 | 说明 |
|---------------|--------|------|
| `c_void` | `void` | 无类型 |
| `c_bool` | `bool` | 布尔类型 |
| `c_char` | `char` | 有符号字符 |
| `c_uchar` | `unsigned char` | 无符号字符 |
| `c_short` | `short` | 有符号短整型 |
| `c_ushort` | `unsigned short` | 无符号短整型 |
| `c_int` | `int` | 有符号整型 |
| `c_uint` | `unsigned int` | 无符号整型 |
| `c_long` | `long` | 有符号长整型 |
| `c_ulong` | `unsigned long` | 无符号长整型 |
| `c_float` | `float` | 单精度浮点 |
| `c_double` | `double` | 双精度浮点 |
| `c_string` | `char*` | C 字符串 |

### 标准整数类型

| 类型别名 | 说明 | 典型大小 |
|---------|------|---------|
| `int8_t` | 8位有符号整数 | 1字节 |
| `uint8_t` | 8位无符号整数 | 1字节 |
| `int16_t` | 16位有符号整数 | 2字节 |
| `uint16_t` | 16位无符号整数 | 2字节 |
| `int32_t` | 32位有符号整数 | 4字节 |
| `uint32_t` | 32位无符号整数 | 4字节 |
| `int64_t` | 64位有符号整数 | 8字节 |
| `uint64_t` | 64位无符号整数 | 8字节 |
| `size_t` | 大小类型 | 平台相关 |
| `ssize_t` | 有符号大小类型 | 平台相关 |
| `uintptr_t` | 无符号指针整数 | 平台相关 |
| `intptr_t` | 有符号指针整数 | 平台相关 |

### 指针类型

| 类型别名 | C 类型 | 说明 |
|---------|--------|------|
| `void_ptr` | `void*` | 通用指针 |
| `const_void_ptr` | `const void*` | 常量通用指针 |
| `char_ptr` | `char*` | 字符指针 |
| `const_char_ptr` | `const char*` | 常量字符指针 |

### 函数指针类型

```cavy
// 比较函数指针类型
alias CompareFn = fn(ptr, ptr) -> c_int;

// 排序函数指针类型
alias SortFn = fn(ptr, size_t, size_t, CompareFn) -> void;
```

## 标准 C 库函数

### stdio.h - 标准输入输出

#### 格式化输出

```cavy
extern {
    c_int printf(c_string fmt, ...);           // 格式化输出到 stdout
    c_int fprintf(ptr stream, c_string fmt, ...);  // 格式化输出到流
    c_int sprintf(ptr str, c_string fmt, ...);     // 格式化输出到字符串
    c_int snprintf(ptr str, size_t size, c_string fmt, ...);  // 安全格式化输出
}
```

**示例：**
```cavy
printf("Hello %s, number: %d\n", "World", 42);
```

#### 格式化输入

```cavy
extern {
    c_int scanf(c_string fmt, ...);            // 从 stdin 读取
    c_int fscanf(ptr stream, c_string fmt, ...);   // 从流读取
    c_int sscanf(c_string str, c_string fmt, ...); // 从字符串读取
}
```

#### 字符 I/O

```cavy
extern {
    c_int getchar();                           // 读取一个字符
    c_int putchar(c_int c);                    // 输出一个字符
    c_int puts(c_string str);                  // 输出字符串并换行
    c_string fgets(ptr str, c_int n, ptr stream);  // 读取一行
    c_int fputs(c_string str, ptr stream);     // 输出字符串到流
}
```

#### 文件操作

```cavy
extern {
    ptr fopen(c_string filename, c_string mode);   // 打开文件
    c_int fclose(ptr stream);                  // 关闭文件
    c_int fflush(ptr stream);                  // 刷新缓冲区
    
    size_t fread(ptr ptr, size_t size, size_t nmemb, ptr stream);   // 读取
    size_t fwrite(ptr ptr, size_t size, size_t nmemb, ptr stream);  // 写入
    
    c_int fseek(ptr stream, long offset, c_int whence);  // 定位
    long ftell(ptr stream);                    // 获取当前位置
    void rewind(ptr stream);                   // 重置到开头
}
```

**文件打开模式：**
- `"r"` - 只读
- `"w"` - 只写（创建或截断）
- `"a"` - 追加
- `"r+"` - 读写
- `"w+"` - 读写（创建或截断）
- `"a+"` - 读写追加

**定位常量：**
- `SEEK_SET` (0) - 文件开头
- `SEEK_CUR` (1) - 当前位置
- `SEEK_END` (2) - 文件末尾

### stdlib.h - 标准库

#### 内存分配

```cavy
extern {
    void_ptr malloc(size_t size);              // 分配内存
    void_ptr calloc(size_t nmemb, size_t size);    // 分配并清零
    void_ptr realloc(void_ptr ptr, size_t size);   // 重新分配
    void free(void_ptr ptr);                   // 释放内存
}
```

**示例：**
```cavy
// 分配 1024 字节
void_ptr buffer = malloc(1024);
if (buffer != null) {
    // 使用内存...
    free(buffer);
}

// 分配 100 个 int 并初始化为 0
void_ptr arr = calloc(100, sizeof(int));
if (arr != null) {
    free(arr);
}
```

#### 程序控制

```cavy
extern {
    void exit(c_int status);                   // 正常退出
    void abort();                              // 异常终止
    void _Exit(c_int status);                  // 立即退出
}
```

**退出状态码：**
- `EXIT_SUCCESS` (0) - 成功
- `EXIT_FAILURE` (1) - 失败

#### 搜索和排序

```cavy
extern {
    void qsort(void_ptr base, size_t nmemb, size_t size, CompareFn compar);
    void_ptr bsearch(void_ptr key, void_ptr base, size_t nmemb, size_t size, CompareFn compar);
}
```

**示例：**
```cavy
// 比较函数
fn compare_ints(a: ptr, b: ptr) -> c_int {
    c_int x = *(a as ptr<c_int>);
    c_int y = *(b as ptr<c_int>);
    return x - y;
}

// 排序数组
int[] arr = new int[5] {5, 2, 8, 1, 9};
qsort(arr as void_ptr, 5, sizeof(int), compare_ints);
```

#### 随机数

```cavy
extern {
    c_int rand();                              // 生成随机数
    void srand(c_uint seed);                   // 设置随机种子
}
```

**示例：**
```cavy
srand(time(null) as c_uint);  // 使用当前时间作为种子
c_int r = rand() % 100;       // 0-99 的随机数
```

#### 字符串转换

```cavy
extern {
    c_int atoi(c_string nptr);                 // 字符串转 int
    long atol(c_string nptr);                  // 字符串转 long
    c_double atof(c_string nptr);              // 字符串转 double
    long strtol(c_string nptr, ptr endptr, c_int base);  // 指定进制转换
}
```

### string.h - 字符串操作

#### 内存操作

```cavy
extern {
    void_ptr memcpy(void_ptr dest, void_ptr src, size_t n);    // 内存复制
    void_ptr memmove(void_ptr dest, void_ptr src, size_t n);   // 安全内存复制
    void_ptr memset(void_ptr s, c_int c, size_t n);            // 内存填充
    c_int memcmp(void_ptr s1, void_ptr s2, size_t n);          // 内存比较
    void_ptr memchr(void_ptr s, c_int c, size_t n);            // 内存查找
}
```

**示例：**
```cavy
void_ptr src = malloc(100);
void_ptr dest = malloc(100);
memcpy(dest, src, 100);
memset(dest, 0, 100);  // 清零
```

#### 字符串操作

```cavy
extern {
    char_ptr strcpy(char_ptr dest, c_string src);      // 字符串复制
    char_ptr strncpy(char_ptr dest, c_string src, size_t n);  // 安全复制
    char_ptr strcat(char_ptr dest, c_string src);      // 字符串连接
    char_ptr strncat(char_ptr dest, c_string src, size_t n);  // 安全连接
    
    c_int strcmp(c_string s1, c_string s2);            // 字符串比较
    c_int strncmp(c_string s1, c_string s2, size_t n); // 安全比较
    
    size_t strlen(c_string s);                         // 字符串长度
    char_ptr strchr(c_string s, c_int c);              // 查找字符
    char_ptr strstr(c_string haystack, c_string needle);  // 查找子串
    char_ptr strtok(char_ptr str, c_string delim);     // 字符串分割
}
```

### ctype.h - 字符分类

```cavy
extern {
    c_int isalnum(c_int c);    // 字母数字
    c_int isalpha(c_int c);    // 字母
    c_int isdigit(c_int c);    // 数字
    c_int isspace(c_int c);    // 空白字符
    c_int islower(c_int c);    // 小写字母
    c_int isupper(c_int c);    // 大写字母
    
    c_int tolower(c_int c);    // 转小写
    c_int toupper(c_int c);    // 转大写
}
```

### math.h - 数学函数

#### 三角函数

```cavy
extern {
    c_double sin(c_double x);
    c_double cos(c_double x);
    c_double tan(c_double x);
    c_double asin(c_double x);
    c_double acos(c_double x);
    c_double atan(c_double x);
    c_double atan2(c_double y, c_double x);
}
```

#### 指数和对数

```cavy
extern {
    c_double exp(c_double x);
    c_double log(c_double x);      // 自然对数
    c_double log10(c_double x);    // 常用对数
    c_double log2(c_double x);     // 二进制对数
}
```

#### 幂函数

```cavy
extern {
    c_double pow(c_double base, c_double exp);
    c_double sqrt(c_double x);
    c_double cbrt(c_double x);     // 立方根
}
```

#### 取整函数

```cavy
extern {
    c_double ceil(c_double x);     // 向上取整
    c_double floor(c_double x);    // 向下取整
    c_double round(c_double x);    // 四舍五入
    c_double trunc(c_double x);    // 截断小数
}
```

#### 绝对值

```cavy
extern {
    c_double fabs(c_double x);
    c_int abs(c_int n);
    long labs(long n);
}
```

**数学常量：**
- `M_PI` - π (3.14159...)
- `M_E` - e (2.71828...)
- `M_LN2` - ln(2)
- `M_LN10` - ln(10)
- `M_SQRT2` - √2

### time.h - 时间函数

```cavy
extern {
    c_int64_t time(ptr timer);     // 获取当前时间戳
    c_double difftime(c_int64_t end, c_int64_t start);  // 计算时间差
    char_ptr ctime(ptr timer);     // 时间戳转字符串
    ptr localtime(ptr timer);      // 时间戳转本地时间结构
    c_int64_t clock();             // 处理器时间
}
```

**示例：**
```cavy
c_int64_t start = time(null);
// ... 执行某些操作 ...
c_int64_t end = time(null);
c_double elapsed = difftime(end, start);
printf("Elapsed: %.0f seconds\n", elapsed);
```

## Windows API 函数

### 内存管理

```cavy
extern stdcall {
    void_ptr VirtualAlloc(void_ptr lpAddress, size_t dwSize, c_uint flAllocationType, c_uint flProtect);
    c_int VirtualFree(void_ptr lpAddress, size_t dwSize, c_uint dwFreeType);
    c_int VirtualProtect(void_ptr lpAddress, size_t dwSize, c_uint flNewProtect, ptr lpflOldProtect);
}
```

**内存分配类型：**
- `MEM_COMMIT` (0x1000) - 提交内存
- `MEM_RESERVE` (0x2000) - 保留内存
- `MEM_RELEASE` (0x8000) - 释放内存

**内存保护常量：**
- `PAGE_NOACCESS` (0x01) - 不可访问
- `PAGE_READONLY` (0x02) - 只读
- `PAGE_READWRITE` (0x04) - 读写
- `PAGE_EXECUTE` (0x10) - 可执行
- `PAGE_EXECUTE_READWRITE` (0x40) - 可执行读写

### 堆管理

```cavy
extern stdcall {
    ptr GetProcessHeap();
    void_ptr HeapAlloc(ptr hHeap, c_uint dwFlags, size_t dwBytes);
    c_int HeapFree(ptr hHeap, c_uint dwFlags, void_ptr lpMem);
    void_ptr HeapReAlloc(ptr hHeap, c_uint dwFlags, void_ptr lpMem, size_t dwBytes);
    size_t HeapSize(ptr hHeap, c_uint dwFlags, void_ptr lpMem);
}
```

## POSIX 函数 (Unix/Linux)

### 文件描述符操作

```cavy
extern {
    c_int open(c_string pathname, c_int flags, ...);
    c_int close(c_int fd);
    ssize_t read(c_int fd, void_ptr buf, size_t count);
    ssize_t write(c_int fd, void_ptr buf, size_t count);
    off_t lseek(c_int fd, off_t offset, c_int whence);
}
```

**文件打开标志：**
- `O_RDONLY` (0) - 只读
- `O_WRONLY` (1) - 只写
- `O_RDWR` (2) - 读写
- `O_CREAT` (0x40) - 创建文件
- `O_TRUNC` (0x200) - 截断文件
- `O_APPEND` (0x400) - 追加模式

### 目录操作

```cavy
extern {
    c_int mkdir(c_string pathname, c_uint mode);
    c_int rmdir(c_string pathname);
    c_int chdir(c_string path);
    c_string getcwd(char_ptr buf, size_t size);
    ptr opendir(c_string name);
    c_int closedir(ptr dirp);
    ptr readdir(ptr dirp);
}
```

### 进程控制

```cavy
extern {
    c_int fork();                      // 创建子进程
    c_int execv(c_string pathname, ptr argv);  // 执行程序
    c_int wait(ptr status);            // 等待子进程
    c_int getpid();                    // 获取进程 ID
    c_int getppid();                   // 获取父进程 ID
}
```

### 内存映射

```cavy
extern {
    void_ptr mmap(void_ptr addr, size_t length, c_int prot, c_int flags, c_int fd, off_t offset);
    c_int munmap(void_ptr addr, size_t length);
    c_int mprotect(void_ptr addr, size_t len, c_int prot);
}
```

**内存保护：**
- `PROT_NONE` (0) - 不可访问
- `PROT_READ` (1) - 可读
- `PROT_WRITE` (2) - 可写
- `PROT_EXEC` (4) - 可执行

**映射标志：**
- `MAP_SHARED` (1) - 共享映射
- `MAP_PRIVATE` (2) - 私有映射
- `MAP_ANONYMOUS` (0x20) - 匿名映射

## 套接字 API

### 基本套接字操作

```cavy
extern {
    c_int socket(c_int domain, c_int type, c_int protocol);
    c_int bind(c_int sockfd, ptr addr, c_uint addrlen);
    c_int listen(c_int sockfd, c_int backlog);
    c_int accept(c_int sockfd, ptr addr, ptr addrlen);
    c_int connect(c_int sockfd, ptr addr, c_uint addrlen);
    c_int shutdown(c_int sockfd, c_int how);
}
```

**地址族：**
- `AF_UNSPEC` (0) - 未指定
- `AF_UNIX` (1) - Unix 域套接字
- `AF_INET` (2) - IPv4
- `AF_INET6` (10) - IPv6

**套接字类型：**
- `SOCK_STREAM` (1) - 流式套接字 (TCP)
- `SOCK_DGRAM` (2) - 数据报套接字 (UDP)
- `SOCK_RAW` (3) - 原始套接字

### 数据传输

```cavy
extern {
    ssize_t send(c_int sockfd, void_ptr buf, size_t len, c_int flags);
    ssize_t recv(c_int sockfd, void_ptr buf, size_t len, c_int flags);
    ssize_t sendto(c_int sockfd, void_ptr buf, size_t len, c_int flags, ptr dest_addr, c_uint addrlen);
    ssize_t recvfrom(c_int sockfd, void_ptr buf, size_t len, c_int flags, ptr src_addr, ptr addrlen);
}
```

### 地址解析

```cavy
extern {
    ptr gethostbyname(c_string name);
    c_int gethostname(char_ptr name, size_t len);
    c_int getaddrinfo(c_string node, c_string service, ptr hints, ptr res);
    void freeaddrinfo(ptr res);
}
```

### 字节序转换

```cavy
extern {
    c_uint16_t htons(c_uint16_t hostshort);   // 主机字节序转网络字节序 (16位)
    c_uint16_t ntohs(c_uint16_t netshort);    // 网络字节序转主机字节序 (16位)
    c_uint32_t htonl(c_uint32_t hostlong);    // 主机字节序转网络字节序 (32位)
    c_uint32_t ntohl(c_uint32_t netlong);     // 网络字节序转主机字节序 (32位)
}
```

## 线程 API (pthreads)

### 线程管理

```cavy
extern {
    c_int pthread_create(ptr thread, ptr attr, fn(void_ptr) -> void_ptr start_routine, void_ptr arg);
    c_int pthread_join(c_int64_t thread, ptr retval);
    c_int pthread_detach(c_int64_t thread);
    c_int pthread_exit(void_ptr retval);
    c_int pthread_cancel(c_int64_t thread);
}
```

### 互斥锁

```cavy
extern {
    c_int pthread_mutex_init(ptr mutex, ptr attr);
    c_int pthread_mutex_destroy(ptr mutex);
    c_int pthread_mutex_lock(ptr mutex);
    c_int pthread_mutex_trylock(ptr mutex);
    c_int pthread_mutex_unlock(ptr mutex);
}
```

### 条件变量

```cavy
extern {
    c_int pthread_cond_init(ptr cond, ptr attr);
    c_int pthread_cond_destroy(ptr cond);
    c_int pthread_cond_wait(ptr cond, ptr mutex);
    c_int pthread_cond_signal(ptr cond);
    c_int pthread_cond_broadcast(ptr cond);
}
```

### 读写锁

```cavy
extern {
    c_int pthread_rwlock_init(ptr rwlock, ptr attr);
    c_int pthread_rwlock_destroy(ptr rwlock);
    c_int pthread_rwlock_rdlock(ptr rwlock);
    c_int pthread_rwlock_wrlock(ptr rwlock);
    c_int pthread_rwlock_unlock(ptr rwlock);
}
```

## 辅助函数

标准库还提供了一些辅助函数，简化常见操作：

### safe_malloc

```cavy
fn safe_malloc(size: size_t) -> void_ptr
```

安全分配内存，如果 size 为 0 则返回 null。

### safe_free

```cavy
fn safe_free(p: void_ptr) -> void
```

安全释放内存，自动检查 null 指针。

### array_count

```cavy
fn array_count(total_size: size_t, element_size: size_t) -> size_t
```

计算数组元素数量。

### is_valid_ptr

```cavy
fn is_valid_ptr(p: void_ptr) -> bool
```

检查指针是否有效（非 null）。

### is_empty_string

```cavy
fn is_empty_string(s: c_string) -> bool
```

检查字符串是否为空或 null。

## 完整示例

### 示例 1：文件操作

```cavy
#include <std/ffi.cay>

public int main() {
    // 打开文件
    ptr file = fopen("test.txt", "w");
    if (file == null) {
        printf("Failed to open file\n");
        return 1;
    }
    
    // 写入数据
    fprintf(file, "Hello, %s!\n", "World");
    fprintf(file, "Number: %d\n", 42);
    
    // 关闭文件
    fclose(file);
    
    // 读取文件
    file = fopen("test.txt", "r");
    if (file != null) {
        char[256] buffer;
        while (fgets(buffer as ptr, 256, file) != null) {
            printf("Read: %s", buffer as c_string);
        }
        fclose(file);
    }
    
    return 0;
}
```

### 示例 2：动态内存

```cavy
#include <std/ffi.cay>

public int main() {
    // 分配内存
    c_int arr_count = 100;
    void_ptr mem = malloc(arr_count * sizeof(c_int));
    if (mem == null) {
        printf("Memory allocation failed\n");
        return 1;
    }
    
    // 使用内存
    ptr<c_int> arr = mem as ptr<c_int>;
    for (c_int i = 0; i < arr_count; i = i + 1) {
        arr[i] = i * i;
    }
    
    // 打印结果
    for (c_int i = 0; i < 10; i = i + 1) {
        printf("arr[%d] = %d\n", i, arr[i]);
    }
    
    // 释放内存
    free(mem);
    
    return 0;
}
```

### 示例 3：排序

```cavy
#include <std/ffi.cay>

// 比较函数
fn compare_ints(a: ptr, b: ptr) -> c_int {
    c_int x = *(a as ptr<c_int>);
    c_int y = *(b as ptr<c_int>);
    return x - y;
}

public int main() {
    // 创建数组
    c_int count = 10;
    void_ptr mem = malloc(count * sizeof(c_int));
    ptr<c_int> arr = mem as ptr<c_int>;
    
    // 填充随机数
    srand(time(null) as c_uint);
    for (c_int i = 0; i < count; i = i + 1) {
        arr[i] = rand() % 100;
    }
    
    // 打印排序前
    printf("Before: ");
    for (c_int i = 0; i < count; i = i + 1) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    
    // 排序
    qsort(mem, count as size_t, sizeof(c_int), compare_ints);
    
    // 打印排序后
    printf("After:  ");
    for (c_int i = 0; i < count; i = i + 1) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    
    free(mem);
    return 0;
}
```

### 示例 4：字符串操作

```cavy
#include <std/ffi.cay>

public int main() {
    // 字符串复制
    char[256] dest;
    strcpy(dest as char_ptr, "Hello, World!");
    printf("Copied: %s\n", dest as c_string);
    
    // 字符串长度
    size_t len = strlen(dest as c_string);
    printf("Length: %zu\n", len);
    
    // 字符串连接
    strncat(dest as char_ptr, " Welcome!", 256 - len - 1);
    printf("Concatenated: %s\n", dest as c_string);
    
    // 字符串比较
    c_int cmp = strcmp(dest as c_string, "Hello, World!");
    printf("Compare result: %d\n", cmp);
    
    // 查找子串
    char_ptr found = strstr(dest as c_string, "World");
    if (found != null) {
        printf("Found 'World' at position: %zu\n", found - (dest as char_ptr));
    }
    
    return 0;
}
```

## 注意事项

1. **内存管理**：使用 `malloc` 分配的内存必须使用 `free` 释放，避免内存泄漏。

2. **类型安全**：FFI 函数不进行类型检查，调用时必须确保参数类型正确。

3. **缓冲区溢出**：使用字符串函数时注意缓冲区大小，优先使用带 `n` 的安全版本（如 `strncpy`）。

4. **错误处理**：许多 C 函数通过返回值或全局变量 `errno` 报告错误，调用后应检查。

5. **平台差异**：某些函数（如 Windows API 和 POSIX 函数）是平台相关的，编写跨平台代码时需要条件编译。

6. **线程安全**：C 标准库函数不一定是线程安全的，多线程环境下使用需要额外注意。

## 相关文档

- [FFI 指南](ffi-guide.md) - FFI 使用详细指南
- [语法参考](syntax-reference.md) - Cavvy 语法参考
- [语言指南](language-guide.md) - Cavvy 语言特性指南
