//! # markdown-readtime
//!
//! 一个用于估算 Markdown 内容阅读时间的 Rust 库。
//!
//! ## 功能特性
//!
//! - 📊 准确估算 Markdown 文本的阅读时间
//! - 🌍 支持中英文文本
//! - 😊 Emoji 处理支持
//! - 🖼️ 图片阅读时间计算
//! - 💻 代码块阅读时间计算
//! - ⚙️ 可自定义阅读速度参数
//! - 📦 轻量级，零依赖（可选 serde 支持）
//!
//! ## 快速开始
//!
//! ### 基础用法
//!
//! ```
//! use markdown_readtime::{estimate, minutes, words, formatted};
//!
//! let markdown_content = r#"
//! # 我的第一篇博客文章
//!
//! 这是一些示例内容，用来演示如何使用 markdown-readtime 库。
//!
//! ## 子标题
//!
//! 我们还可以添加一些列表:
//! - 第一项
//! - 第二项
//! - 第三项
//! "#;
//!
//! // 获取完整的阅读时间信息
//! let read_time = estimate(markdown_content);
//! println!("总阅读时间: {}秒", read_time.total_seconds);
//! println!("格式化时间: {}", read_time.formatted);
//! println!("字数统计: {}", read_time.word_count);
//!
//! // 或者使用快捷函数
//! println!("预计需要 {} 分钟读完", minutes(markdown_content));
//! println!("大约有 {} 个字", words(markdown_content));
//! println!("阅读时间: {}", formatted(markdown_content));
//! ```
//!
//! ### 自定义阅读速度
//!
//! ```
//! use markdown_readtime::{estimate_with_speed, ReadSpeed};
//!
//! let markdown_content = "# 示例文章\n\n这是用来测试的文章内容。";
//!
//! // 创建自定义阅读速度配置
//! let speed = ReadSpeed::default()
//!     .wpm(180.0)             // 设置每分钟阅读180个词
//!     .image_time(15.0)       // 每张图片额外增加15秒
//!     .code_block_time(25.0)  // 每个代码块额外增加25秒
//!     .emoji(true)            // 考虑emoji
//!     .chinese(true);         // 中文模式
//!
//! let read_time = estimate_with_speed(markdown_content, &speed);
//! println!("自定义配置下的阅读时间: {}秒", read_time.total_seconds);
//! ```
mod utils;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use utils::*;

/// 阅读时间估算结果
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadTime {
    /// 总阅读时间（秒）
    ///
    /// 这是向上取整后的总秒数，包括文本阅读时间、图片额外时间和代码块额外时间。
    pub total_seconds: u64,

    /// 格式化后的阅读时间字符串
    ///
    /// 将秒数转换为人类友好的格式，例如 "30秒"、"5分钟" 或 "2分30秒"。
    pub formatted: String,

    /// 单词数量
    ///
    /// 根据是否为中文文本，分别采用不同的计数方式：
    /// - 中文：计算非空白字符数
    /// - 英文：计算空格分隔的单词数
    pub word_count: usize,

    /// 图片数量
    ///
    /// Markdown 中 `![alt text](image_url)` 格式的图片数量。
    pub image_count: usize,

    /// 代码块数量
    ///
    /// Markdown 中 ```code``` 格式的代码块数量。
    pub code_block_count: usize,
}

/// 阅读速度配置
///
/// 允许自定义各种影响阅读时间的因素。
///
/// # Examples
///
/// ```
/// use markdown_readtime::ReadSpeed;
///
/// // 使用构建器模式创建自定义配置
/// let speed = ReadSpeed::default()
///     .wpm(180.0)
///     .image_time(15.0)
///     .code_block_time(25.0)
///     .emoji(false);
///
/// // 或者直接创建
/// let speed = ReadSpeed::new(180.0, 15.0, 25.0, false, true);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ReadSpeed {
    /// 每分钟阅读单词数（默认：200）
    ///
    /// 这是阅读速度的核心参数，用于计算文本的基础阅读时间。
    pub words_per_minute: f64,

    /// 每张图片额外时间（秒，默认：12）
    ///
    /// 每发现一张图片就会增加相应的时间，因为读者通常需要额外时间查看图片。
    pub seconds_per_image: f64,

    /// 每个代码块额外时间（秒，默认：20）
    ///
    /// 每发现一个代码块就会增加相应的时间，因为代码通常需要更仔细的阅读。
    pub seconds_per_code_block: f64,

    /// 是否考虑emoji（默认：true）
    ///
    /// 当启用时，emoji 会被单独计数，影响总的阅读时间估算。
    pub count_emoji: bool,

    /// 是否中文（默认：true）
    ///
    /// 决定使用哪种文本计数方式：
    /// - `true`: 使用中文计数方式（计算字符数）
    /// - `false`: 使用英文计数方式（计算单词数）
    pub chinese: bool,
}

impl Default for ReadSpeed {
    fn default() -> Self {
        Self {
            words_per_minute: 200.0,
            seconds_per_image: 12.0,
            seconds_per_code_block: 20.0,
            count_emoji: true,
            chinese: true,
        }
    }
}

impl ReadSpeed {
    pub fn new(
        wpm: f64,
        seconds_per_image: f64,
        seconds_per_code_block: f64,
        count_emoji: bool,
        chinese: bool,
    ) -> Self {
        Self {
            words_per_minute: wpm,
            seconds_per_image,
            seconds_per_code_block,
            count_emoji,
            chinese,
        }
    }

    pub fn wpm(mut self, wpm: f64) -> Self {
        self.words_per_minute = wpm;
        self
    }

    pub fn image_time(mut self, seconds: f64) -> Self {
        self.seconds_per_image = seconds;
        self
    }

    pub fn code_block_time(mut self, seconds: f64) -> Self {
        self.seconds_per_code_block = seconds;
        self
    }

    pub fn emoji(mut self, count: bool) -> Self {
        self.count_emoji = count;
        self
    }

    pub fn chinese(mut self, is_chinese: bool) -> Self {
        self.chinese = is_chinese;
        self
    }
}

/// 估算Markdown的阅读时间
///
/// 使用默认的阅读速度配置来估算给定 Markdown 文本的阅读时间。
///
/// # Arguments
///
/// * `markdown` - 需要估算阅读时间的 Markdown 文本
///
/// # Returns
///
/// 返回包含阅读时间信息的 [`ReadTime`] 结构体。
///
/// # Examples
///
/// ```
/// use markdown_readtime::estimate;
///
/// let markdown = "# 标题\n\n这是内容";
/// let read_time = estimate(markdown);
/// println!("阅读需要 {} 时间", read_time.formatted);
/// ```
pub fn estimate(markdown: &str) -> ReadTime {
    estimate_with_speed(markdown, &ReadSpeed::default())
}

/// 使用自定义速度配置估算阅读时间
///
/// 使用指定的阅读速度配置来估算给定 Markdown 文本的阅读时间。
///
/// # Arguments
///
/// * `markdown` - 需要估算阅读时间的 Markdown 文本
/// * `speed` - 自定义的阅读速度配置
///
/// # Returns
///
/// 返回包含阅读时间信息的 [`ReadTime`] 结构体。
///
/// # Examples
///
/// ```
/// use markdown_readtime::{estimate_with_speed, ReadSpeed};
///
/// let markdown = "# Title\n\nThis is content";
/// let speed = ReadSpeed::default().wpm(180.0);
/// let read_time = estimate_with_speed(markdown, &speed);
/// println!("阅读需要 {} 时间", read_time.formatted);
/// ```
pub fn estimate_with_speed(markdown: &str, speed: &ReadSpeed) -> ReadTime {
    let parser = Parser::new(markdown);

    let mut word_count = 0;
    let mut image_count = 0;
    let mut code_block_count = 0;
    let mut in_code_block = false;
    let mut in_image_alt = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Image { .. } => {
                    image_count += 1;
                    in_image_alt = true;
                }
                Tag::CodeBlock(_) => {
                    code_block_count += 1;
                    in_code_block = true;
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Image { .. } => {
                    in_image_alt = false;
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                }
                _ => {}
            },
            Event::Text(text) => {
                if !in_image_alt && !in_code_block {
                    if speed.chinese {
                        word_count += count_words(&text.to_string(), speed.count_emoji);
                    } else {
                        word_count += count_english_words(&text.to_string(), speed.count_emoji);
                    }
                }
            }
            Event::Code(code) => {
                if !in_code_block {
                    if speed.chinese {
                        word_count += count_words(&code.to_string(), speed.count_emoji);
                    } else {
                        word_count += count_english_words(&code.to_string(), speed.count_emoji);
                    }
                }
            }
            _ => {}
        }
    }

    // 计算基础阅读时间（基于单词数）
    let base_seconds = (word_count as f64 / speed.words_per_minute) * 60.0;

    // 添加图片和代码块的额外时间
    let image_seconds = image_count as f64 * speed.seconds_per_image;
    let code_seconds = code_block_count as f64 * speed.seconds_per_code_block;

    let total_seconds = (base_seconds + image_seconds + code_seconds).ceil() as u64;

    ReadTime {
        total_seconds,
        formatted: format_time(total_seconds),
        word_count,
        image_count,
        code_block_count,
    }
}

/// 快捷函数：获取分钟数
///
/// 估算阅读时间并向上去整到最近的分钟数。
///
/// # Arguments
///
/// * `markdown` - 需要估算阅读时间的 Markdown 文本
///
/// # Returns
///
/// 向上取整后的分钟数。
///
/// # Examples
///
/// ```
/// use markdown_readtime::minutes;
///
/// let markdown = "# 标题\n\n这是内容";
/// let mins = minutes(markdown);
/// println!("大约需要 {} 分钟阅读", mins);
/// ```
pub fn minutes(markdown: &str) -> u64 {
    let read_time = estimate(markdown);
    (read_time.total_seconds as f64 / 60.0).ceil() as u64
}

/// 快捷函数：获取单词数
///
/// 计算 Markdown 文本中的单词数量。
///
/// # Arguments
///
/// * `markdown` - 需要计算单词数的 Markdown 文本
///
/// # Returns
///
/// 单词数量。
///
/// # Examples
///
/// ```
/// use markdown_readtime::words;
///
/// let markdown = "# 标题\n\n这是内容";
/// let word_count = words(markdown);
/// println!("共有 {} 个字", word_count);
/// ```
pub fn words(markdown: &str) -> usize {
    estimate(markdown).word_count
}

/// 快捷函数：获取格式化字符串
///
/// 获取格式化后的阅读时间字符串。
///
/// # Arguments
///
/// * `markdown` - 需要估算阅读时间的 Markdown 文本
///
/// # Returns
///
/// 格式化后的阅读时间字符串，例如 "30秒"、"5分钟" 或 "2分30秒"。
///
/// # Examples
///
/// ```
/// use markdown_readtime::formatted;
///
/// let markdown = "# 标题\n\n这是内容";
/// let formatted_time = formatted(markdown);
/// println!("阅读时间: {}", formatted_time);
/// ```
pub fn formatted(markdown: &str) -> String {
    estimate(markdown).formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate() {
        let md_txt = r#"
# 标题
## 子标题
### 子子标题
1. 列表1
2. 列表2
"#
        .trim();
        let read_time = estimate(md_txt);
        assert_eq!(read_time.word_count, 15);
        assert_eq!(read_time.image_count, 0);
        assert_eq!(read_time.code_block_count, 0);
        assert_eq!(read_time.total_seconds, 5);
        assert_eq!(read_time.formatted, "5秒");
    }

    #[test]
    fn test_estimate_with_speed() {
        // 测试中文
        let md_txt = r#"
# 标题
## 子标题
### 子子标题
1. 列表1
2. 列表2
"#
        .trim();
        let speed = ReadSpeed::new(100.0, 10.0, 15.0, true, true);
        let read_time = estimate_with_speed(md_txt, &speed);
        assert_eq!(read_time.word_count, 15);
        assert_eq!(read_time.image_count, 0);
        assert_eq!(read_time.code_block_count, 0);
        assert_eq!(read_time.total_seconds, 9);
        assert_eq!(read_time.formatted, "9秒");

        // 测试英文
        let md_txt_english = r#"
# Title

This is a test paragraph. It contains some words.
"#
        .trim();

        let speed = ReadSpeed::new(200.0, 10.0, 15.0, true, false);
        let read_time = estimate_with_speed(md_txt_english, &speed);
        assert_eq!(read_time.word_count, 10);
        assert_eq!(read_time.total_seconds, 3);
        assert_eq!(read_time.formatted, "3秒");
    }

    #[test]
    fn test_formatted() {
        let md_txt = r#"
# 测试标题
## 子标题
### 子子标题
- 列表项1
- 列表项2
"#
        .trim();
        let formatted_time = formatted(md_txt);
        assert_eq!(formatted_time, "6秒");
    }
}
