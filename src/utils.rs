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

/// 计算文本中的中文字数
pub fn count_words(text: &str, count_emoji: bool) -> usize {
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
pub fn count_english_words(text: &str, count_emoji: bool) -> usize {
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
pub fn format_time(seconds: u64) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_format_time() {
        assert_eq!(format_time(45), "45秒");
    }
}
