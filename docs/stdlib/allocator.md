# Allocator - 内存分配器标准库

## 版本信息

- **版本**: 0.5.0.0
- **文件**: `caylibs/Allocator.cay`
- **依赖**: C标准库 (malloc, free)

## 概述

Allocator 模块提供了 Cavvy 的内存分配抽象，包括全局堆分配器、竞技场分配器和作用域分配器三种实现。该模块遵循 RAII 原则，确保内存资源的正确管理。

## 接口定义

### Allocator 接口

所有分配器必须实现的接口：

```cay
public interface Allocator {
    long allocate(long size);                          // 分配指定大小内存
    long allocateAligned(long size, long alignBytes);  // 对齐分配
    void deallocate(long ptr);                         // 释放内存
}
```

## 实现类

### GlobalAlloc - 全局堆分配器

基于 C 标准库 malloc/free 的全局分配器，使用单例模式。

**时间复杂度**: O(1) 分配/释放
**空间复杂度**: O(n) n=分配大小

```cay
// 获取单例实例
GlobalAlloc alloc = GlobalAlloc.getInstance();

// 分配内存
long ptr = alloc.allocate(1024);

// 对齐分配（16字节对齐）
long aligned = alloc.allocateAligned(1024, 16);

// 释放内存
alloc.deallocate(ptr);
```

**宏定义**:
```cay
#define GLOBAL_ALLOC GlobalAlloc.getInstance()
```

### Arena - 竞技场分配器

适用于大量小对象分配的场景，支持批量释放。

**时间复杂度**: O(1) 分配, O(1) 批量释放
**空间复杂度**: O(capacity)

```cay
// 创建容量为64KB的Arena
Arena arena = Arena.create(65536);

// 分配内存
long obj1 = arena.allocate(100);
long obj2 = arena.allocate(200);

// 获取使用统计
long used = arena.used();         // 已使用字节数
long remaining = arena.remaining(); // 剩余字节数

// 重置Arena（批量释放所有内存）
arena.reset();
```

**特点**:
- 线性分配，无内存碎片
- 不支持单独释放单个对象
- 适合游戏帧分配、请求处理等场景

**宏定义**:
```cay
#define ARENA(capacity) Arena.create(capacity)
```

### ScopeAlloc - 作用域分配器

与 Cavvy 的 `scope` 关键字配合使用，提供栈式内存分配。

**时间复杂度**: O(1)
**空间复杂度**: O(1)

```cay
// 创建作用域分配器
ScopeAlloc scope = ScopeAlloc.create();

// 设置标记点
scope.setMarker(0);

// 获取当前标记
long marker = scope.getMarker();
```

**宏定义**:
```cay
#define SCOPE_ALLOC ScopeAlloc.create()
```

## 使用示例

### 基本内存分配

```cay
#include <Allocator.cay>

public class Main {
    public static void main(String[] args) {
        // 使用全局分配器
        GlobalAlloc alloc = GLOBAL_ALLOC;
        
        // 分配1024字节
        long buffer = alloc.allocate(1024);
        if (buffer == 0) {
            println("内存分配失败");
            return;
        }
        
        // 使用内存...
        
        // 释放内存
        alloc.deallocate(buffer);
    }
}
```

### Arena分配器模式

```cay
#include <Allocator.cay>

public class ParticleSystem {
    private Arena arena;
    
    public ParticleSystem() {
        // 为粒子系统预分配1MB内存
        this.arena = ARENA(1024 * 1024);
    }
    
    public Particle createParticle() {
        long ptr = arena.allocate(Particle.SIZE);
        return new Particle(ptr);
    }
    
    public void update() {
        // 每帧重置，释放所有粒子
        arena.reset();
    }
}
```

### 对齐分配

```cay
#include <Allocator.cay>

public class Main {
    public static void main(String[] args) {
        GlobalAlloc alloc = GLOBAL_ALLOC;
        
        // 16字节对齐（SSE要求）
        long sseBuffer = alloc.allocateAligned(1024, 16);
        
        // 64字节对齐（缓存行对齐）
        long cacheBuffer = alloc.allocateAligned(1024, 64);
        
        // 4096字节对齐（页对齐）
        long pageBuffer = alloc.allocateAligned(4096, 4096);
        
        alloc.deallocate(sseBuffer);
        alloc.deallocate(cacheBuffer);
        alloc.deallocate(pageBuffer);
    }
}
```

## 性能对比

| 分配器 | 分配速度 | 释放速度 | 内存碎片 | 适用场景 |
|--------|----------|----------|----------|----------|
| GlobalAlloc | 中等 | 中等 | 有 | 通用分配 |
| Arena | 极快 | 极快(O(1)批量) | 无 | 大量临时对象 |
| ScopeAlloc | 极快 | 自动 | 无 | 栈式分配 |

## 最佳实践

1. **短期对象使用 Arena**: 避免频繁 malloc/free 的开销
2. **长期对象使用 GlobalAlloc**: 支持单独释放
3. **对齐分配用于硬件交互**: DMA、GPU缓冲区需要特定对齐
4. **检查返回值**: 所有 allocate 方法可能返回 0 表示失败

## 注意事项

- 不要混用不同分配器的分配/释放
- Arena 不支持单独释放，只能 reset
- 对齐分配的指针可能不等于原始 malloc 返回的指针
- 作用域分配器需要语言级别的 `scope` 关键字支持
