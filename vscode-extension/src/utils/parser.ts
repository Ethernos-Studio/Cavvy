import * as vscode from 'vscode';

/**
 * 符号类型
 */
export type SymbolType = 
    | 'class' 
    | 'method' 
    | 'field' 
    | 'variable' 
    | 'parameter' 
    | 'reference';

/**
 * 符号信息接口
 */
export interface SymbolInfo {
    name: string;
    type: SymbolType;
    range: vscode.Range;
    selectionRange: vscode.Range;
    detail?: string;
    parent?: string;
}

/**
 * Cavvy 语言解析器
 * 用于解析文档中的符号（类、方法、字段、变量等）
 */
export class CavvyParser {
    
    // 正则表达式模式
    private readonly patterns = {
        // 类声明: public class MyClass : ParentClass
        classDeclaration: /^(?:\s*(?:public|private|protected|abstract|final)\s+)*class\s+([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*))?/,
        
        // 方法声明: public static int methodName(params)
        methodDeclaration: /^(?:\s*(?:public|private|protected|static|final|abstract|native)\s+)*(?:(int|long|float|double|bool|string|char|void)|([a-zA-Z_][a-zA-Z0-9_]*))\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/,
        
        // 字段声明: private static int fieldName = value;
        fieldDeclaration: /^(?:\s*(?:public|private|protected|static|final)\s+)*(int|long|float|double|bool|string|char)\s+([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*=\s*[^;]+)?\s*;/,
        
        // 变量声明: int varName = value;
        variableDeclaration: /^(?:\s*final\s+)?(int|long|float|double|bool|string|char)\s+([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*=\s*[^;]+)?\s*;/,
        
        // 参数: Type name
        parameter: /(?:int|long|float|double|bool|string|char)\s+([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*,\s*|\s*\))/g,
        
        // 标识符引用
        identifier: /\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g,
        
        // 注解
        annotation: /^\s*@([a-zA-Z_][a-zA-Z0-9_]*)/,
        
        // 注释
        comment: /^(?:\/\/|\/\*)/,
        
        // 字符串
        string: /["']/, 
    };

    /**
     * 解析文档中的所有符号
     * @param document 要解析的文档
     * @returns 符号信息数组
     */
    async parseDocument(document: vscode.TextDocument): Promise<SymbolInfo[]> {
        const symbols: SymbolInfo[] = [];
        const text = document.getText();
        const lines = text.split('\n');
        
        let currentClass: string | undefined;
        let currentMethod: string | undefined;
        let inBlockComment = false;
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const lineStart = document.positionAt(
                lines.slice(0, i).join('\n').length + (i > 0 ? 1 : 0)
            );
            
            // 处理块注释
            if (inBlockComment) {
                if (line.includes('*/')) {
                    inBlockComment = false;
                }
                continue;
            }
            
            if (line.includes('/*')) {
                if (!line.includes('*/')) {
                    inBlockComment = true;
                }
                continue;
            }
            
            // 跳过行注释
            if (this.patterns.comment.test(line.trim())) {
                continue;
            }

            // 解析类声明
            const classMatch = this.patterns.classDeclaration.exec(line);
            if (classMatch) {
                const className = classMatch[1];
                const classIndex = line.indexOf(className);
                const startPos = new vscode.Position(i, classIndex);
                const endPos = new vscode.Position(i, classIndex + className.length);
                
                symbols.push({
                    name: className,
                    type: 'class',
                    range: new vscode.Range(startPos, endPos),
                    selectionRange: new vscode.Range(startPos, endPos),
                    detail: classMatch[2] ? `extends ${classMatch[2]}` : undefined
                });
                
                currentClass = className;
                currentMethod = undefined;
                continue;
            }

            // 解析方法声明
            const methodMatch = this.patterns.methodDeclaration.exec(line);
            if (methodMatch) {
                const returnType = methodMatch[1] || methodMatch[2];
                const methodName = methodMatch[3];
                const methodIndex = line.indexOf(methodName);
                const startPos = new vscode.Position(i, methodIndex);
                const endPos = new vscode.Position(i, methodIndex + methodName.length);
                
                symbols.push({
                    name: methodName,
                    type: 'method',
                    range: new vscode.Range(startPos, endPos),
                    selectionRange: new vscode.Range(startPos, endPos),
                    detail: `() -> ${returnType}`,
                    parent: currentClass
                });
                
                currentMethod = methodName;
                
                // 解析方法参数
                const paramStart = line.indexOf('(');
                const paramEnd = line.indexOf(')');
                if (paramStart !== -1 && paramEnd !== -1) {
                    const paramSection = line.substring(paramStart, paramEnd + 1);
                    let paramMatch;
                    while ((paramMatch = this.patterns.parameter.exec(paramSection)) !== null) {
                        const paramName = paramMatch[1];
                        const paramIndex = paramSection.indexOf(paramName);
                        const paramStartPos = new vscode.Position(i, paramStart + paramIndex);
                        const paramEndPos = new vscode.Position(i, paramStart + paramIndex + paramName.length);
                        
                        symbols.push({
                            name: paramName,
                            type: 'parameter',
                            range: new vscode.Range(paramStartPos, paramEndPos),
                            selectionRange: new vscode.Range(paramStartPos, paramEndPos),
                            parent: currentMethod
                        });
                    }
                }
                continue;
            }

            // 解析字段声明
            const fieldMatch = this.patterns.fieldDeclaration.exec(line);
            if (fieldMatch && currentClass && !currentMethod) {
                const fieldType = fieldMatch[1];
                const fieldName = fieldMatch[2];
                const fieldIndex = line.indexOf(fieldName);
                const startPos = new vscode.Position(i, fieldIndex);
                const endPos = new vscode.Position(i, fieldIndex + fieldName.length);
                
                symbols.push({
                    name: fieldName,
                    type: 'field',
                    range: new vscode.Range(startPos, endPos),
                    selectionRange: new vscode.Range(startPos, endPos),
                    detail: fieldType,
                    parent: currentClass
                });
                continue;
            }

            // 解析变量声明
            const varMatch = this.patterns.variableDeclaration.exec(line);
            if (varMatch && currentMethod) {
                const varType = varMatch[1];
                const varName = varMatch[2];
                const varIndex = line.indexOf(varName);
                const startPos = new vscode.Position(i, varIndex);
                const endPos = new vscode.Position(i, varIndex + varName.length);
                
                symbols.push({
                    name: varName,
                    type: 'variable',
                    range: new vscode.Range(startPos, endPos),
                    selectionRange: new vscode.Range(startPos, endPos),
                    detail: varType,
                    parent: currentMethod
                });
                continue;
            }
        }
        
        return symbols;
    }

    /**
     * 获取指定位置的符号
     * @param document 文档
     * @param position 位置
     * @returns 符号信息或 undefined
     */
    async getSymbolAtPosition(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<SymbolInfo | undefined> {
        const symbols = await this.parseDocument(document);
        
        for (const symbol of symbols) {
            if (symbol.range.contains(position)) {
                return symbol;
            }
        }
        
        return undefined;
    }

    /**
     * 获取所有类定义
     * @param document 文档
     * @returns 类符号数组
     */
    async getClasses(document: vscode.TextDocument): Promise<SymbolInfo[]> {
        const symbols = await this.parseDocument(document);
        return symbols.filter(s => s.type === 'class');
    }

    /**
     * 获取所有方法定义
     * @param document 文档
     * @returns 方法符号数组
     */
    async getMethods(document: vscode.TextDocument): Promise<SymbolInfo[]> {
        const symbols = await this.parseDocument(document);
        return symbols.filter(s => s.type === 'method');
    }

    /**
     * 获取指定类中的所有方法
     * @param document 文档
     * @param className 类名
     * @returns 方法符号数组
     */
    async getMethodsOfClass(
        document: vscode.TextDocument,
        className: string
    ): Promise<SymbolInfo[]> {
        const symbols = await this.parseDocument(document);
        return symbols.filter(s => s.type === 'method' && s.parent === className);
    }
}
