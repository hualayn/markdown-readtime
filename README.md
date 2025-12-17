# markdown-readtime

一个用于估算 Markdown 内容阅读时间的 Rust 库。

[![Crates.io](https://img.shields.io/crates/v/markdown-readtime)](https://crates.io/crates/markdown-readtime)
[![Documentation](https://docs.rs/markdown-readtime/badge.svg)](https://docs.rs/markdown-readtime)
[![License](https://img.shields.io/github/hualayn/markdown-readtime)](https://github.com/hualayn/markdown-readtime/blob/main/README.md)

## 功能特性

- 📊 准确估算 Markdown 文本的阅读时间
- 🌍 支持中英文文本
- 😊 Emoji 处理支持
- 🖼️ 图片阅读时间计算
- 💻 代码块阅读时间计算
- ⚙️ 可自定义阅读速度参数
- 📦 轻量级，零依赖（可选 serde 支持）

## 安装

在 [Cargo.toml] 中添加以下依赖：

```toml
[dependencies]
markdown-readtime = "0.1"
```

## 快速开始

### 基础用法

```rust
use markdown_readtime::{estimate, minutes, words, formatted};

let markdown_content = r#"
# 我的第一篇博客文章

这是一些示例内容，用来演示如何使用 markdown-readtime 库。

## 子标题

我们还可以添加一些列表:
- 第一项
- 第二项
- 第三项

以及一些代码示例:

```rust
fn main() {
    println!("Hello, world!");
}
```
"#;

// 获取完整的阅读时间信息
let read_time = estimate(markdown_content);
println!("总阅读时间: {}秒", read_time.total_seconds);
println!("格式化时间: {}", read_time.formatted);
println!("字数统计: {}", read_time.word_count);
println!("图片数量: {}", read_time.image_count);
println!("代码块数量: {}", read_time.code_block_count);

// 或者使用快捷函数
println!("预计需要 {} 分钟读完", minutes(markdown_content));
println!("大约有 {} 个字", words(markdown_content));
println!("阅读时间: {}", formatted(markdown_content));
```

### 自定义阅读速度

```rust
use markdown_readtime::{estimate_with_speed, ReadSpeed};

let markdown_content = "# 示例文章\n\n这是用来测试的文章内容。";

// 创建自定义阅读速度配置
let speed = ReadSpeed::default()
    .wpm(180.0)             // 设置每分钟阅读180个词
    .image_time(15.0)       // 每张图片额外增加15秒
    .code_block_time(25.0)  // 每个代码块额外增加25秒
    .emoji(true)            // 考虑emoji
    .chinese(true);         // 中文模式

let read_time = estimate_with_speed(markdown_content, &speed);
println!("自定义配置下的阅读时间: {}秒", read_time.total_seconds);
```

## API 文档

### 主要函数

- `estimate(markdown: &str) -> ReadTime`: 估算 Markdown 内容的阅读时间
- `estimate_with_speed(markdown: &str, speed: &ReadSpeed) -> ReadTime`: 使用自定义速度配置估算阅读时间
- `minutes(markdown: &str) -> u64`: 快捷函数，返回向上取整的分钟数
- `words(markdown: &str) -> usize`: 快捷函数，返回字数统计
- `formatted(markdown: &str) -> String`: 快捷函数，返回格式化的阅读时间字符串

### 数据结构

#### ReadTime

```rust
pub struct ReadTime {
    pub total_seconds: u64,     // 总阅读时间（秒）
    pub formatted: String,      // 格式化后的阅读时间字符串
    pub word_count: usize,      // 单词数量
    pub image_count: usize,     // 图片数量
    pub code_block_count: usize, // 代码块数量
}
```

#### ReadSpeed

```rust
pub struct ReadSpeed {
    pub words_per_minute: f64,        // 每分钟阅读单词数（默认：200）
    pub seconds_per_image: f64,       // 每张图片额外时间（秒，默认：12）
    pub seconds_per_code_block: f64,  // 每个代码块额外时间（秒，默认：20）
    pub count_emoji: bool,            // 是否考虑emoji（默认：true）
    pub chinese: bool,                // 是否中文（默认：true）
}
```

## 特性（Features）

### serde

启用 `serde` 特性可以为 `ReadTime` 结构体添加序列化和反序列化支持：

```toml
[dependencies]
markdown-readtime = { version = "0.1", features = ["serde"] }
```

## 许可证

本项目采用 MIT 许可证。详细信息请查看 [LICENSE-MIT](LICENSE-MIT)文件。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个库！

## 致谢

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - 用于解析 Markdown 内容

---

以上内容由[*通义灵码*](https://lingma.aliyun.com/)生成