import * as vscode from 'vscode';

/**
 * 代码补全提供器
 * 提供 Cavvy 语言的智能代码补全
 */
export class CavvyCompletionProvider implements vscode.CompletionItemProvider {

    // 关键字
    private keywords: string[] = [
        'public', 'private', 'protected', 'static', 'final', 'abstract', 'native',
        'class', 'void', 'int', 'long', 'float', 'double', 'bool', 'char', 'string',
        'if', 'else', 'while', 'for', 'do', 'switch', 'case', 'default', 'break', 'continue', 'return',
        'new', 'null', 'true', 'false', 'this', 'extends', 'implements', 'interface', 'enum'
    ];

    // 预处理器指令
    private preprocessorDirectives: { name: string; detail: string; documentation: string; snippet?: string }[] = [
        {
            name: '#define',
            detail: '#define MACRO [value]',
            documentation: '定义一个宏。可以用于条件编译或简单的文本替换。\n\n示例：\n#define DEBUG\n#define VERSION "0.3.5.0"',
            snippet: '#define ${1:MACRO_NAME}${2: ${3:value}}'
        },
        {
            name: '#ifdef',
            detail: '#ifdef MACRO',
            documentation: '条件编译：如果宏已定义，则包含后续代码块。必须以 #endif 结束。\n\n示例：\n#ifdef DEBUG\n    println("Debug mode");\n#endif',
            snippet: '#ifdef ${1:MACRO_NAME}\n$2\n#endif'
        },
        {
            name: '#ifndef',
            detail: '#ifndef MACRO',
            documentation: '条件编译：如果宏未定义，则包含后续代码块。必须以 #endif 结束。\n\n示例：\n#ifndef RELEASE\n    println("Development mode");\n#endif',
            snippet: '#ifndef ${1:MACRO_NAME}\n$2\n#endif'
        },
        {
            name: '#endif',
            detail: '#endif',
            documentation: '结束条件编译块。与 #ifdef 或 #ifndef 配对使用。'
        },
        {
            name: '#undef',
            detail: '#undef MACRO',
            documentation: '取消定义一个宏。\n\n示例：\n#undef DEBUG',
            snippet: '#undef ${1:MACRO_NAME}'
        }
    ];

    // 内置方法
    private builtinMethods: { name: string; detail: string; documentation: string }[] = [
        { name: 'print', detail: 'print(value: any) -> void', documentation: '打印值到控制台（不换行）' },
        { name: 'println', detail: 'println(value: any) -> void', documentation: '打印值到控制台并换行' },
        { name: 'readInt', detail: 'readInt() -> long', documentation: '从标准输入读取一个整数' },
        { name: 'readFloat', detail: 'readFloat() -> double', documentation: '从标准输入读取一个浮点数' },
        { name: 'readLine', detail: 'readLine() -> string', documentation: '从标准输入读取一行字符串' },
        { name: 'length', detail: 'length() -> int', documentation: '获取字符串或数组的长度' },
        { name: 'charAt', detail: 'charAt(index: int) -> char', documentation: '获取字符串指定位置的字符' },
        { name: 'indexOf', detail: 'indexOf(str: string) -> int', documentation: '查找子字符串的位置' },
        { name: 'substring', detail: 'substring(start: int, end?: int) -> string', documentation: '获取子字符串' },
        { name: 'concat', detail: 'concat(str: string) -> string', documentation: '连接字符串' },
        { name: 'replace', detail: 'replace(old: string, new: string) -> string', documentation: '替换字符串' },
        { name: 'toString', detail: 'toString() -> string', documentation: '转换为字符串' }
    ];

    // 代码片段
    private snippets: { name: string; snippet: string; detail: string; documentation?: string }[] = [
        {
            name: 'class',
            snippet: 'public class ${1:ClassName} {\n    public static void main() {\n        $2\n    }\n}',
            detail: '创建类',
            documentation: '创建一个带有 main 方法的公共类'
        },
        {
            name: 'main',
            snippet: 'public static void main() {\n    $1\n}',
            detail: '创建 main 方法',
            documentation: '创建程序入口点 main 方法'
        },
        {
            name: '@main',
            snippet: '@main\npublic class ${1:ClassName} {\n    public static void main() {\n        $2\n    }\n}',
            detail: '@main 注解类',
            documentation: '创建带有 @main 注解的类，指定程序入口'
        },
        {
            name: 'for',
            snippet: 'for (int ${1:i} = 0; ${1:i} < ${2:count}; ${1:i}++) {\n    $3\n}',
            detail: 'for 循环',
            documentation: '标准 for 循环结构'
        },
        {
            name: 'fori',
            snippet: 'for (int ${1:i} = ${2:0}; ${1:i} < ${3:array}.length; ${1:i}++) {\n    $4\n}',
            detail: '数组遍历 for 循环',
            documentation: '遍历数组的标准 for 循环'
        },
        {
            name: 'while',
            snippet: 'while (${1:condition}) {\n    $2\n}',
            detail: 'while 循环',
            documentation: 'while 循环结构'
        },
        {
            name: 'dowhile',
            snippet: 'do {\n    $2\n} while (${1:condition});',
            detail: 'do-while 循环',
            documentation: '至少执行一次的 do-while 循环'
        },
        {
            name: 'if',
            snippet: 'if (${1:condition}) {\n    $2\n}',
            detail: 'if 语句',
            documentation: '条件判断语句'
        },
        {
            name: 'ifelse',
            snippet: 'if (${1:condition}) {\n    $2\n} else {\n    $3\n}',
            detail: 'if-else 语句',
            documentation: '条件判断与备选分支'
        },
        {
            name: 'switch',
            snippet: 'switch (${1:value}) {\n    case ${2:1}:\n        $3\n        break;\n    default:\n        break;\n}',
            detail: 'switch 语句',
            documentation: '多分支选择语句'
        },
        {
            name: 'method',
            snippet: '${1:public} ${2:static} ${3:void} ${4:methodName}(${5:params}) {\n    $6\n}',
            detail: '创建方法',
            documentation: '创建类方法'
        },
        {
            name: 'method-varargs',
            snippet: '${1:public} ${2:static} ${3:int} ${4:methodName}(${3:int}... ${5:args}) {\n    $6\n}',
            detail: '可变参数方法',
            documentation: '创建接受可变数量参数的方法'
        },
        {
            name: 'println',
            snippet: 'println(${1:message});',
            detail: '打印并换行',
            documentation: '输出内容到控制台并换行'
        },
        {
            name: 'print',
            snippet: 'print(${1:message});',
            detail: '打印',
            documentation: '输出内容到控制台不换行'
        },
        {
            name: 'newarray',
            snippet: '${1:int}[] ${2:arr} = new ${1:int}[${3:size}];',
            detail: '创建一维数组',
            documentation: '创建指定类型和大小的数组'
        },
        {
            name: 'newarray2d',
            snippet: '${1:int}[][] ${2:matrix} = new ${1:int}[${3:rows}][${4:cols}];',
            detail: '创建二维数组',
            documentation: '创建二维矩阵数组'
        },
        {
            name: 'array-init',
            snippet: '${1:int}[] ${2:arr} = {${3:1, 2, 3}};',
            detail: '数组初始化',
            documentation: '使用初始化列表创建数组'
        },
        {
            name: 'ifdef',
            snippet: '#ifdef ${1:DEBUG}\n$2\n#endif',
            detail: '条件编译 #ifdef',
            documentation: '如果宏已定义则编译代码块'
        },
        {
            name: 'ifndef',
            snippet: '#ifndef ${1:RELEASE}\n$2\n#endif',
            detail: '条件编译 #ifndef',
            documentation: '如果宏未定义则编译代码块'
        },
        {
            name: 'define',
            snippet: '#define ${1:MACRO_NAME}',
            detail: '定义宏',
            documentation: '定义预处理器宏'
        },
        {
            name: 'final-var',
            snippet: 'final ${1:int} ${2:CONST_NAME} = ${3:value};',
            detail: 'final 常量',
            documentation: '定义不可修改的常量'
        },
        {
            name: 'static-field',
            snippet: 'static ${1:int} ${2:fieldName}${3: = ${4:initialValue}};',
            detail: '静态字段',
            documentation: '定义类级别的静态字段'
        },
        {
            name: 'cast',
            snippet: '(${1:int})${2:expression}',
            detail: '类型转换',
            documentation: '显式类型转换'
        }
    ];

    /**
     * 提供代码补全项
     */
    provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.CompletionContext
    ): vscode.ProviderResult<vscode.CompletionItem[] | vscode.CompletionList> {

        const completions: vscode.CompletionItem[] = [];
        const lineText = document.lineAt(position).text.substring(0, position.character);

        // 检查是否在行首（可能输入预处理器指令）
        const isLineStart = /^\s*$/.test(lineText);
        const isPreprocessor = /^\s*#/.test(lineText);

        // 如果在行首或已输入 #，添加预处理器指令
        if (isLineStart || isPreprocessor) {
            this.preprocessorDirectives.forEach(directive => {
                const item = new vscode.CompletionItem(directive.name, vscode.CompletionItemKind.Keyword);
                item.detail = directive.detail;
                item.documentation = new vscode.MarkdownString(directive.documentation);
                if (directive.snippet) {
                    item.insertText = new vscode.SnippetString(directive.snippet);
                }
                item.sortText = '0' + directive.name; // 让预处理器指令排在前面
                completions.push(item);
            });
        }

        // 添加关键字
        this.keywords.forEach(keyword => {
            const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
            item.detail = '关键字';
            completions.push(item);
        });

        // 添加内置方法
        this.builtinMethods.forEach(method => {
            const item = new vscode.CompletionItem(method.name, vscode.CompletionItemKind.Function);
            item.detail = method.detail;
            item.documentation = new vscode.MarkdownString(method.documentation);
            item.insertText = method.name + '($1)';
            item.command = { command: 'editor.action.triggerParameterHints', title: '触发参数提示' };
            completions.push(item);
        });

        // 添加代码片段
        this.snippets.forEach(snippet => {
            const item = new vscode.CompletionItem(snippet.name, vscode.CompletionItemKind.Snippet);
            item.detail = snippet.detail;
            if (snippet.documentation) {
                item.documentation = new vscode.MarkdownString(snippet.documentation);
            }
            item.insertText = new vscode.SnippetString(snippet.snippet);
            completions.push(item);
        });

        // 从文档中提取用户定义的符号
        const userSymbols = this.extractUserDefinedSymbols(document);
        userSymbols.forEach(symbol => {
            let kind: vscode.CompletionItemKind;
            switch (symbol.type) {
                case 'class':
                    kind = vscode.CompletionItemKind.Class;
                    break;
                case 'method':
                    kind = vscode.CompletionItemKind.Method;
                    break;
                case 'variable':
                    kind = vscode.CompletionItemKind.Variable;
                    break;
                default:
                    kind = vscode.CompletionItemKind.Text;
            }
            const item = new vscode.CompletionItem(symbol.name, kind);
            item.detail = `用户定义的${symbol.type}`;
            completions.push(item);
        });

        return completions;
    }

    /**
     * 提取用户定义的符号
     */
    private extractUserDefinedSymbols(document: vscode.TextDocument): Array<{ name: string; type: string }> {
        const symbols: Array<{ name: string; type: string }> = [];
        const text = document.getText();

        // 提取类名
        const classMatches = text.match(/class\s+([a-zA-Z_][a-zA-Z0-9_]*)/g);
        if (classMatches) {
            classMatches.forEach(match => {
                const name = match.replace(/class\s+/, '');
                symbols.push({ name, type: 'class' });
            });
        }

        // 提取方法名
        const methodMatches = text.match(/\b(?:public|private|protected)?\s*(?:static)?\s*(?:final)?\s*(?:int|long|float|double|bool|string|char|void)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g);
        if (methodMatches) {
            methodMatches.forEach(match => {
                const name = match.replace(/.*\s+/, '').replace('(', '');
                symbols.push({ name, type: 'method' });
            });
        }

        // 提取变量名
        const varMatches = text.match(/\b(int|long|float|double|bool|string|char)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g);
        if (varMatches) {
            varMatches.forEach(match => {
                const name = match.replace(/.*\s+/, '');
                if (!symbols.some(s => s.name === name)) {
                    symbols.push({ name, type: 'variable' });
                }
            });
        }

        return symbols;
    }
}
