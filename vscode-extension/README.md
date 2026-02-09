# Cavvy Analyzer

Cavvy (EOL) 语言的 VSCode 插件，提供语法高亮、跳转定义、语法错误提示等功能。

## 功能特性

### 1. 语法高亮
- 完整支持 Cavvy 语言的关键字、类型、修饰符
- 字符串、字符、数字字面量高亮
- 注释高亮（单行和多行）
- 方法调用和字段访问高亮
- 注解高亮（如 `@main`）

### 2. 跳转到定义 (Ctrl+Click / F12)
- 支持跳转到类定义
- 支持跳转到方法定义
- 支持跳转到字段定义
- 支持跳转到变量定义
- 支持跳转到参数定义

### 3. 语法错误提示
- 实时语法检查
- 括号匹配检查
- 基本语法规则验证
- 支持集成 `eolc` 编译器进行深度检查
- 错误波浪线提示

### 4. 文档大纲 (Outline)
- 在 VSCode 大纲视图中显示文档结构
- 类、方法、字段、变量层级展示

## 安装方法

### 方法一：从 VSIX 安装

1. 构建插件：
```bash
cd vscode-extension
npm install
npm run compile
npm run package
```

2. 在 VSCode 中安装：
   - 按 `Ctrl+Shift+P` 打开命令面板
   - 输入 `Extensions: Install from VSIX`
   - 选择生成的 `cavvy-analyzer-0.1.0.vsix` 文件

### 方法二：开发模式运行

1. 安装依赖：
```bash
cd vscode-extension
npm install
```

2. 编译：
```bash
npm run compile
```

3. 按 `F5` 启动调试，将在新窗口中加载插件

## 配置选项

在 VSCode 设置中搜索 "Cavvy" 进行配置：

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `cavvyAnalyzer.compilerPath` | string | `eolc` | Cavvy 编译器路径 |
| `cavvyAnalyzer.enableDiagnostics` | boolean | `true` | 启用语法错误诊断 |
| `cavvyAnalyzer.diagnosticDelay` | number | `500` | 诊断延迟时间（毫秒） |

## 支持的文件扩展名

- `.cay` - Cavvy 源文件
- `.eol` - EOL 源文件

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| `F12` | 跳转到定义 |
| `Ctrl+Click` | 跳转到定义 |
| `Ctrl+Shift+O` | 显示文档符号 |
| `Ctrl+Shift+M` | 显示问题面板 |

## 示例代码

```cavvy
@main
public class HelloWorld {
    public static void main() {
        println("Hello, Cavvy!");
        
        int sum = add(10, 20);
        println(sum);
    }
    
    public static int add(int a, int b) {
        return a + b;
    }
}
```

## 开发

### 项目结构

```
vscode-extension/
├── src/
│   ├── extension.ts              # 插件入口
│   ├── providers/
│   │   ├── definitionProvider.ts # 跳转到定义
│   │   ├── diagnosticProvider.ts # 语法错误诊断
│   │   └── documentSymbolProvider.ts # 文档符号
│   └── utils/
│       └── parser.ts             # Cavvy 解析器
├── syntaxes/
│   └── cavvy.tmLanguage.json     # 语法高亮定义
├── icons/
│   └── cavvy-icon.svg            # 文件图标
├── package.json                  # 插件配置
├── tsconfig.json                 # TypeScript 配置
└── README.md                     # 本文件
```

### 构建

```bash
# 安装依赖
npm install

# 编译 TypeScript
npm run compile

# 监视模式编译
npm run watch

# 打包为 VSIX
npm run package
```

## 许可证

MIT License
