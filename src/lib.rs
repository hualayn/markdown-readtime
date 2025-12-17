use pulldown_cmark::{Event, Parser, Tag, TagEnd};

/// 阅读时间计算结果
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadTime {
    /// 总阅读时间（秒）
    pub total_seconds: u64,
    /// 格式化后的阅读时间字符串
    pub formatted: String,
    /// 单词数量
    pub word_count: usize,
    /// 图片数量
    pub image_count: usize,
    /// 代码块数量
    pub code_block_count: usize,
}

/// 阅读速度配置
#[derive(Debug, Clone, Copy)]
pub struct ReadSpeed {
    /// 每分钟阅读单词数（默认：200）
    pub words_per_minute: f64,
    /// 每张图片额外时间（秒，默认：12）
    pub seconds_per_image: f64,
    /// 每个代码块额外时间（秒，默认：20）
    pub seconds_per_code_block: f64,
    /// 是否考虑emoji（默认：true）
    pub count_emoji: bool,
    /// 是否中文
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
pub fn estimate(markdown: &str) -> ReadTime {
    estimate_with_speed(markdown, &ReadSpeed::default())
}

/// 使用自定义速度配置估算阅读时间
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

/// 计算文本中的中文字数
fn count_words(text: &str, count_emoji: bool) -> usize {
    if count_emoji {
        // 对于包含emoji的文本，计算非空白字符数
        text.chars()
            .filter(|c| !c.is_whitespace() && (!c.is_control() || c.is_emoji()))
            .count()
    } else {
        // 直接计算非空白字符数，适用于中文等无空格分隔的语言
        text.chars().filter(|c| !c.is_whitespace()).count()
    }
}

/// 计算文本中的英文字数
fn count_english_words(text: &str, count_emoji: bool) -> usize {
    if count_emoji {
        // 计算空格分隔的单词数，并考虑emoji作为独立单位
        text.split_whitespace()
            .map(|word| {
                // 对于每个单词，如果包含emoji，则每个emoji算作一个单位
                let emoji_count = word.chars().filter(|c| c.is_emoji()).count();
                if emoji_count > 0 {
                    // 如果有emoji，将单词拆分为普通字符和emoji
                    let non_emoji_chars: usize = word
                        .chars()
                        .filter(|c| !c.is_emoji() && !c.is_whitespace())
                        .count();
                    // 每个非emoji字符算一个单位，每个emoji也算一个单位
                    non_emoji_chars + emoji_count
                } else {
                    // 没有emoji则整个单词算一个单位
                    1
                }
            })
            .sum()
    } else {
        text.split_whitespace().count()
    }
}

/// 格式化时间显示
fn format_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;

    if minutes == 0 {
        format!("{}秒", seconds)
    } else if remaining_seconds == 0 {
        format!("{}分钟", minutes)
    } else {
        format!("{}分{}秒", minutes, remaining_seconds)
    }
}

/// 快捷函数：获取分钟数
pub fn minutes(markdown: &str) -> u64 {
    let read_time = estimate(markdown);
    (read_time.total_seconds as f64 / 60.0).ceil() as u64
}

/// 快捷函数：获取单词数
pub fn words(markdown: &str) -> usize {
    estimate(markdown).word_count
}

/// 快捷函数：获取格式化字符串
pub fn formatted(markdown: &str) -> String {
    estimate(markdown).formatted
}

/// emoji支持扩展
trait CharExt {
    fn is_emoji(&self) -> bool;
}

impl CharExt for char {
    fn is_emoji(&self) -> bool {
        // 简单的emoji范围检测
        matches!(*self as u32,
            0x1F600..=0x1F64F |  // Emoticons
            0x1F300..=0x1F5FF |  // Miscellaneous Symbols and Pictographs
            0x1F680..=0x1F6FF |  // Transport and Map Symbols
            0x1F700..=0x1F77F |  // Alchemical Symbols
            0x1F780..=0x1F7FF |  // Geometric Shapes Extended
            0x1F800..=0x1F8FF |  // Supplemental Arrows-C
            0x1F900..=0x1F9FF |  // Supplemental Symbols and Pictographs
            0x1FA00..=0x1FA6F |  // Chess Symbols
            0x1FA70..=0x1FAFF |  // Symbols and Pictographs Extended-A
            0x2600..=0x26FF   |  // Miscellaneous Symbols
            0x2700..=0x27BF   |  // Dingbats
            0x2B50           |  // star
            0x2B55              // heavy large circle
        )
    }
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
    fn test_count_words() {
        let text = "你好，世界！";
        let word_count = count_words(text, true);
        assert_eq!(word_count, 6);
    }

    #[test]
    fn test_count_english_words() {
        let text = "Hello world! This is a test.";
        let word_count = count_english_words(text, true);
        assert_eq!(word_count, 6);
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
