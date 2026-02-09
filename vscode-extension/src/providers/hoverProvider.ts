import * as vscode from 'vscode';

/**
 * Hover 提供器
 * 提供 Cavvy 语言的悬停提示信息
 */
export class CavvyHoverProvider implements vscode.HoverProvider {
    
    // 关键字文档
    private keywordDocs: Map<string, string> = new Map([
        ['public', '**public** - 访问修饰符，表示公开的，任何地方都可以访问。\n\n```cavvy\npublic class MyClass {\n    public int value;\n}\n```'],
        ['private', '**private** - 访问修饰符，表示私有的，只能在类内部访问。\n\n```cavvy\npublic class MyClass {\n    private int secret;\n}\n```'],
        ['protected', '**protected** - 访问修饰符，表示受保护的，可在类内部和子类中访问。'],
        ['static', '**static** - 静态修饰符，表示属于类而不是实例。\n\n```cavvy\npublic static void main() {\n    // 静态方法\n}\n```'],
        ['final', '**final** - 最终修饰符，表示不可修改（常量）。\n\n```cavvy\nfinal int MAX_SIZE = 100;\n```'],
        ['abstract', '**abstract** - 抽象修饰符，用于抽象类和抽象方法。'],
        ['native', '**native** - 本地方法修饰符，表示由外部实现。'],
        ['class', '**class** - 用于声明类。\n\n```cavvy\npublic class MyClass {\n    // 类体\n}\n```'],
        ['void', '**void** - 表示无返回值。\n\n```cavvy\npublic void doSomething() {\n    // 无返回值\n}\n```'],
        ['int', '**int** - 32位整数类型。\n\n范围: -2,147,483,648 到 2,147,483,647\n\n```cavvy\nint count = 10;\n```'],
        ['long', '**long** - 64位整数类型。\n\n范围: -9,223,372,036,854,775,808 到 9,223,372,036,854,775,807\n\n```cavvy\nlong bigNumber = 10000000000L;\n```'],
        ['float', '**float** - 32位单精度浮点数。\n\n```cavvy\nfloat price = 19.99f;\n```'],
        ['double', '**double** - 64位双精度浮点数。\n\n```cavvy\ndouble precise = 3.14159265359;\n```'],
        ['bool', '**bool** - 布尔类型，值为 `true` 或 `false`。\n\n```cavvy\nbool isReady = true;\n```'],
        ['char', '**char** - 16位 Unicode 字符。\n\n```cavvy\nchar letter = \'A\';\n```'],
        ['string', '**string** - 字符串类型。\n\n```cavvy\nstring message = "Hello, World!";\n```'],
        ['if', '**if** - 条件语句。\n\n```cavvy\nif (condition) {\n    // 条件为真时执行\n}\n```'],
        ['else', '**else** - 与 if 配合使用，条件为假时执行。\n\n```cavvy\nif (condition) {\n    // 条件为真\n} else {\n    // 条件为假\n}\n```'],
        ['while', '**while** - 循环语句，条件为真时重复执行。\n\n```cavvy\nwhile (condition) {\n    // 循环体\n}\n```'],
        ['for', '**for** - 循环语句，用于已知次数的循环。\n\n```cavvy\nfor (int i = 0; i < 10; i++) {\n    // 循环体\n}\n```'],
        ['do', '**do** - do-while 循环的开头。\n\n```cavvy\ndo {\n    // 循环体\n} while (condition);\n```'],
        ['switch', '**switch** - 多分支选择语句。\n\n```cavvy\nswitch (value) {\n    case 1:\n        // ...\n        break;\n    default:\n        // ...\n}\n```'],
        ['case', '**case** - switch 语句中的分支标签。\n\n```cavvy\ncase 1:\n    println("One");\n    break;\n```'],
        ['default', '**default** - switch 语句中的默认分支。\n\n```cavvy\ndefault:\n    println("Other");\n    break;\n```'],
        ['break', '**break** - 跳出循环或 switch 语句。\n\n```cavvy\nwhile (true) {\n    if (done) break;\n}\n```'],
        ['continue', '**continue** - 跳过当前循环迭代，继续下一次。\n\n```cavvy\nfor (int i = 0; i < 10; i++) {\n    if (i == 5) continue;\n    println(i);\n}\n```'],
        ['return', '**return** - 从方法返回，可带返回值。\n\n```cavvy\nreturn 42;\nreturn;  // 无返回值\n```'],
        ['new', '**new** - 创建新对象或数组。\n\n```cavvy\nint[] arr = new int[10];\n```'],
        ['null', '**null** - 空引用。\n\n```cavvy\nstring s = null;\n```'],
        ['true', '**true** - 布尔真值。'],
        ['false', '**false** - 布尔假值。'],
        ['import', '**import** - 导入其他包。'],
        ['package', '**package** - 声明包名。'],
        ['this', '**this** - 引用当前对象实例。']
    ]);
    
    // 内置方法文档
    private methodDocs: Map<string, { signature: string; description: string; example: string }> = new Map([
        ['print', {
            signature: 'print(value: any) -> void',
            description: '打印值到控制台，不换行。',
            example: 'print("Hello");\nprint(42);'
        }],
        ['println', {
            signature: 'println(value: any) -> void',
            description: '打印值到控制台，并在末尾添加换行符。',
            example: 'println("Hello, World!");\nprintln(123);'
        }],
        ['length', {
            signature: 'length() -> int',
            description: '返回字符串或数组的长度。',
            example: 'string s = "hello";\nint len = s.length();  // 5\nint[] arr = new int[10];\nint arrLen = arr.length();  // 10'
        }],
        ['charAt', {
            signature: 'charAt(index: int) -> char',
            description: '返回字符串指定位置的字符。索引从 0 开始。',
            example: 'string s = "hello";\nchar c = s.charAt(1);  // \'e\''
        }],
        ['indexOf', {
            signature: 'indexOf(str: string) -> int',
            description: '查找子字符串在字符串中的位置。如果未找到返回 -1。',
            example: 'string s = "hello world";\nint pos = s.indexOf("world");  // 6\nint notFound = s.indexOf("xyz");  // -1'
        }],
        ['substring', {
            signature: 'substring(start: int, end: int) -> string',
            description: '返回从 start（包含）到 end（不包含）的子字符串。',
            example: 'string s = "hello world";\nstring sub = s.substring(0, 5);  // "hello"'
        }],
        ['concat', {
            signature: 'concat(str: string) -> string',
            description: '将指定字符串连接到当前字符串末尾。',
            example: 'string s = "hello";\nstring result = s.concat(" world");  // "hello world"'
        }],
        ['replace', {
            signature: 'replace(old: string, new: string) -> string',
            description: '替换字符串中所有匹配的子字符串。',
            example: 'string s = "hello world";\nstring result = s.replace("world", "Cavvy");  // "hello Cavvy"'
        }],
        ['toString', {
            signature: 'toString() -> string',
            description: '将值转换为字符串表示。',
            example: 'int num = 42;\nstring s = num.toString();  // "42"'
        }]
    ]);

    /**
     * 提供悬停信息
     */
    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.Hover> {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return undefined;
        }
        
        const word = document.getText(wordRange);
        
        // 检查关键字
        if (this.keywordDocs.has(word)) {
            const content = this.keywordDocs.get(word);
            if (content) {
                return new vscode.Hover(new vscode.MarkdownString(content));
            }
        }
        
        // 检查内置方法
        if (this.methodDocs.has(word)) {
            const doc = this.methodDocs.get(word);
            if (doc) {
                const content = new vscode.MarkdownString();
                content.appendCodeblock(doc.signature, 'cavvy');
                content.appendMarkdown(`\n${doc.description}\n\n**示例：**\n`);
                content.appendCodeblock(doc.example, 'cavvy');
                return new vscode.Hover(content);
            }
        }
        
        return undefined;
    }
}
