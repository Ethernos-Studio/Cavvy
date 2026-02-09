import * as vscode from 'vscode';
import { CavvyParser, SymbolInfo, SymbolType } from '../utils/parser';

/**
 * 查找引用提供器
 * 支持查找所有引用位置 (Shift+F12)
 */
export class CavvyReferenceProvider implements vscode.ReferenceProvider {
    
    private parser: CavvyParser;

    constructor() {
        this.parser = new CavvyParser();
    }

    /**
     * 提供引用位置
     * @param document 当前文档
     * @param position 光标位置
     * @param context 引用上下文
     * @param token 取消令牌
     * @returns 位置数组
     */
    async provideReferences(
        document: vscode.TextDocument,
        position: vscode.Position,
        context: vscode.ReferenceContext,
        token: vscode.CancellationToken
    ): Promise<vscode.Location[] | undefined> {
        
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return undefined;
        }

        const word = document.getText(wordRange);
        const symbols = await this.parser.parseDocument(document);
        
        // 查找所有引用（包括定义本身和引用位置）
        const references = this.findReferences(word, symbols, document);
        
        if (references.length === 0) {
            return undefined;
        }

        return references.map(ref => new vscode.Location(document.uri, ref.range));
    }

    /**
     * 查找符号的所有引用
     * @param name 符号名称
     * @param symbols 所有符号
     * @param document 文档
     * @returns 符号信息数组
     */
    private findReferences(
        name: string,
        symbols: SymbolInfo[],
        document: vscode.TextDocument
    ): SymbolInfo[] {
        const results: SymbolInfo[] = [];
        
        // 首先查找定义
        const definitions = symbols.filter(s => s.name === name && this.isDefinition(s));
        results.push(...definitions);
        
        // 然后在文档中查找所有引用位置
        const text = document.getText();
        const lines = text.split('\n');
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            let matchIndex = 0;
            
            // 查找所有匹配的标识符
            while ((matchIndex = line.indexOf(name, matchIndex)) !== -1) {
                // 检查是否为完整的单词（避免子字符串匹配）
                const beforeChar = matchIndex > 0 ? line[matchIndex - 1] : '';
                const afterChar = matchIndex + name.length < line.length ? line[matchIndex + name.length] : '';
                
                const isWordBefore = /[a-zA-Z0-9_]/.test(beforeChar);
                const isWordAfter = /[a-zA-Z0-9_]/.test(afterChar);
                
                if (!isWordBefore && !isWordAfter) {
                    const startPos = new vscode.Position(i, matchIndex);
                    const endPos = new vscode.Position(i, matchIndex + name.length);
                    const range = new vscode.Range(startPos, endPos);
                    
                    // 检查这个位置是否已经是定义
                    const isDefinition = definitions.some(def => 
                        def.range.start.line === i && 
                        def.range.start.character === matchIndex
                    );
                    
                    if (!isDefinition) {
                        results.push({
                            name: name,
                            type: 'reference',
                            range: range,
                            selectionRange: range
                        });
                    }
                }
                
                matchIndex += name.length;
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
