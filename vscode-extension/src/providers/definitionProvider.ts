import * as vscode from 'vscode';
import { CavvyParser, SymbolInfo, SymbolType } from '../utils/parser';

/**
 * 跳转到定义提供器
 * 支持 Ctrl+Click 或 F12 跳转到符号定义
 */
export class CavvyDefinitionProvider implements vscode.DefinitionProvider {
    
    private parser: CavvyParser;

    constructor() {
        this.parser = new CavvyParser();
    }

    /**
     * 提供定义位置
     * @param document 当前文档
     * @param position 光标位置
     * @param token 取消令牌
     * @returns 定义位置数组
     */
    async provideDefinition(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): Promise<vscode.Location[] | undefined> {
        
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return undefined;
        }

        const word = document.getText(wordRange);
        const symbols = await this.parser.parseDocument(document);
        
        // 查找符号定义
        const definitions = this.findDefinitions(word, symbols, document);
        
        if (definitions.length === 0) {
            return undefined;
        }

        return definitions.map(def => new vscode.Location(document.uri, def.range));
    }

    /**
     * 查找符号定义
     * @param name 符号名称
     * @param symbols 所有符号
     * @param document 文档
     * @returns 符号信息数组
     */
    private findDefinitions(
        name: string,
        symbols: SymbolInfo[],
        document: vscode.TextDocument
    ): SymbolInfo[] {
        const results: SymbolInfo[] = [];
        
        for (const symbol of symbols) {
            if (symbol.name === name) {
                // 检查是否是定义（而非引用）
                if (this.isDefinition(symbol)) {
                    results.push(symbol);
                }
            }
        }
        
        return results;
    }

    /**
     * 判断符号是否是定义
     * @param symbol 符号信息
     * @returns 是否是定义
     */
    private isDefinition(symbol: SymbolInfo): boolean {
        const definitionTypes: SymbolType[] = [
            'class',
            'method',
            'field',
            'variable',
            'parameter'
        ];
        return definitionTypes.includes(symbol.type);
    }
}
