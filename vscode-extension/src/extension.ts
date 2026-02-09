import * as vscode from 'vscode';
import * as path from 'path';
import { CavvyDefinitionProvider } from './providers/definitionProvider';
import { CavvyDiagnosticProvider } from './providers/diagnosticProvider';
import { CavvyDocumentSymbolProvider } from './providers/documentSymbolProvider';
import { CavvyReferenceProvider } from './providers/referenceProvider';
import { CavvyCompletionProvider } from './providers/completionProvider';
import { CavvyHoverProvider } from './providers/hoverProvider';

/**
 * Cavvy Analyzer 插件主入口
 * 提供语法高亮、跳转定义、语法错误诊断等功能
 */

let diagnosticProvider: CavvyDiagnosticProvider | undefined;

/**
 * 插件激活时调用
 * @param context 插件上下文
 */
export function activate(context: vscode.ExtensionContext): void {
    console.log('Cavvy Analyzer 插件已激活');

    // 注册跳转到定义提供器
    const definitionProvider = vscode.languages.registerDefinitionProvider(
        'cavvy',
        new CavvyDefinitionProvider()
    );
    context.subscriptions.push(definitionProvider);

    // 注册文档符号提供器（用于大纲视图）
    const documentSymbolProvider = vscode.languages.registerDocumentSymbolProvider(
        'cavvy',
        new CavvyDocumentSymbolProvider()
    );
    context.subscriptions.push(documentSymbolProvider);

    // 注册查找引用提供器
    const referenceProvider = vscode.languages.registerReferenceProvider(
        'cavvy',
        new CavvyReferenceProvider()
    );
    context.subscriptions.push(referenceProvider);

    // 注册代码补全提供器
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'cavvy',
        new CavvyCompletionProvider(),
        '.',  // 触发字符：点号
        '('   // 触发字符：左括号
    );
    context.subscriptions.push(completionProvider);

    // 注册 Hover 提供器
    const hoverProvider = vscode.languages.registerHoverProvider(
        'cavvy',
        new CavvyHoverProvider()
    );
    context.subscriptions.push(hoverProvider);

    // 初始化诊断提供器
    diagnosticProvider = new CavvyDiagnosticProvider();
    diagnosticProvider.activate(context);

    // 注册命令：手动检查语法
    const checkSyntaxCommand = vscode.commands.registerCommand(
        'cavvyAnalyzer.checkSyntax',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cavvy') {
                vscode.window.showWarningMessage('请先打开一个 Cavvy 文件');
                return;
            }
            await diagnosticProvider?.checkDocument(editor.document);
            vscode.window.showInformationMessage('语法检查完成');
        }
    );
    context.subscriptions.push(checkSyntaxCommand);

    // 注册命令：跳转到定义（用于右键菜单）
    const gotoDefinitionCommand = vscode.commands.registerCommand(
        'cavvyAnalyzer.gotoDefinition',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'cavvy') {
                return;
            }
            
            const position = editor.selection.active;
            const locations = await new CavvyDefinitionProvider().provideDefinition(
                editor.document,
                position,
                new vscode.CancellationTokenSource().token
            );
            
            if (locations && locations.length > 0) {
                const location = locations[0] as vscode.Location;
                await vscode.window.showTextDocument(location.uri, {
                    selection: location.range
                });
            } else {
                vscode.window.showInformationMessage('未找到定义');
            }
        }
    );
    context.subscriptions.push(gotoDefinitionCommand);

    // 监听文档打开事件，为空文件生成模板
    const onDidOpenDisposable = vscode.workspace.onDidOpenTextDocument(
        (document) => {
            if (document.languageId === 'cavvy' && document.getText().trim().length === 0) {
                generateTemplate(document);
            }
        }
    );
    context.subscriptions.push(onDidOpenDisposable);

    // 检查当前已打开的空文档
    vscode.workspace.textDocuments.forEach((doc) => {
        if (doc.languageId === 'cavvy' && doc.getText().trim().length === 0) {
            generateTemplate(doc);
        }
    });

    // 监听配置变更
    const configChangeDisposable = vscode.workspace.onDidChangeConfiguration(
        (event) => {
            if (event.affectsConfiguration('cavvyAnalyzer')) {
                diagnosticProvider?.onConfigurationChanged();
            }
        }
    );
    context.subscriptions.push(configChangeDisposable);
}

/**
 * 为空的 .cay 文件生成模板代码
 * @param document 文档
 */
async function generateTemplate(document: vscode.TextDocument): Promise<void> {
    // 获取文件名（不含扩展名）
    const fileName = path.basename(document.fileName, '.cay');
    
    // 转换为 PascalCase（大驼峰形式）
    const className = toPascalCase(fileName);
    
    // 生成模板代码
    const template = `@main
public class ${className} {
    public static void main() {
        // 在这里写入你的代码
        
    }
}`;
    
    // 获取编辑器
    const editor = await vscode.window.showTextDocument(document);
    
    // 插入模板
    await editor.edit((editBuilder) => {
        editBuilder.insert(new vscode.Position(0, 0), template);
    });
    
    // 将光标定位到注释后的位置（在 main 方法体内）
    const position = new vscode.Position(4, 8);  // 注释行的下一行，缩进位置
    editor.selection = new vscode.Selection(position, position);
}

/**
 * 将字符串转换为 PascalCase（大驼峰形式）
 * @param str 输入字符串
 * @returns PascalCase 形式的字符串
 */
function toPascalCase(str: string): string {
    // 如果字符串已经是全大写或全小写，首字母大写即可
    if (/^[a-z]+$/.test(str)) {
        return str.charAt(0).toUpperCase() + str.slice(1);
    }
    if (/^[A-Z]+$/.test(str)) {
        return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
    }
    
    // 处理下划线、连字符或空格分隔的字符串
    return str
        .replace(/[-_]/g, ' ')
        .replace(/\s+(.)/g, (_, char) => char.toUpperCase())
        .replace(/^[a-z]/, (char) => char.toUpperCase())
        .replace(/\s+/g, '');
}

/**
 * 插件停用时调用
 */
export function deactivate(): void {
    console.log('Cavvy Analyzer 插件已停用');
    diagnosticProvider?.dispose();
    diagnosticProvider = undefined;
}
