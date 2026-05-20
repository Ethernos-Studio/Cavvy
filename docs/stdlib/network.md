# Network - 网络编程标准库

## 版本信息

- **版本**: 0.5.1.0
- **文件**: `caylibs/Network.cay`
- **依赖**: 平台特定（Windows: ws2_32, Linux: socket）

## 概述

Network 模块提供了完整的 TCP/UDP 网络编程支持，包括 Socket 操作、地址管理、DNS 解析和简单的 HTTP 功能。支持 Windows 和 Linux 跨平台开发。

**核心特性**:
- TCP/UDP Socket 支持
- 跨平台兼容（Windows/Linux）
- 地址解析和转换
- 网络工具函数
- HTTP 请求/响应构建

## 常量定义

### 地址族

```cay
#define AF_UNSPEC    0    // 未指定
#define AF_INET      2    // IPv4
#define AF_INET6    23    // IPv6
```

### Socket 类型

```cay
#define SOCK_STREAM  1    // TCP
#define SOCK_DGRAM   2    // UDP
#define SOCK_RAW     3    // 原始Socket
```

### Socket 选项级别

```cay
#define SOL_SOCKET  0xFFFF    // Socket级别
```

### Socket 选项

```cay
#define SO_REUSEADDR  0x0004    // 地址重用
#define SO_BROADCAST  0x0020    // 广播
#define SO_SNDBUF     0x1001    // 发送缓冲区大小
#define SO_RCVBUF     0x1002    // 接收缓冲区大小
```

### 协议

```cay
#define IPPROTO_TCP    6    // TCP协议
#define IPPROTO_UDP   17    // UDP协议
```

### 无效Socket

```cay
#define INVALID_SOCKET  -1
```

### Shutdown 选项

```cay
#define SD_RECEIVE  0    // 关闭接收
#define SD_SEND     1    // 关闭发送
#define SD_BOTH     2    // 关闭双向
```

## 核心类

### SocketAddr - 网络地址类

封装 IPv4 地址和端口信息。

```cay
// 创建地址
SocketAddr addr = new SocketAddr().fromString("127.0.0.1", 8080);
SocketAddr addr = new SocketAddr().fromIpPort("192.168.1.1", 80);
SocketAddr addr = new SocketAddr().localhost(3000);    // 127.0.0.1:3000
SocketAddr addr = new SocketAddr().any(8080);          // 0.0.0.0:8080

// 获取信息
int port = addr.getPort();
String ip = addr.getIp();
```

### NetworkUtils - 网络工具类

```cay
// 初始化/清理网络库
bool initialized = NetworkUtils.init();
NetworkUtils.cleanup();

// 获取错误码
int error = NetworkUtils.getLastError();

// 字节序转换
int networkShort = NetworkUtils.htons(8080);    // 主机转网络(16位)
int networkLong = NetworkUtils.htonl(12345);    // 主机转网络(32位)
int hostShort = NetworkUtils.ntohs(networkShort); // 网络转主机(16位)
int hostLong = NetworkUtils.ntohl(networkLong);   // 网络转主机(32位)

// DNS解析
String ip = NetworkUtils.resolveHost("www.example.com");

// 创建连接
TcpSocket socket = NetworkUtils.connectTcp("127.0.0.1", 8080);
UdpSocket udp = NetworkUtils.createUdp();
TcpServer server = NetworkUtils.createTcpServer(8080);
```

### TcpSocket - TCP客户端Socket

```cay
// 创建Socket
TcpSocket socket = new TcpSocket();

// 连接到服务器
bool connected = socket.connectTo("127.0.0.1", 8080);

// 发送数据
int sent = socket.send("Hello Server");

// 接收数据
String data = socket.receive(4096);        // 最大接收4096字节
String data = socket.receiveString(4096);  // 同上

// 关闭连接
socket.shutdownWrite();    // 关闭写入端
socket.shutdownRead();     // 关闭读取端
socket.shutdown();         // 关闭双向
socket.close();            // 完全关闭

// 状态检查
bool isConnected = socket.isConnected();
bool isValid = socket.isValid();

// Socket选项
socket.setReuseAddr(true);
socket.setTcpNoDelay(true);           // 禁用Nagle算法
socket.setSendBufferSize(8192);
socket.setRecvBufferSize(8192);
```

### TcpServer - TCP服务器

```cay
// 创建服务器
TcpServer server = new TcpServer();

// 绑定端口
bool bound = server.bindTo(8080);
bool bound = server.bind(SocketAddr addr);

// 开始监听
bool listening = server.listen(5);    // 连接队列长度5

// 接受连接
TcpSocket client = server.accept();

// 获取信息
int port = server.getPort();
bool isListening = server.isListening();

// 关闭服务器
server.close();
```

### UdpSocket - UDP Socket

```cay
// 创建UDP Socket
UdpSocket udp = new UdpSocket();
UdpSocket udp = NetworkUtils.createUdp();

// 绑定端口
bool bound = udp.bindTo(8080);

// 发送数据到指定地址
int sent = udp.sendTo("Hello", "127.0.0.1", 8080);

// 接收数据
SocketAddr sender = new SocketAddr();
String data = udp.receiveFrom(4096, sender);

// 状态检查
bool isBound = udp.isBound();
bool isValid = udp.isValid();

// 关闭
udp.close();
```

### HttpRequest - HTTP请求构建器

```cay
// 创建请求
HttpRequest request = new HttpRequest();

// 设置请求信息
request.setMethod("POST");
request.setUrl("/api/users");
request.addHeader("Content-Type", "application/json");
request.setBody("{\"name\":\"Alice\"}");

// 构建请求字符串
String httpRequest = request.build();
```

### HttpResponse - HTTP响应构建器

```cay
// 创建响应
HttpResponse response = new HttpResponse();

// 设置响应信息
response.setStatus(200, "OK");
response.addHeader("Content-Type", "application/json");
response.setBody("{\"id\":1,\"name\":\"Alice\"}");

// 构建响应字符串
String httpResponse = response.build();

// 获取信息
int statusCode = response.getStatusCode();
String body = response.getBody();
```

### HttpClient - HTTP客户端

```cay
// 创建客户端
HttpClient client = new HttpClient();
client.setBaseUrl("http://api.example.com");

// 发送请求
String response = client.get("/users");
String response = client.post("/users", "name=Alice");
```

### WebServer - 简单Web服务器

```cay
// 创建服务器
WebServer server = new WebServer(8080);

// 启动
bool started = server.start();

// 检查状态
bool running = server.isRunning();

// 停止
server.stop();
```

## 使用示例

### TCP客户端

```cay
#include <Network.cay>

public class Main {
    public static void main(String[] args) {
        // 初始化网络
        if (!NetworkUtils.init()) {
            println("网络初始化失败");
            return;
        }
        
        // 创建Socket并连接
        TcpSocket socket = new TcpSocket();
        if (!socket.connectTo("127.0.0.1", 8080)) {
            println("连接失败");
            NetworkUtils.cleanup();
            return;
        }
        
        println("已连接到服务器");
        
        // 发送数据
        socket.send("Hello Server!");
        
        // 接收响应
        String response = socket.receive(1024);
        println("收到: " + response);
        
        // 关闭连接
        socket.close();
        NetworkUtils.cleanup();
    }
}
```

### TCP服务器

```cay
#include <Network.cay>

public class Main {
    public static void main(String[] args) {
        // 初始化网络
        NetworkUtils.init();
        
        // 创建服务器
        TcpServer server = new TcpServer();
        
        // 绑定并监听
        if (!server.bindTo(8080)) {
            println("绑定失败");
            return;
        }
        
        if (!server.listen(5)) {
            println("监听失败");
            return;
        }
        
        println("服务器监听在端口 8080");
        
        // 接受连接
        TcpSocket client = server.accept();
        if (client != null) {
            println("客户端已连接");
            
            // 接收数据
            String data = client.receive(1024);
            println("收到: " + data);
            
            // 发送响应
            client.send("Hello Client!");
            
            // 关闭客户端连接
            client.close();
        }
        
        // 关闭服务器
        server.close();
        NetworkUtils.cleanup();
    }
}
```

### UDP通信

```cay
#include <Network.cay>

public class Main {
    public static void main(String[] args) {
        NetworkUtils.init();
        
        // 创建UDP Socket并绑定
        UdpSocket udp = new UdpSocket();
        udp.bindTo(8080);
        
        // 发送数据
        udp.sendTo("Hello UDP!", "127.0.0.1", 9090);
        
        // 接收数据
        SocketAddr sender = new SocketAddr();
        String data = udp.receiveFrom(1024, sender);
        println("从 " + sender.getIp() + ":" + sender.getPort() + " 收到: " + data);
        
        udp.close();
        NetworkUtils.cleanup();
    }
}
```

### HTTP客户端请求

```cay
#include <Network.cay>

public class Main {
    public static void main(String[] args) {
        NetworkUtils.init();
        
        // 创建HTTP客户端
        HttpClient client = new HttpClient();
        
        // 发送GET请求
        String response = client.get("http://example.com/api/data");
        println(response);
        
        // 发送POST请求
        String postResponse = client.post("http://example.com/api/users", "name=Alice");
        println(postResponse);
        
        NetworkUtils.cleanup();
    }
}
```

### 简单Web服务器

```cay
#include <Network.cay>

public class Main {
    public static void main(String[] args) {
        // 创建并启动服务器
        WebServer server = new WebServer(8080);
        
        if (server.start()) {
            println("服务器运行在 http://localhost:8080");
            
            // 保持运行（实际应用需要循环处理请求）
            while (server.isRunning()) {
                // 处理请求...
            }
        }
        
        server.stop();
    }
}
```

## 性能优化

### 时间复杂度

| 操作 | 时间复杂度 | 说明 |
|------|-----------|------|
| socket创建 | O(1) | 系统调用 |
| connect | O(1) | 网络延迟除外 |
| send | O(n) | n=数据大小 |
| receive | O(n) | n=数据大小 |
| bind | O(1) | 系统调用 |
| accept | O(1) | 阻塞等待 |

### 最佳实践

1. **初始化检查**: 始终检查 NetworkUtils.init() 返回值
2. **资源清理**: 使用完毕后调用 cleanup()
3. **错误处理**: 检查所有网络操作的返回值
4. **超时设置**: 生产环境应设置超时
5. **缓冲区大小**: 根据应用场景调整缓冲区

## 跨平台注意事项

### Windows 平台

- 使用 WSAStartup/WSACleanup 初始化 Winsock
- 使用 closesocket 关闭 Socket
- 使用 WSAGetLastError 获取错误码

### Linux 平台

- 使用 socket 系统调用
- 使用 close 关闭 Socket
- 使用 errno 获取错误码

### 条件编译

```cay
#ifdef _WIN32
    // Windows特定代码
#else
    // Linux特定代码
#endif
```

## 注意事项

1. **线程安全**: Socket 操作不是线程安全的
2. **信号处理**: Linux 下注意 EINTR 错误
3. **缓冲区溢出**: 始终指定最大接收长度
4. **资源泄漏**: 确保所有 Socket 都被关闭
5. **字节序**: 网络传输使用大端字节序
