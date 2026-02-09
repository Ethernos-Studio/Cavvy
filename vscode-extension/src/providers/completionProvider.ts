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
        'new', 'null', 'true', 'false', 'import', 'package', 'this'
    ];
    
    // 内置方法
    private builtinMethods: { name: string; detail: string; documentation: string }[] = [
        { name: 'print', detail: 'print(value: any) -> void', documentation: '打印值到控制台（不换行）' },
        { name: 'println', detail: 'println(value: any) -> void', documentation: '打印值到控制台并换行' },
        { name: 'length', detail: 'length() -> int', documentation: '获取字符串或数组的长度' },
        { name: 'charAt', detail: 'charAt(index: int) -> char', documentation: '获取字符串指定位置的字符' },
        { name: 'indexOf', detail: 'indexOf(str: string) -> int', documentation: '查找子字符串的位置' },
        { name: 'substring', detail: 'substring(start: int, end: int) -> string', documentation: '获取子字符串' },
        { name: 'concat', detail: 'concat(str: string) -> string', documentation: '连接字符串' },
        { name: 'replace', detail: 'replace(old: string, new: string) -> string', documentation: '替换字符串' },
        { name: 'toString', detail: 'toString() -> string', documentation: '转换为字符串' }
    ];
    
    // 代码片段
    private snippets: { name: string; snippet: string; detail: string }[] = [
        { 
            name: 'class', 
            snippet: 'public class ${1:ClassName} {\n    public static void main() {\n        $2\n    }\n}',
            detail: '创建类'
        },
        { 
            name: 'main', 
            snippet: 'public static void main() {\n    $1\n}',
            detail: '创建 main 方法'
        },
        { 
            name: 'for', 
            snippet: 'for (int ${1:i} = 0; ${1:i} < ${2:count}; ${1:i}++) {\n    $3\n}',
            detail: 'for 循环'
        },
        { 
            name: 'while', 
            snippet: 'while (${1:condition}) {\n    $2\n}',
            detail: 'while 循环'
        },
        { 
            name: 'if', 
            snippet: 'if (${1:condition}) {\n    $2\n}',
            detail: 'if 语句'
        },
        { 
            name: 'ifelse', 
            snippet: 'if (${1:condition}) {\n    $2\n} else {\n    $3\n}',
            detail: 'if-else 语句'
        },
        { 
            name: 'switch', 
            snippet: 'switch (${1:value}) {\n    case ${2:1}:\n        $3\n        break;\n    default:\n        break;\n}',
            detail: 'switch 语句'
        },
        { 
            name: 'method', 
            snippet: '${1:public} ${2:static} ${3:void} ${4:methodName}(${5:params}) {\n    $6\n}',
            detail: '创建方法'
        },
        {
            name: 'println',
            snippet: 'println(${1:message});',
            detail: '打印并换行'
        },
        {
            name: 'print',
            snippet: 'print(${1:message});',
            detail: '打印'
        },
        {
            name: 'newarray',
            snippet: '${1:int}[] ${2:arr} = new ${1:int}[${3:size}];',
            detail: '创建数组'
        },
        {
            name: 'atmain',
            snippet: '@main\npublic static void ${1:main}() {\n    $2\n}',
            detail: '@main 注解'
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
