import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';

const execAsync = promisify(exec);

/**
 * 诊断提供器
 * 用于检测 Cavvy 代码中的语法错误
 */
export class CavvyDiagnosticProvider {
    
    private diagnosticCollection: vscode.DiagnosticCollection;
    private disposables: vscode.Disposable[] = [];
    private timeout: NodeJS.Timeout | undefined;
    private config: vscode.WorkspaceConfiguration;

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('cavvy');
        this.config = vscode.workspace.getConfiguration('cavvyAnalyzer');
    }

    /**
     * 激活诊断提供器
     * @param context 插件上下文
     */
    activate(context: vscode.ExtensionContext): void {
        // 监听文档打开事件
        const onDidOpenDisposable = vscode.workspace.onDidOpenTextDocument(
            (document) => this.onDocumentOpen(document)
        );
        context.subscriptions.push(onDidOpenDisposable);
        this.disposables.push(onDidOpenDisposable);

        // 监听文档内容变更事件
        const onDidChangeDisposable = vscode.workspace.onDidChangeTextDocument(
            (event) => this.onDocumentChange(event)
        );
        context.subscriptions.push(onDidChangeDisposable);
        this.disposables.push(onDidChangeDisposable);

        // 监听文档保存事件
        const onDidSaveDisposable = vscode.workspace.onDidSaveTextDocument(
            (document) => this.onDocumentSave(document)
        );
        context.subscriptions.push(onDidSaveDisposable);
        this.disposables.push(onDidSaveDisposable);

        // 监听文档关闭事件
        const onDidCloseDisposable = vscode.workspace.onDidCloseTextDocument(
            (document) => this.onDocumentClose(document)
        );
        context.subscriptions.push(onDidCloseDisposable);
        this.disposables.push(onDidCloseDisposable);

        // 初始化时检查所有已打开的文档
        vscode.workspace.textDocuments.forEach((doc) => {
            if (doc.languageId === 'cavvy') {
                this.scheduleCheck(doc);
            }
        });
    }

    /**
     * 文档打开时的处理
     * @param document 文档
     */
    private onDocumentOpen(document: vscode.TextDocument): void {
        if (document.languageId === 'cavvy') {
            this.scheduleCheck(document);
        }
    }

    /**
     * 文档内容变更时的处理
     * @param event 文本文档变更事件
     */
    private onDocumentChange(event: vscode.TextDocumentChangeEvent): void {
        if (event.document.languageId === 'cavvy') {
            this.scheduleCheck(event.document);
        }
    }

    /**
     * 文档保存时的处理
     * @param document 文档
     */
    private onDocumentSave(document: vscode.TextDocument): void {
        if (document.languageId === 'cavvy') {
            this.checkDocument(document);
        }
    }

    /**
     * 文档关闭时的处理
     * @param document 文档
     */
    private onDocumentClose(document: vscode.TextDocument): void {
        this.diagnosticCollection.delete(document.uri);
    }

    /**
     * 调度检查（带延迟）
     * @param document 文档
     */
    private scheduleCheck(document: vscode.TextDocument): void {
        if (!this.config.get<boolean>('enableDiagnostics', true)) {
            return;
        }

        // 清除之前的定时器
        if (this.timeout) {
            clearTimeout(this.timeout);
        }

        // 设置新的定时器
        const delay = this.config.get<number>('diagnosticDelay', 500);
        this.timeout = setTimeout(() => {
            this.checkDocument(document);
        }, delay);
    }

    /**
     * 检查文档语法
     * @param document 文档
     */
    async checkDocument(document: vscode.TextDocument): Promise<void> {
        if (!this.config.get<boolean>('enableDiagnostics', true)) {
            return;
        }

        const diagnostics: vscode.Diagnostic[] = [];
        
        try {
            // 首先进行基本的语法检查
            const basicDiagnostics = this.performBasicSyntaxCheck(document);
            diagnostics.push(...basicDiagnostics);
            
            // 如果配置了编译器路径，尝试使用编译器进行更详细的检查
            const compilerPath = this.config.get<string>('compilerPath', 'eolc');
            if (compilerPath && compilerPath !== '') {
                const compilerDiagnostics = await this.runCompilerCheck(document, compilerPath);
                diagnostics.push(...compilerDiagnostics);
            }
        } catch (error) {
            console.error('Cavvy 语法检查出错:', error);
        }

        this.diagnosticCollection.set(document.uri, diagnostics);
    }

    /**
     * 执行基本语法检查
     * @param document 文档
     * @returns 诊断数组
     */
    private performBasicSyntaxCheck(document: vscode.TextDocument): vscode.Diagnostic[] {
        const diagnostics: vscode.Diagnostic[] = [];
        const text = document.getText();
        const lines = text.split('\n');
        
        let inBlockComment = false;
        let braceStack: { char: string; line: number; col: number }[] = [];
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            
            // 处理块注释
            if (inBlockComment) {
                const endIndex = line.indexOf('*/');
                if (endIndex !== -1) {
                    inBlockComment = false;
                }
                continue;
            }
            
            // 检查块注释开始
            const blockCommentStart = line.indexOf('/*');
            if (blockCommentStart !== -1) {
                const blockCommentEnd = line.indexOf('*/', blockCommentStart + 2);
                if (blockCommentEnd === -1) {
                    inBlockComment = true;
                }
            }
            
            // 跳过纯注释行
            const trimmedLine = line.trim();
            if (trimmedLine.startsWith('//')) {
                continue;
            }
            
            // 检查括号匹配
            for (let j = 0; j < line.length; j++) {
                const char = line[j];
                
                // 跳过字符串内的字符
                if (char === '"' || char === "'") {
                    j++;
                    while (j < line.length && line[j] !== char) {
                        if (line[j] === '\\') j++;
                        j++;
                    }
                    continue;
                }
                
                // 跳过行注释
                if (char === '/' && j + 1 < line.length && line[j + 1] === '/') {
                    break;
                }
                
                if (char === '{' || char === '(' || char === '[') {
                    braceStack.push({ char, line: i, col: j });
                } else if (char === '}' || char === ')' || char === ']') {
                    const expectedOpen = char === '}' ? '{' : (char === ')' ? '(' : '[');
                    if (braceStack.length === 0 || braceStack[braceStack.length - 1].char !== expectedOpen) {
                        const range = new vscode.Range(i, j, i, j + 1);
                        const diagnostic = new vscode.Diagnostic(
                            range,
                            `不匹配的括号: 期望 '${expectedOpen}' 但找到 '${char}'`,
                            vscode.DiagnosticSeverity.Error
                        );
                        diagnostic.code = 'unmatched-brace';
                        diagnostics.push(diagnostic);
                    } else {
                        braceStack.pop();
                    }
                }
            }
            
            // 检查基本语法错误
            const lineDiagnostics = this.checkLineSyntax(line, i);
            diagnostics.push(...lineDiagnostics);
        }
        
        // 检查未闭合的括号
        for (const unclosed of braceStack) {
            const range = new vscode.Range(unclosed.line, unclosed.col, unclosed.line, unclosed.col + 1);
            const matching = unclosed.char === '{' ? '}' : (unclosed.char === '(' ? ')' : ']');
            const diagnostic = new vscode.Diagnostic(
                range,
                `未闭合的括号: '${unclosed.char}' 没有匹配的 '${matching}'`,
                vscode.DiagnosticSeverity.Error
            );
            diagnostic.code = 'unclosed-brace';
            diagnostics.push(diagnostic);
        }
        
        return diagnostics;
    }

    // 追踪当前上下文
    private currentContext: {
        inClass: boolean;
        inMethod: boolean;
        className: string | null;
        methodName: string | null;
        hasMainMethod: boolean;
        braceDepth: number;
    } = {
        inClass: false,
        inMethod: false,
        className: null,
        methodName: null,
        hasMainMethod: false,
        braceDepth: 0
    };

    /**
     * 检查单行语法
     * @param line 行内容
     * @param lineNumber 行号
     * @returns 诊断数组
     */
    private checkLineSyntax(line: string, lineNumber: number): vscode.Diagnostic[] {
        const diagnostics: vscode.Diagnostic[] = [];
        const trimmedLine = line.trim();
        
        // 跳过空行和纯注释行
        if (!trimmedLine || trimmedLine.startsWith('//')) {
            return diagnostics;
        }
        
        // 移除行内注释以便检查
        const lineWithoutComment = trimmedLine.replace(/\/\/.*$/, '').trim();
        
        // 更新花括号深度
        const openBraces = (lineWithoutComment.match(/{/g) || []).length;
        const closeBraces = (lineWithoutComment.match(/}/g) || []).length;
        
        // 检查类声明语法
        const classPattern = /^(?:\s*(?:public|private|protected|abstract|final)\s+)*class\s+([a-zA-Z_][a-zA-Z0-9_]*)/;
        const classMatch = classPattern.exec(lineWithoutComment);
        if (classMatch) {
            this.currentContext.inClass = true;
            this.currentContext.className = classMatch[1];
            
            // 检查类名是否以大写字母开头（Java 命名规范）
            if (!/^[A-Z]/.test(classMatch[1])) {
                const classIndex = line.indexOf(classMatch[1]);
                const range = new vscode.Range(lineNumber, classIndex, lineNumber, classIndex + classMatch[1].length);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    `类名 '${classMatch[1]}' 建议以大写字母开头（遵循 PascalCase 命名规范）`,
                    vscode.DiagnosticSeverity.Information
                );
                diagnostic.code = 'class-name-convention';
                diagnostics.push(diagnostic);
            }
        }
        
        // 检查方法声明语法
        const methodPattern = /\b(public|private|protected|static|final|abstract|native)?\s*(static)?\s*(final)?\s*(int|long|float|double|bool|string|char|void)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/;
        const methodMatch = methodPattern.exec(lineWithoutComment);
        if (methodMatch && !lineWithoutComment.includes('class')) {
            this.currentContext.inMethod = true;
            this.currentContext.methodName = methodMatch[5];
            
            // 检查是否是 main 方法
            if (methodMatch[5] === 'main' && methodMatch[4] === 'void') {
                this.currentContext.hasMainMethod = true;
            }
            
            // 检查方法名是否以小写字母开头（Java 命名规范）
            if (/^[A-Z]/.test(methodMatch[5])) {
                const methodIndex = line.indexOf(methodMatch[5]);
                const range = new vscode.Range(lineNumber, methodIndex, lineNumber, methodIndex + methodMatch[5].length);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    `方法名 '${methodMatch[5]}' 建议以小写字母开头（遵循 camelCase 命名规范）`,
                    vscode.DiagnosticSeverity.Information
                );
                diagnostic.code = 'method-name-convention';
                diagnostics.push(diagnostic);
            }
            
            // 检查方法体是否存在
            if (!lineWithoutComment.includes('{')) {
                // 检查下一行是否有 {
                if (!line.includes('{')) {
                    const range = new vscode.Range(lineNumber, line.length - 1, lineNumber, line.length);
                    const diagnostic = new vscode.Diagnostic(
                        range,
                        `方法 '${methodMatch[5]}' 可能缺少方法体起始符号 '{'`,
                        vscode.DiagnosticSeverity.Warning
                    );
                    diagnostic.code = 'missing-method-body';
                    diagnostics.push(diagnostic);
                }
            }
        }
        
        // 检查变量声明语法
        const varPattern = /\b(int|long|float|double|bool|string|char)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:=|;)/;
        const varMatch = varPattern.exec(lineWithoutComment);
        if (varMatch) {
            // 检查变量名是否以大写字母开头
            if (/^[A-Z]/.test(varMatch[2])) {
                const varIndex = line.indexOf(varMatch[2]);
                const range = new vscode.Range(lineNumber, varIndex, lineNumber, varIndex + varMatch[2].length);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    `变量名 '${varMatch[2]}' 建议以小写字母开头（遵循 camelCase 命名规范）`,
                    vscode.DiagnosticSeverity.Information
                );
                diagnostic.code = 'variable-name-convention';
                diagnostics.push(diagnostic);
            }
        }
        
        // 检查变量未声明就使用
        const undefinedVarPattern = /\b([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:=|\+\+|--)/;
        const undefinedMatch = undefinedVarPattern.exec(lineWithoutComment);
        if (undefinedMatch) {
            const varName = undefinedMatch[1];
            // 检查是否是关键字或已知类型
            const keywords = ['if', 'for', 'while', 'switch', 'case', 'return', 'new', 'true', 'false', 'null'];
            const types = ['int', 'long', 'float', 'double', 'bool', 'char', 'string', 'void'];
            if (!keywords.includes(varName) && !types.includes(varName)) {
                // 检查是否有声明
                const varDeclarationPattern = new RegExp(`\\b(int|long|float|double|bool|string|char)\\s+${varName}\\b`);
                const text = line; // 简化处理，实际应该检查整个文档
                if (!varDeclarationPattern.test(text) && !lineWithoutComment.includes(varName + ' =')) {
                    // 可能是未声明的变量（警告级别，因为可能只是赋值）
                }
            }
        }
        
        // 检查字符串引号匹配（先移除转义序列再检查）
        // 步骤1: 移除所有转义的引号，避免误判
        const lineWithEscapesRemoved = lineWithoutComment
            .replace(/\\"/g, '\x00')  // 将 \" 替换为占位符
            .replace(/\\'/g, '\x01');  // 将 \' 替换为占位符
        
        // 步骤2: 检查双引号是否成对
        const doubleQuotes = (lineWithEscapesRemoved.match(/"/g) || []).length;
        if (doubleQuotes % 2 !== 0) {
            // 找到最后一个未匹配的引号位置（在原始行中）
            let quoteCount = 0;
            let lastQuoteIndex = -1;
            for (let i = 0; i < lineWithoutComment.length; i++) {
                const char = lineWithoutComment[i];
                if (char === '"' && (i === 0 || lineWithoutComment[i-1] !== '\\')) {
                    quoteCount++;
                    lastQuoteIndex = i;
                }
            }
            if (quoteCount % 2 !== 0 && lastQuoteIndex !== -1) {
                const range = new vscode.Range(lineNumber, lastQuoteIndex, lineNumber, line.length);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    '字符串引号未闭合',
                    vscode.DiagnosticSeverity.Error
                );
                diagnostic.code = 'unclosed-string';
                diagnostics.push(diagnostic);
            }
        }
        
        // 步骤3: 检查单引号是否成对（同样移除转义序列）
        const singleQuotes = (lineWithEscapesRemoved.match(/'/g) || []).length;
        if (singleQuotes % 2 !== 0) {
            // 找到最后一个未匹配的引号位置（在原始行中）
            let quoteCount = 0;
            let lastQuoteIndex = -1;
            for (let i = 0; i < lineWithoutComment.length; i++) {
                const char = lineWithoutComment[i];
                if (char === "'" && (i === 0 || lineWithoutComment[i-1] !== '\\')) {
                    quoteCount++;
                    lastQuoteIndex = i;
                }
            }
            if (quoteCount % 2 !== 0 && lastQuoteIndex !== -1) {
                const range = new vscode.Range(lineNumber, lastQuoteIndex, lineNumber, line.length);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    '字符引号未闭合',
                    vscode.DiagnosticSeverity.Error
                );
                diagnostic.code = 'unclosed-char';
                diagnostics.push(diagnostic);
            }
        }
        
        // 检查 break/continue 是否在循环内
        if (/\bbreak\b/.test(lineWithoutComment) && !this.isInLoop(lineNumber)) {
            // 简化检查，实际需要跟踪循环上下文
        }
        if (/\bcontinue\b/.test(lineWithoutComment) && !this.isInLoop(lineNumber)) {
            // 简化检查
        }
        
        // 检查 return 语句
        if (/\breturn\b/.test(lineWithoutComment)) {
            if (!this.currentContext.inMethod) {
                const returnIndex = line.indexOf('return');
                const range = new vscode.Range(lineNumber, returnIndex, lineNumber, returnIndex + 6);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    'return 语句应在方法体内使用',
                    vscode.DiagnosticSeverity.Error
                );
                diagnostic.code = 'return-outside-method';
                diagnostics.push(diagnostic);
            }
        }
        
        // 检查空语句（连续分号）
        if (/;;/.test(lineWithoutComment)) {
            const range = new vscode.Range(lineNumber, line.indexOf(';;'), lineNumber, line.indexOf(';;') + 2);
            const diagnostic = new vscode.Diagnostic(
                range,
                '发现空语句（连续分号）',
                vscode.DiagnosticSeverity.Warning
            );
            diagnostic.code = 'empty-statement';
            diagnostics.push(diagnostic);
        }
        
        // 检查死代码（return 后的代码）
        if (this.hasReturnOnLine(lineNumber - 1) && lineWithoutComment && !lineWithoutComment.startsWith('}')) {
            const range = new vscode.Range(lineNumber, 0, lineNumber, line.length);
            const diagnostic = new vscode.Diagnostic(
                range,
                'return 语句后的代码不可达（死代码）',
                vscode.DiagnosticSeverity.Warning
            );
            diagnostic.code = 'unreachable-code';
            diagnostics.push(diagnostic);
        }
        
        // 检查语句结束符
        if (!lineWithoutComment.endsWith('{') &&
            !lineWithoutComment.endsWith('}') &&
            !lineWithoutComment.endsWith(')') &&
            !lineWithoutComment.endsWith(';') &&
            !lineWithoutComment.endsWith(':') &&
            !trimmedLine.startsWith('/*') &&
            !trimmedLine.startsWith('*') &&
            !trimmedLine.startsWith('import') &&
            !trimmedLine.startsWith('package') &&
            !trimmedLine.startsWith('@') &&
            !trimmedLine.startsWith('#') &&  // 排除预处理器指令
            lineWithoutComment.length > 0) {
            // 这是一个可能的错误，但不是所有情况都需要分号
            // 例如：if/for/while/switch 语句后面不需要分号
            // 例如：case/default 标签以冒号结尾
            const controlFlowPattern = /\b(if|for|while|switch|do)\s*[{(]/;
            const casePattern = /\b(case\s+.+|default)\s*:/;
            const elsePattern = /\belse\b/;
            if (!controlFlowPattern.test(lineWithoutComment) &&
                !casePattern.test(lineWithoutComment) &&
                !elsePattern.test(lineWithoutComment)) {
                const range = new vscode.Range(lineNumber, Math.max(0, line.length - 1), lineNumber, line.length);
                const diagnostic = new vscode.Diagnostic(
                    range,
                    '语句可能缺少分号结束符',
                    vscode.DiagnosticSeverity.Warning
                );
                diagnostic.code = 'missing-semicolon';
                diagnostics.push(diagnostic);
            }
        }
        
        return diagnostics;
    }
    
    /**
     * 检查是否在循环内（简化实现）
     */
    private isInLoop(lineNumber: number): boolean {
        // 实际实现需要跟踪整个文档的上下文
        return true; // 简化处理
    }
    
    /**
     * 检查前一行是否有 return 语句
     */
    private hasReturnOnLine(lineNumber: number): boolean {
        // 简化实现
        return false;
    }

    /**
     * 运行编译器检查
     * @param document 文档
     * @param compilerPath 编译器路径
     * @returns 诊断数组
     */
    private async runCompilerCheck(
        document: vscode.TextDocument,
        compilerPath: string
    ): Promise<vscode.Diagnostic[]> {
        const diagnostics: vscode.Diagnostic[] = [];
        
        try {
            // 使用 eol-check 检查语法
            const { stdout, stderr } = await execAsync(
                `"${compilerPath}" --check "${document.fileName}"`,
                { timeout: 30000 }
            );
            
            // 解析编译器输出
            const output = stdout || stderr;
            if (output) {
                const compilerDiagnostics = this.parseCompilerOutput(output, document);
                diagnostics.push(...compilerDiagnostics);
            }
        } catch (error: any) {
            // 编译器返回非零退出码表示有错误
            if (error.stdout || error.stderr) {
                const output = error.stdout || error.stderr;
                const compilerDiagnostics = this.parseCompilerOutput(output, document);
                diagnostics.push(...compilerDiagnostics);
            }
        }
        
        return diagnostics;
    }

    /**
     * 解析编译器输出
     * @param output 编译器输出
     * @param document 文档
     * @returns 诊断数组
     */
    private parseCompilerOutput(output: string, document: vscode.TextDocument): vscode.Diagnostic[] {
        const diagnostics: vscode.Diagnostic[] = [];
        const lines = output.split('\n');
        
        // 匹配常见的错误格式: file.cay:10:5: error: message
        const errorPattern = /(.+?):(\d+):(\d+):\s*(error|warning|note):\s*(.+)/i;
        
        for (const line of lines) {
            const match = errorPattern.exec(line);
            if (match) {
                const [, filePath, lineStr, colStr, severity, message] = match;
                const lineNum = parseInt(lineStr, 10) - 1; // 转换为 0-based
                const colNum = parseInt(colStr, 10) - 1;
                
                // 只处理当前文档的错误
                if (document.fileName.includes(filePath) || filePath.includes(path.basename(document.fileName))) {
                    const range = new vscode.Range(
                        lineNum,
                        colNum,
                        lineNum,
                        colNum + 1
                    );
                    
                    const diagnosticSeverity = this.parseSeverity(severity);
                    const diagnostic = new vscode.Diagnostic(range, message.trim(), diagnosticSeverity);
                    diagnostic.code = 'compiler-error';
                    diagnostics.push(diagnostic);
                }
            }
        }
        
        return diagnostics;
    }

    /**
     * 解析严重级别
     * @param severity 严重级别字符串
     * @returns DiagnosticSeverity
     */
    private parseSeverity(severity: string): vscode.DiagnosticSeverity {
        switch (severity.toLowerCase()) {
            case 'error':
                return vscode.DiagnosticSeverity.Error;
            case 'warning':
                return vscode.DiagnosticSeverity.Warning;
            case 'note':
            case 'info':
                return vscode.DiagnosticSeverity.Information;
            default:
                return vscode.DiagnosticSeverity.Error;
        }
    }

    /**
     * 配置变更时的处理
     */
    onConfigurationChanged(): void {
        this.config = vscode.workspace.getConfiguration('cavvyAnalyzer');
        
        // 重新检查所有打开的文档
        vscode.workspace.textDocuments.forEach((doc) => {
            if (doc.languageId === 'cavvy') {
                this.scheduleCheck(doc);
            }
        });
    }

    /**
     * 释放资源
     */
    dispose(): void {
        if (this.timeout) {
            clearTimeout(this.timeout);
        }
        this.diagnosticCollection.dispose();
        this.disposables.forEach(d => d.dispose());
    }
}
