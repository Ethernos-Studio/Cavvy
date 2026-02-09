import * as vscode from 'vscode';
import { CavvyParser, SymbolInfo, SymbolType } from '../utils/parser';

/**
 * 文档符号提供器
 * 用于在 Outline（大纲）视图中显示文档结构
 */
export class CavvyDocumentSymbolProvider implements vscode.DocumentSymbolProvider {
    
    private parser: CavvyParser;

    constructor() {
        this.parser = new CavvyParser();
    }

    /**
     * 提供文档符号
     * @param document 当前文档
     * @param token 取消令牌
     * @returns 文档符号数组
     */
    async provideDocumentSymbols(
        document: vscode.TextDocument,
        token: vscode.CancellationToken
    ): Promise<vscode.DocumentSymbol[]> {
        
        const symbols = await this.parser.parseDocument(document);
        const documentSymbols: vscode.DocumentSymbol[] = [];
        
        // 按类型组织符号
        const classSymbols = new Map<string, vscode.DocumentSymbol>();
        const methodSymbols = new Map<string, vscode.DocumentSymbol>();
        
        for (const symbol of symbols) {
            const docSymbol = this.createDocumentSymbol(symbol);
            
            switch (symbol.type) {
                case 'class':
                    classSymbols.set(symbol.name, docSymbol);
                    documentSymbols.push(docSymbol);
                    break;
                    
                case 'method':
                    if (symbol.parent && classSymbols.has(symbol.parent)) {
                        // 将方法添加为类的子项
                        const parentClass = classSymbols.get(symbol.parent);
                        if (parentClass) {
                            parentClass.children.push(docSymbol);
                        }
                    } else {
                        documentSymbols.push(docSymbol);
                    }
                    methodSymbols.set(symbol.name, docSymbol);
                    break;
                    
                case 'field':
                    if (symbol.parent && classSymbols.has(symbol.parent)) {
                        const parentClass = classSymbols.get(symbol.parent);
                        if (parentClass) {
                            parentClass.children.push(docSymbol);
                        }
                    } else {
                        documentSymbols.push(docSymbol);
                    }
                    break;
                    
                case 'variable':
                case 'parameter':
                    if (symbol.parent && methodSymbols.has(symbol.parent)) {
                        const parentMethod = methodSymbols.get(symbol.parent);
                        if (parentMethod) {
                            parentMethod.children.push(docSymbol);
                        }
                    } else {
                        documentSymbols.push(docSymbol);
                    }
                    break;
                    
                default:
                    documentSymbols.push(docSymbol);
            }
        }
        
        return documentSymbols;
    }

    /**
     * 创建文档符号
     * @param symbol 符号信息
     * @returns VSCode 文档符号
     */
    private createDocumentSymbol(symbol: SymbolInfo): vscode.DocumentSymbol {
        const kind = this.getSymbolKind(symbol.type);
        const name = symbol.name;
        const detail = symbol.detail || '';
        
        return new vscode.DocumentSymbol(
            name,
            detail,
            kind,
            symbol.range,
            symbol.selectionRange
        );
    }

    /**
     * 获取符号类型对应的 VSCode SymbolKind
     * @param type 符号类型
     * @returns SymbolKind
     */
    private getSymbolKind(type: SymbolType): vscode.SymbolKind {
        switch (type) {
            case 'class':
                return vscode.SymbolKind.Class;
            case 'method':
                return vscode.SymbolKind.Method;
            case 'field':
                return vscode.SymbolKind.Field;
            case 'variable':
                return vscode.SymbolKind.Variable;
            case 'parameter':
                return vscode.SymbolKind.TypeParameter;
            case 'reference':
                return vscode.SymbolKind.Property;
            default:
                return vscode.SymbolKind.Property;
        }
    }
}
