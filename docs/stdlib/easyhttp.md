# EasyHTTP - HTTP客户端标准库

## 版本信息

- **版本**: 0.5.1.0
- **文件**: `caylibs/EasyHTTP.cay`
- **依赖**: `Network.cay`, `StringBuilder.cay`

## 概述

EasyHTTP 提供了简洁易用的 HTTP 客户端功能，支持链式 API 设计、自动 URL 编码、自定义请求头、JSON 和表单数据处理。基于 Network 模块构建，支持完整的 HTTP/1.1 协议。

**核心特性**:
- 链式 API 设计，代码简洁优雅
- 自动 URL 编码处理
- 支持自定义请求头
- 支持 JSON 和表单数据
- 连接超时控制
- 完整的错误处理

## 常量定义

### HTTP 方法

```cay
#define HTTP_GET     "GET"
#define HTTP_POST    "POST"
#define HTTP_PUT     "PUT"
#define HTTP_DELETE  "DELETE"
#define HTTP_HEAD    "HEAD"
#define HTTP_OPTIONS "OPTIONS"
#define HTTP_PATCH   "PATCH"
```

### Content-Type

```cay
#define CONTENT_TYPE_JSON       "application/json"
#define CONTENT_TYPE_FORM       "application/x-www-form-urlencoded"
#define CONTENT_TYPE_TEXT       "text/plain"
#define CONTENT_TYPE_HTML       "text/html"
#define CONTENT_TYPE_XML        "application/xml"
#define CONTENT_TYPE_MULTIPART  "multipart/form-data"
```

### 常用请求头

```cay
#define HEADER_ACCEPT           "Accept"
#define HEADER_CONTENT_TYPE     "Content-Type"
#define HEADER_CONTENT_LENGTH   "Content-Length"
#define HEADER_USER_AGENT       "User-Agent"
#define HEADER_AUTHORIZATION    "Authorization"
#define HEADER_HOST             "Host"
#define HEADER_CONNECTION       "Connection"
```

### 默认超时设置

```cay
#define DEFAULT_CONNECT_TIMEOUT 5000     // 连接超时：5秒
#define DEFAULT_READ_TIMEOUT    10000    // 读取超时：10秒
#define MAX_REDIRECTS           5        // 最大重定向次数
```

## 核心类

### HttpHeaders - HTTP头部管理

用于存储和管理 HTTP 请求/响应头部。

**时间复杂度**: O(n) get/set, O(1) add
**空间复杂度**: O(n) n=头部数量

```cay
// 创建头部管理器
HttpHeaders headers = new HttpHeaders();

// 添加/更新头部（支持链式调用）
headers.set("Content-Type", "application/json")
       .set("Authorization", "Bearer token123");

// 添加头部（允许重复）
headers.add("Accept", "application/json")
       .add("Accept", "text/html");

// 获取头部值
String contentType = headers.get("Content-Type");
String[] accepts = headers.getAll("Accept");

// 移除头部
headers.remove("X-Custom-Header");

// 检查头部是否存在
bool hasAuth = headers.contains("Authorization");

// 获取数量
int count = headers.size();

// 清空
headers.clear();

// 构建HTTP头部字符串
String headerStr = headers.build();
// 输出: Content-Type: application/json\r\nAuthorization: Bearer token123\r\n
```

### HttpParams - URL参数管理

用于构建 URL 查询字符串。

**时间复杂度**: O(n) build
**空间复杂度**: O(n)

```cay
// 创建参数管理器
HttpParams params = new HttpParams();

// 添加参数（支持链式调用）
params.add("name", "Alice")
      .add("age", 25)
      .add("active", true);

// 构建查询字符串（自动URL编码）
String query = params.build();
// 输出: name=Alice&age=25&active=true

// 检查是否为空
bool empty = params.isEmpty();

// 获取数量
int count = params.size();

// 清空
params.clear();
```

### HttpResponse - HTTP响应封装

封装 HTTP 响应的所有信息。

**时间复杂度**: O(1) 大多数操作
**空间复杂度**: O(n) n=响应体大小

```cay
// 创建响应对象（通常由请求自动创建）
HttpResponse response = new HttpResponse();

// 获取状态信息
int statusCode = response.getStatusCode();      // 200
String statusText = response.getStatusText();   // "OK"
bool success = response.isSuccess();            // true (200-299)

// 获取响应体
String body = response.getBody();

// 获取响应头
HttpHeaders headers = response.getHeaders();
String contentType = headers.get("Content-Type");

// 检查是否为JSON响应
bool isJson = response.isJson();

// 获取响应时间（毫秒）
long responseTime = response.getResponseTime();

// 获取错误信息
String error = response.getError();

// 字符串表示
String str = response.toString();
```

### HttpRequest - HTTP请求构建器

用于构建和发送 HTTP 请求，支持流畅 API 设计。

**时间复杂度**: O(n) build, O(m) send m=响应大小
**空间复杂度**: O(n + m)

```cay
// 创建请求构建器
HttpRequest request = new HttpRequest();

// 设置请求方法
request.setMethod(HTTP_POST);

// 设置URL
request.setUrl("https://api.example.com/users");

// 设置请求头
request.setHeader("X-API-Key", "secret123")
       .setContentType(CONTENT_TYPE_JSON)
       .setAccept("application/json")
       .setAuthorization("Basic dXNlcjpwYXNz");

// 设置Bearer Token认证
request.setBearerToken("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...");

// 添加URL参数
request.addParam("page", 1)
       .addParam("limit", 10);

// 设置请求体
request.setBody("{\"name\":\"Alice\"}");
request.setJsonBody("{\"name\":\"Alice\"}");
request.setFormBody(params);

// 设置超时
request.setConnectTimeout(5000)   // 5秒
       .setReadTimeout(10000);    // 10秒

// 设置重定向
request.setFollowRedirects(true)
       .setMaxRedirects(5);

// 发送请求
HttpResponse response = request.send();
```

### EasyHTTP - 静态工具类

提供便捷的静态 HTTP 请求方法，无需实例化。

```cay
// GET请求
HttpResponse response = EasyHTTP.get("https://api.example.com/users");

// GET请求（带参数）
HttpParams params = new HttpParams().add("page", 1).add("limit", 10);
HttpResponse response = EasyHTTP.get("https://api.example.com/users", params);

// POST请求
HttpResponse response = EasyHTTP.post("https://api.example.com/users", "name=Alice");

// POST JSON
HttpResponse response = EasyHTTP.postJson("https://api.example.com/users", 
    "{\"name\":\"Alice\",\"age\":25}");

// POST表单
HttpParams formData = new HttpParams().add("name", "Alice").add("age", "25");
HttpResponse response = EasyHTTP.postForm("https://api.example.com/users", formData);

// PUT请求
HttpResponse response = EasyHTTP.put("https://api.example.com/users/1", "name=Bob");
HttpResponse response = EasyHTTP.putJson("https://api.example.com/users/1", 
    "{\"name\":\"Bob\"}");

// DELETE请求
HttpResponse response = EasyHTTP.delete("https://api.example.com/users/1");
HttpResponse response = EasyHTTP.delete("https://api.example.com/users/1", "reason=test");

// HEAD请求
HttpResponse response = EasyHTTP.head("https://api.example.com/users");

// 创建构建器
HttpRequest request = EasyHTTP.request();
HttpParams params = EasyHTTP.params();
HttpHeaders headers = EasyHTTP.headers();
```

## 使用示例

### 基本GET请求

```cay
#include <EasyHTTP.cay>

public class Main {
    public static void main(String[] args) {
        // 发送GET请求
        HttpResponse response = EasyHTTP.get("https://api.github.com/users/github");
        
        if (response.isSuccess()) {
            println("状态码: " + response.getStatusCode());
            println("响应体: " + response.getBody());
        } else {
            println("请求失败: " + response.getError());
        }
    }
}
```

### POST JSON数据

```cay
#include <EasyHTTP.cay>

public class Main {
    public static void main(String[] args) {
        // 构建JSON数据
        String json = "{\"name\":\"Alice\",\"email\":\"alice@example.com\",\"age\":25}";
        
        // 发送POST请求
        HttpResponse response = EasyHTTP.postJson("https://api.example.com/users", json);
        
        if (response.isSuccess()) {
            println("创建成功: " + response.getBody());
        } else {
            println("创建失败: " + response.getStatusCode());
        }
    }
}
```

### 使用请求构建器

```cay
#include <EasyHTTP.cay>

public class Main {
    public static void main(String[] args) {
        // 构建复杂请求
        HttpResponse response = EasyHTTP.request()
            .setMethod(HTTP_POST)
            .setUrl("https://api.example.com/search")
            .setHeader("X-API-Key", "your-api-key")
            .setBearerToken("your-access-token")
            .setContentType(CONTENT_TYPE_JSON)
            .addParam("page", 1)
            .addParam("limit", 20)
            .setJsonBody("{\"query\":\"cavvy\",\"filters\":{\"lang\":\"cay\"}}")
            .setConnectTimeout(10000)
            .setReadTimeout(30000)
            .send();
        
        if (response.isSuccess()) {
            println("搜索结果: " + response.getBody());
        }
    }
}
```

### 表单提交

```cay
#include <EasyHTTP.cay>

public class Main {
    public static void main(String[] args) {
        // 构建表单参数
        HttpParams params = EasyHTTP.params()
            .add("username", "alice")
            .add("password", "secret123")
            .add("remember", true);
        
        // 发送表单POST请求
        HttpResponse response = EasyHTTP.postForm("https://api.example.com/login", params);
        
        if (response.isSuccess()) {
            println("登录成功");
        } else {
            println("登录失败: " + response.getBody());
        }
    }
}
```

### 文件下载（简单实现）

```cay
#include <EasyHTTP.cay>
#include <File.cay>

public class Main {
    public static void main(String[] args) {
        // 下载文件
        HttpResponse response = EasyHTTP.get("https://example.com/file.txt");
        
        if (response.isSuccess()) {
            // 保存到文件
            File.writeAllText("downloaded.txt", response.getBody());
            println("下载完成");
        }
    }
}
```

### RESTful API 完整示例

```cay
#include <EasyHTTP.cay>

public class ApiClient {
    private String baseUrl;
    private String apiKey;
    
    public ApiClient(String baseUrl, String apiKey) {
        this.baseUrl = baseUrl;
        this.apiKey = apiKey;
    }
    
    // 获取资源列表
    public HttpResponse list(String resource) {
        return EasyHTTP.get(this.baseUrl + "/" + resource);
    }
    
    // 获取单个资源
    public HttpResponse get(String resource, int id) {
        return EasyHTTP.get(this.baseUrl + "/" + resource + "/" + id);
    }
    
    // 创建资源
    public HttpResponse create(String resource, String json) {
        return EasyHTTP.request()
            .setMethod(HTTP_POST)
            .setUrl(this.baseUrl + "/" + resource)
            .setHeader("X-API-Key", this.apiKey)
            .setJsonBody(json)
            .send();
    }
    
    // 更新资源
    public HttpResponse update(String resource, int id, String json) {
        return EasyHTTP.putJson(this.baseUrl + "/" + resource + "/" + id, json);
    }
    
    // 删除资源
    public HttpResponse delete(String resource, int id) {
        return EasyHTTP.delete(this.baseUrl + "/" + resource + "/" + id);
    }
}

public class Main {
    public static void main(String[] args) {
        ApiClient client = new ApiClient("https://api.example.com", "my-api-key");
        
        // 列出所有用户
        HttpResponse users = client.list("users");
        println("用户列表: " + users.getBody());
        
        // 创建用户
        HttpResponse created = client.create("users", "{\"name\":\"Alice\"}");
        println("创建结果: " + created.getBody());
    }
}
```

## 性能优化

### 时间复杂度

| 操作 | 时间复杂度 | 说明 |
|------|-----------|------|
| 构建请求 | O(n) | n=请求大小 |
| 发送请求 | O(n+m) | n=请求大小, m=响应大小 |
| URL编码 | O(n) | n=字符串长度 |
| 头部操作 | O(1) amortized | 动态数组 |

### 最佳实践

1. **复用连接**: 当前版本每次请求新建连接，生产环境建议使用连接池
2. **超时设置**: 始终设置合理的超时时间
3. **错误处理**: 检查 response.isSuccess() 和 response.getError()
4. **流式处理**: 大文件下载应使用流式API（待实现）

## 注意事项

1. **HTTPS支持**: 当前版本不支持HTTPS，仅支持HTTP
2. **重定向**: 默认自动跟随重定向，可通过 setFollowRedirects 控制
3. **编码**: 自动进行URL编码，无需手动处理
4. **内存**: 响应体完全加载到内存，不适合大文件下载
5. **线程安全**: HttpRequest 不是线程安全的，每个线程应创建独立实例
