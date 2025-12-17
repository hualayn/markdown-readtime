use markdown_readtime::{ReadSpeed, estimate_with_speed};

fn main() {
    let markdown_content = "# 示例文章\n\n这是用来测试的文章内容。";

    // 创建自定义阅读速度配置
    let speed = ReadSpeed::default()
        .wpm(180.0) // 设置每分钟阅读180个词
        .image_time(15.0) // 每张图片额外增加15秒
        .code_block_time(25.0) // 每个代码块额外增加25秒
        .emoji(true) // 考虑emoji
        .chinese(true); // 中文模式

    let read_time = estimate_with_speed(markdown_content, &speed);
    println!("自定义配置下的阅读时间: {}秒", read_time.total_seconds);
}
