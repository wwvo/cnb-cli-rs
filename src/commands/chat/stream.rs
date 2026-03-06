//! SSE 流式响应解析与输出

use anyhow::Result;
use cnb_api::types::ChatStreamChunk;
use cnb_tui::style::dim;

/// 从 SSE 响应逐行读取并输出内容
///
/// - `Content` → stdout（正文）
/// - `ReasoningContent` → stderr（灰色，思考过程）
/// - `ModelResponse.type = 1` → stderr "Thinking..."
/// - `ModelResponse.type = 3` → stderr "...Done thinking."
/// - `data: [DONE]` → 结束
pub async fn print_stream(mut resp: reqwest::Response) -> Result<()> {
    let mut buf = String::new();

    // 逐 chunk 读取，手动按行拆分
    while let Some(chunk) = resp.chunk().await? {
        let text = String::from_utf8_lossy(&chunk);
        buf.push_str(&text);

        // 按行处理已收到的数据
        while let Some(pos) = buf.find('\n') {
            let line = buf[..pos].trim().to_string();
            buf.drain(..=pos);

            if let Some(data) = line.strip_prefix("data: ") {
                if data == "[DONE]" {
                    println!();
                    return Ok(());
                }
                process_sse_data(data);
            }
        }
    }

    // 处理缓冲区中剩余的最后一行
    let line = buf.trim().to_string();
    if let Some(data) = line.strip_prefix("data: ")
        && data != "[DONE]"
    {
        process_sse_data(data);
    }

    println!();
    Ok(())
}

/// 解析并输出单条 SSE data 事件
fn process_sse_data(data: &str) {
    let chunk: ChatStreamChunk = match serde_json::from_str(data) {
        Ok(c) => c,
        Err(_) => return,
    };

    // 处理 ModelResponse 状态
    if let Some(ref mr) = chunk.model_response {
        match mr.resp_type {
            1 => eprintln!("{}", dim("Thinking...")),
            3 => eprintln!("{}", dim("...Done thinking.")),
            _ => {}
        }
    }

    for choice in &chunk.choices {
        // 推理过程（灰色输出到 stderr）
        if !choice.delta.reasoning_content.is_empty() {
            eprint!("{}", dim(&choice.delta.reasoning_content));
        }

        // 正文内容（输出到 stdout）
        if !choice.delta.content.is_empty() {
            print!("{}", choice.delta.content);
        }
    }
}
