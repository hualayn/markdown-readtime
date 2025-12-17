use markdown_readtime::{estimate, formatted, minutes, words};

fn main() {
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
}"#;
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
}
