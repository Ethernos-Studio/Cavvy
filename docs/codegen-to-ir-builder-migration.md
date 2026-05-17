# CodeGen -> IR Builder 管线迁移路线图

## 版本: 0.5.x (阶段三：零开销标准库)

**目标**: 逐步将代码生成从直接的LLVM IR字符串拼接(CodeGen)迁移到结构化的IR Builder API，实现更安全、更灵活的IR生成，同时保持CodeGen与IR Builder的协作能力。

---

## 当前架构分析

### CodeGen管线 (现有)
```
AST -> CodeGen (字符串拼接) -> LLVM IR文本 -> ir2exe -> 可执行文件
```

**特点**:
- 直接操作LLVM IR字符串
- 高可控性，低抽象
- 容易出错（语法错误、类型不匹配）
- 难以进行IR级优化

### IR Builder管线 (目标)
```
AST -> IR Builder (结构化API) -> IR Module -> LLVM IR文本 -> ir2exe -> 可执行文件
```

**特点**:
- 结构化IR表示
- 类型安全
- 支持IR级分析和优化
- 更易于维护和扩展

### 协作模式 (0.5.x 核心)
```
AST -> CodeGen + IR Builder (协作) -> 统一LLVM IR -> ir2exe -> 可执行文件
```

**设计原则**:
- CodeGen负责高层结构（函数定义、类布局）
- IR Builder负责复杂表达式和优化
- 两者通过Bridge模块协作

---

## 迁移阶段规划

### 阶段 0: 基础设施 (0.5.0.x) ✅ 已完成

**目标**: 建立CodeGen与IR Builder的协作桥梁

**已完成工作**:
- [x] 实现 `InlineIrBridge` 模块
- [x] 支持内联IR语法 `__ir { ... }`
- [x] 变量映射系统（参数索引、变量名）
- [x] 类型转换桥接
- [x] 8个完整测试用例

**关键组件**:
```rust
// src/codegen/bridge.rs
pub struct InlineIrBridge {
    parser: InlineIrParser,
}

impl InlineIrBridge {
    pub fn process_inline_ir(
        &self,
        codegen: &IRGenerator,
        inline_ir: &InlineIrStmt,
    ) -> cayResult<InlineIrResult>;
}
```

**验证标准**:
- [x] 内联IR可以正确引用函数参数
- [x] 内联IR可以正确引用局部变量
- [x] 生成的LLVM IR能通过clang编译
- [x] 所有121个测试通过

---

### 阶段 1: 表达式迁移 (0.5.1.x)

**目标**: 将复杂表达式生成迁移到IR Builder

**范围**:
1. **算术表达式**
   - 当前: CodeGen直接生成 `add`, `sub`, `mul`, `sdiv`, `srem` 指令
   - 目标: 通过IR Builder构建表达式树
   - 复杂度: O(n) 表达式节点数

2. **逻辑表达式**
   - 当前: CodeGen生成 `and`, `or`, `xor`, `icmp` 指令
   - 目标: IR Builder统一处理布尔运算
   - 复杂度: O(n)

3. **比较表达式**
   - 当前: CodeGen生成各种 `icmp` 变体
   - 目标: IR Builder抽象比较操作
   - 复杂度: O(1)

**实现方案**:
```rust
// src/codegen/bridge.rs 扩展
pub struct ExpressionBridge {
    builder: IrBuilder,
}

impl ExpressionBridge {
    /// 生成算术表达式
    /// 时间复杂度: O(n), n为表达式节点数
    pub fn generate_arithmetic_expr(
        &mut self,
        op: BinaryOp,
        left: IrValue,
        right: IrValue,
    ) -> IrValue;
    
    /// 生成比较表达式
    /// 时间复杂度: O(1)
    pub fn generate_comparison_expr(
        &mut self,
        op: ComparisonOp,
        left: IrValue,
        right: IrValue,
    ) -> IrValue;
}
```

**迁移步骤**:
1. 在 `src/ir/builder.rs` 扩展表达式构建API
2. 在 `src/codegen/bridge.rs` 添加ExpressionBridge
3. 逐步替换 `src/codegen/expressions/` 中的表达式生成
4. 每个替换点添加功能开关，支持回滚

**回滚策略**:
```rust
// 在codegen中保留旧实现
if use_ir_builder_expressions {
    expression_bridge.generate_arithmetic_expr(...)
} else {
    // 旧实现
    self.emit_line(format!("{} = add i32 {}, {}", ...));
}
```

**验证标准**:
- [ ] 算术表达式测试通过
- [ ] 逻辑表达式测试通过
- [ ] 比较表达式测试通过
- [ ] 性能不下降（对比基准测试）

---

### 阶段 2: 控制流迁移 (0.5.2.x)

**目标**: 将控制流结构迁移到IR Builder

**范围**:
1. **if/else 语句**
   - 当前: CodeGen手动管理基本块和分支
   - 目标: IR Builder提供结构化控制流API
   - 复杂度: O(1) 每个if语句

2. **循环语句 (while, for, do-while)**
   - 当前: CodeGen生成 `br` 指令管理循环
   - 目标: IR Builder提供循环构建API
   - 复杂度: O(1) 每个循环

3. **switch 语句**
   - 当前: CodeGen生成 `switch` 指令
   - 目标: IR Builder统一处理多分支
   - 复杂度: O(n) n个case

**实现方案**:
```rust
// src/ir/builder.rs 新增
pub struct ControlFlowBuilder {
    current_function: FunctionRef,
}

impl ControlFlowBuilder {
    /// 构建if-else结构
    /// 时间复杂度: O(1)
    pub fn build_if_else<F, G>(
        &mut self,
        condition: IrValue,
        then_builder: F,
        else_builder: Option<G>,
    ) -> BasicBlockRef
    where F: FnOnce(&mut Self),
          G: FnOnce(&mut Self);
    
    /// 构建while循环
    /// 时间复杂度: O(1)
    pub fn build_while<F>(
        &mut self,
        condition_builder: impl FnOnce(&mut Self) -> IrValue,
        body_builder: F,
    ) -> BasicBlockRef
    where F: FnOnce(&mut Self);
}
```

**迁移步骤**:
1. 在IR Builder中实现控制流原语
2. 在Bridge中添加ControlFlowBridge
3. 替换 `src/codegen/statements/conditionals.rs` 中的if/else生成
4. 替换 `src/codegen/statements/loops.rs` 中的循环生成

**验证标准**:
- [ ] 所有控制流测试通过
- [ ] 嵌套控制流正确
- [ ] break/continue 正确工作
- [ ] PHI节点正确生成

---

### 阶段 3: 函数与调用迁移 (0.5.3.x)

**目标**: 将函数定义和调用迁移到IR Builder

**范围**:
1. **函数定义**
   - 当前: CodeGen生成 `define` 指令和基本块
   - 目标: IR Builder构建函数IR
   - 复杂度: O(n) n条指令

2. **函数调用**
   - 当前: CodeGen生成 `call` 指令
   - 目标: IR Builder统一管理调用约定
   - 复杂度: O(m) m个参数

3. **方法调用与虚函数**
   - 当前: CodeGen处理vtable查找
   - 目标: IR Builder抽象动态分派
   - 复杂度: O(1)

**实现方案**:
```rust
// src/ir/builder.rs 新增
pub struct FunctionBuilder {
    module: ModuleRef,
}

impl FunctionBuilder {
    /// 定义函数
    /// 时间复杂度: O(n), n为指令数
    pub fn define_function(
        &mut self,
        name: &str,
        params: Vec<(String, IrType)>,
        ret_type: IrType,
        body_builder: impl FnOnce(&mut IrBuilder),
    ) -> FunctionRef;
    
    /// 生成函数调用
    /// 时间复杂度: O(m), m为参数数
    pub fn build_call(
        &mut self,
        func: FunctionRef,
        args: Vec<IrValue>,
    ) -> Option<IrValue>;
}
```

**迁移步骤**:
1. 实现FunctionBuilder API
2. 替换顶层函数生成
3. 替换类方法生成
4. 替换函数调用生成

**验证标准**:
- [ ] 函数定义测试通过
- [ ] 函数调用测试通过
- [ ] 递归调用正确
- [ ] 方法调用正确

---

### 阶段 4: 类型系统迁移 (0.5.4.x)

**目标**: 将类型定义和布局迁移到IR Builder

**范围**:
1. **结构体定义**
   - 当前: CodeGen生成 `type { ... }` 定义
   - 目标: IR Builder构建类型IR
   - 复杂度: O(f) f个字段

2. **类布局与vtable**
   - 当前: CodeGen手动计算类布局
   - 目标: IR Builder统一管理类IR
   - 复杂度: O(m) m个方法

3. **数组类型**
   - 当前: CodeGen处理数组索引计算
   - 目标: IR Builder提供数组操作API
   - 复杂度: O(1)

**实现方案**:
```rust
// src/ir/builder.rs 新增
pub struct TypeBuilder {
    module: ModuleRef,
}

impl TypeBuilder {
    /// 定义结构体类型
    /// 时间复杂度: O(f), f为字段数
    pub fn define_struct(
        &mut self,
        name: &str,
        fields: Vec<(String, IrType)>,
    ) -> TypeRef;
    
    /// 定义类类型（含vtable）
    /// 时间复杂度: O(m), m为方法数
    pub fn define_class(
        &mut self,
        name: &str,
        fields: Vec<(String, IrType)>,
        methods: Vec<MethodRef>,
        parent: Option<TypeRef>,
    ) -> TypeRef;
}
```

**迁移步骤**:
1. 实现TypeBuilder API
2. 替换结构体定义生成
3. 替换类定义生成
4. 统一类型引用

**验证标准**:
- [ ] 结构体测试通过
- [ ] 类继承测试通过
- [ ] 多态调用正确
- [ ] 内存布局正确

---

### 阶段 5: 内存操作迁移 (0.5.5.x)

**目标**: 将内存分配和访问迁移到IR Builder

**范围**:
1. **栈分配 (alloca)**
   - 当前: CodeGen生成 `alloca` 指令
   - 目标: IR Builder统一管理栈空间
   - 复杂度: O(1)

2. **堆分配**
   - 当前: CodeGen调用malloc
   - 目标: IR Builder提供分配器抽象
   - 复杂度: O(1)

3. **内存访问 (load/store/GEP)**
   - 当前: CodeGen手动生成内存指令
   - 目标: IR Builder提供安全内存API
   - 复杂度: O(1)

**实现方案**:
```rust
// src/ir/builder.rs 新增
pub struct MemoryBuilder {
    builder: IrBuilder,
}

impl MemoryBuilder {
    /// 栈分配
    /// 时间复杂度: O(1)
    pub fn alloca(&mut self, ty: IrType, name: &str) -> IrValue;
    
    /// 堆分配
    /// 时间复杂度: O(1)
    pub fn malloc(&mut self, size: IrValue, align: usize) -> IrValue;
    
    /// 安全加载
    /// 时间复杂度: O(1)
    pub fn load(&mut self, ptr: IrValue, ty: IrType) -> IrValue;
    
    /// 安全存储
    /// 时间复杂度: O(1)
    pub fn store(&mut self, ptr: IrValue, value: IrValue);
    
    /// 获取元素指针
    /// 时间复杂度: O(1)
    pub fn gep(&mut self, ptr: IrValue, indices: Vec<IrValue>) -> IrValue;
}
```

**迁移步骤**:
1. 实现MemoryBuilder API
2. 替换变量声明中的alloca
3. 替换所有load/store操作
4. 替换数组索引计算(GEP)

**验证标准**:
- [ ] 变量生命周期测试通过
- [ ] 数组访问测试通过
- [ ] 对象字段访问测试通过
- [ ] 无内存泄漏

---

### 阶段 6: 优化集成 (0.5.6.x)

**目标**: 在IR Builder管线中集成优化

**范围**:
1. **IR级分析**
   - 定义使用链(Def-Use Chain)
   - 控制流图(CFG)
   - 支配树(Dominator Tree)

2. **基础优化**
   - 常量传播
   - 死代码消除
   - 公共子表达式消除

3. **Cavvy特定优化**
   - 内联IR优化
   - 虚函数去虚拟化
   - 逃逸分析

**实现方案**:
```rust
// src/ir/optimizer.rs 新增
pub struct IrOptimizer {
    module: ModuleRef,
}

impl IrOptimizer {
    /// 运行优化pass
    /// 时间复杂度: O(n * p), n为指令数, p为pass数
    pub fn optimize(&mut self, passes: Vec<OptimizationPass>);
    
    /// 常量传播
    /// 时间复杂度: O(n)
    pub fn constant_propagation(&mut self);
    
    /// 死代码消除
    /// 时间复杂度: O(n)
    pub fn dead_code_elimination(&mut self);
}
```

**迁移步骤**:
1. 实现IR分析基础设施
2. 实现基础优化pass
3. 在编译管线中插入优化阶段
4. 对比优化前后的性能

**验证标准**:
- [ ] 优化不改变语义
- [ ] 性能提升>10%
- [ ] 编译时间不增加>20%

---

### 阶段 7: 完全迁移 (0.5.7.x)

**目标**: 完全切换到IR Builder管线，CodeGen仅作为薄封装层

**架构变化**:
```
Before:
  AST -> CodeGen (字符串拼接) -> LLVM IR

After:
  AST -> IR Builder (结构化IR) -> IR Module -> LLVM IR文本
```

**CodeGen新角色**:
- 高层AST遍历和语义检查
- 调用IR Builder API
- 处理Cavvy特定语义

**移除的CodeGen功能**:
- 直接字符串拼接
- 手动基本块管理
- 手动寄存器命名

**保留的CodeGen功能**:
- 类布局计算
- vtable生成
- 外部函数声明

**验证标准**:
- [ ] 所有测试通过
- [ ] 性能不下降
- [ ] 代码复杂度降低
- [ ] 新功能开发效率提升

---

## 风险管理

### 技术风险

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| IR Builder API设计缺陷 | 中 | 高 | 阶段0充分测试，保持向后兼容 |
| 性能回退 | 中 | 高 | 每个阶段都有性能基准测试 |
| 代码复杂度增加 | 低 | 中 | 定期代码审查，重构 |
| 测试覆盖不足 | 中 | 高 | 强制要求新功能100%测试覆盖 |

### 迁移策略

1. **渐进式迁移**
   - 每个阶段独立可回滚
   - 功能开关控制新旧实现
   - 灰度发布，逐步扩大范围

2. **双轨并行**
   - 旧管线保持维护
   - 新管线并行开发
   - 定期对比测试

3. **自动化验证**
   - 每个PR必须通过全部测试
   - 性能基准自动化对比
   - 代码覆盖率检查

---

## 时间线

| 阶段 | 版本 | 预计周期 | 关键里程碑 |
|------|------|----------|------------|
| 0 | 0.5.0.x | 已完成 | 协作桥建立 ✅ |
| 1 | 0.5.1.x | 4周 | 表达式迁移完成 |
| 2 | 0.5.2.x | 4周 | 控制流迁移完成 |
| 3 | 0.5.3.x | 4周 | 函数迁移完成 |
| 4 | 0.5.4.x | 4周 | 类型系统迁移完成 |
| 5 | 0.5.5.x | 4周 | 内存操作迁移完成 |
| 6 | 0.5.6.x | 4周 | 优化集成完成 |
| 7 | 0.5.7.x | 4周 | 完全迁移完成 |

**总计**: 约6个月完成完全迁移

---

## 成功指标

### 技术指标
- [ ] 代码生成正确性: 100%测试通过
- [ ] 性能: 不劣于旧管线
- [ ] 编译时间: 增加<20%
- [ ] 代码复杂度: 降低>30%

### 工程指标
- [ ] 新功能开发效率: 提升>50%
- [ ] Bug修复时间: 减少>30%
- [ ] 代码审查时间: 减少>20%

### 维护指标
- [ ] IR级优化实现: >5种
- [ ] 文档完整度: 100%
- [ ] 开发者满意度: >4/5

---

## 附录

### A. 相关文件清单

**核心文件**:
- `src/codegen/bridge.rs` - 协作桥实现
- `src/codegen/context.rs` - CodeGen上下文
- `src/codegen/generator.rs` - 主生成器
- `src/ir/builder.rs` - IR Builder核心
- `src/ir/module.rs` - IR模块表示
- `src/ir/value.rs` - IR值类型
- `src/ir/types.rs` - IR类型系统

**测试文件**:
- `tests/inline_ir_tests.rs` - 内联IR集成测试
- `examples/test_inline_ir_*.cay` - 测试用例

### B. 术语表

- **CodeGen**: 代码生成器，直接生成LLVM IR字符串
- **IR Builder**: 结构化IR构建器，提供类型安全的API
- **Bridge**: 协作桥，连接CodeGen和IR Builder
- **Inline IR**: 内联IR，在Cavvy代码中直接嵌入LLVM IR
- **Migration**: 迁移，从旧系统逐步转移到新系统

### C. 参考文档

- [LLVM IR Reference](https://llvm.org/docs/LangRef.html)
- [ROADMAP.md](../ROADMAP.md) - 项目整体路线图
- [FFI Guide](ffi-guide.md) - FFI使用指南
