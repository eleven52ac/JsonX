# 产品定位

打造一个**面向开发者的高性能 JSON 在线工具**。

核心目标不是单纯增加 JSON 工具数量，而是解决现有在线 JSON 格式化工具在处理大数据时容易出现的：

- 页面卡死
- 浏览器内存暴涨
- 格式化等待时间长
- 大量节点展开后无法操作
- 超大 JSON 无法查看
- UI 陈旧、信息密度过高
- 工具功能很多，但核心格式化体验一般

产品主打：

> **快、漂亮、能处理大 JSON。**

英文定位：

> **A fast JSON formatter built for large files.**

# 核心用户

主要面向：

- Java / Go / Python / JavaScript 开发者
- 后端开发人员
- 接口联调人员
- 测试人员
- 运维人员
- 数据开发人员

典型场景：

```
接口返回 JSON 太大
↓
复制到普通 JSON 工具
↓
浏览器直接卡死
↓
无法查看数据
```

本工具要解决的就是这个问题。

# 产品核心卖点

## 1. 超大 JSON

这是产品最重要的差异化能力。

目标支持：

```
1 MB       轻松处理
10 MB      轻松处理
100 MB     流畅处理
500 MB     可以正常查看
1 GB       文件模式下可以处理
```

这里不能只追求“能打开”。

真正的目标是：

> 大 JSON 打开以后仍然可以滚动、搜索、查看层级和定位数据。

# 2. 本地处理

JSON 默认不上传服务器。

架构：

```
JSON
 ↓
Browser
 ↓
Web Worker
 ↓
Rust WASM
 ↓
本地完成处理
```

服务器只负责提供：

```
HTML
CSS
JavaScript
WASM
```

因此可以明确宣传：

> 数据仅在浏览器本地处理，不会上传服务器。

英文：

> Your JSON never leaves your browser.

这同时解决：

- 隐私
- 网络速度
- 服务器成本
- 大文件上传
- 企业数据安全顾虑

# 3. 极致性能

不能使用传统模式：

```
JSON.parse()
↓
完整 JS Object
↓
JSON.stringify()
↓
完整渲染
```

大文件模式采用：

```
Stream
 ↓
Chunk
 ↓
Web Worker
 ↓
Rust WASM
 ↓
Streaming Tokenizer
 ↓
Formatter / Index
 ↓
Virtual Viewer
```

重点避免：

```
完整 JSON 对象常驻内存
完整 JSON DOM 渲染
完整 Tree 节点生成
实时全量语法高亮
```

# 4. 两种工作模式

## 普通模式

针对小型 JSON。

例如：

```
0 ~ 20 MB
```

提供完整编辑能力：

- 编辑
- 格式化
- 压缩
- JSON 校验
- 语法高亮
- 行号
- 折叠
- 搜索
- 复制
- 下载

重点：

> 编辑体验优秀。

## 大文件模式

达到一定大小后自动进入：

```
⚡ Large File Mode
```

例如：

```
> 20 MB
```

大文件模式优先保证：

```
不卡死
能打开
能浏览
能搜索
能定位
```

而不是强行保证完整编辑能力。

可以适当关闭：

```
实时语法检测
全量语法高亮
实时编辑
全量 Tree 生成
```

# 页面定位

不要做成传统工具站：

```
几十个菜单
各种广告
各种二维码
大量无关工具
```

而是做成：

> 一个专业、现代、简洁的开发者工具。

首页直接进入 JSON 工作区。

整体风格建议：

```
简洁
专业
现代
信息密度适中
支持暗色模式
弱化装饰
突出内容
```

可以参考：

```
GitHub
Linear
Vercel
Raycast
VS Code
```

的开发者工具视觉语言。

# 首页核心功能

第一版只需要：

```
JSON Formatter
JSON Validator
JSON Minify
Tree View
Copy
Download
Upload JSON File
```

顶部工具栏：

```
Format
Minify
Validate
Tree
Search
Clear
```

不要第一版就塞：

```
XML
YAML
CSV
SQL
JWT
Base64
URL Encode
UUID
时间戳
```

这些全部属于后续扩展。

第一版一定把：

> JSON

本身做好。

# 输入方式

支持：

## 粘贴

```
Ctrl + V
```

直接粘贴 JSON。

## 文件

支持：

```
.json
.txt
```

拖拽上传：

```
Drop JSON file here
```

注意：

文件不是上传服务器。

而是浏览器使用：

```
File API
ReadableStream
```

直接读取。

# Text View

普通文件：

```
CodeMirror 6
```

支持：

```
语法高亮
折叠
行号
搜索
错误提示
```

超大文件：

```
Virtual Text Viewer
```

只渲染可见区域。

例如：

```
JSON 总计：

3,820,000 行

实际 DOM：

200 ~ 500 行
```

# Tree View

这是第二个核心功能。

显示效果：

```
▼ root
   ├─ code: 200
   ├─ message: "success"
   └─ ▼ data [1283921]
        ├─ 0 {...}
        ├─ 1 {...}
        ├─ 2 {...}
```

但是禁止一次性创建：

```
1,283,921 个节点
```

必须使用：

```
Lazy Load
+
Virtual Scroll
```

例如：

```
data Array[1,283,921]

首次只显示：

0 ~ 99
```

继续滚动再加载。

# 大文件信息面板

打开大 JSON 后显示：

```
large-data.json

Size
482.7 MB

Status
Valid JSON

Root
Array

Objects
1,382,761

Arrays
28,421

Max Depth
17

Parse Time
1.21s
```

这会强化专业工具感。

# 处理状态

大文件不能出现：

```
点击格式化
↓
页面什么反应都没有
↓
10 秒
```

必须实时反馈：

```
Processing...

128 MB / 482 MB

27%
```

并展示：

```
读取
解析
建立索引
格式化
```

的进度。

# 技术定位

前端：

```
Vue 3
TypeScript
Vite
```

UI：

```
Tailwind CSS
```

编辑器：

```
CodeMirror 6
```

高性能核心：

```
Rust
WebAssembly
```

线程模型：

```
Web Worker
```

大型 JSON：

```
Streaming Parser
Streaming Formatter
Virtual Scroll
Lazy Tree
Offset Index
```

# Rust Core 职责

Rust 不负责 UI。

Rust 负责：

```
validate
format
minify
tokenize
scan
index
statistics
```

推荐组织：

```
rust-core
├── tokenizer
├── parser
├── formatter
├── validator
├── minifier
├── indexer
└── statistics
```

JS 只负责：

```
用户操作
UI
状态
展示
Worker 通信
```

# 性能原则

整个项目必须遵守：

## 禁止

```
大型 JSON 直接 JSON.parse
大型 JSON 直接 JSON.stringify
大型 JSON 全量高亮
大型 JSON 全量 DOM
大型数组全量 TreeNode
主线程处理大型 JSON
```

## 必须

```
Web Worker
分块处理
流式处理
虚拟滚动
按需渲染
懒加载
```

# 第一阶段目标

第一版不要追求功能多。

只实现：

```
1. JSON 粘贴

2. JSON 文件拖拽

3. Format

4. Minify

5. Validate

6. Text View

7. Tree View

8. 大文件模式

9. Web Worker

10. Rust WASM
```

做到：

> 普通 JSON 体验比传统 JSON 工具更漂亮。

同时：

> 大 JSON 明显比传统 JSON 工具更稳定。

# 后续扩展

第二阶段：

```
JSONPath
JSON Search
JSON Diff
JSON Sort
```

第三阶段：

```
JSON → YAML
JSON → XML
JSON → CSV
JSON → TypeScript
JSON → Java Bean
JSON → Go Struct
```

再往后才考虑成为：

```
Developer Tools
```

而不是第一天就做成大杂烩。

# 最终产品定位

这个产品不是：

> 又一个 JSON 格式化网站。

而应该是：

> 一个现代、高性能、专门为大 JSON 设计的 JSON 查看和处理工具。

三个核心标签：

**Large JSON**

**Local Processing**

**High Performance**

产品价值一句话：

> **让几百 MB 的 JSON，也能像普通 JSON 一样查看。**