import re

# 读取文件
with open('src/semantic/expr_inference.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# 替换模式：semantic_error(call.loc.line, call.loc.column, ...) -> semantic_error_at_loc(&call.loc, ...)
# 使用正则表达式匹配多行的情况
pattern = r'return Err\(semantic_error\(\s*call\.loc\.line,\s*call\.loc\.column,\s*'
replacement = 'return Err(semantic_error_at_loc(&call.loc, '

content = re.sub(pattern, replacement, content)

# 处理其他形式的semantic_error(call.loc.line, call.loc.column, ...)
pattern2 = r'semantic_error\(\s*call\.loc\.line,\s*call\.loc\.column,'
replacement2 = 'semantic_error_at_loc(&call.loc,'

content = re.sub(pattern2, replacement2, content)

# 写入文件
with open('src/semantic/expr_inference.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print("Done!")
