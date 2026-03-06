//! Agent 循环逻辑
//!
//! 最多 10 轮：AI 响应 → 解析动作 → 执行 → 追加结果 → 下一轮

use std::collections::HashMap;

use anyhow::Result;
use cnb_api::client::CnbClient;
use cnb_api::types::{ChatCompletionsRequest, ChatMessage};
use cnb_chat::action::{Action, parse_action};
use cnb_chat::curl::exec_curl;
use cnb_chat::docs::get_api_doc;
use cnb_chat::prompt::build_system_prompt;

use super::stream::print_stream;

/// Agent 最大循环轮次
const MAX_TURNS: usize = 10;

/// 默认模型名称
const DEFAULT_MODEL: &str = "hunyuan-a13b";

/// 执行 Agent 循环
///
/// `stream_output` 控制最终回答是否流式输出
pub async fn run_agent(
    client: &CnbClient,
    question: &str,
    stream_output: bool,
) -> Result<()> {
    let token = client.token().to_string();
    let curl_vars: HashMap<String, String> =
        [("<CNB_TOKEN>".to_string(), token)].into_iter().collect();

    let mut messages = vec![
        ChatMessage::system(&build_system_prompt()),
        ChatMessage::user(question),
    ];

    for _turn in 0..MAX_TURNS {
        // 显示等待提示
        eprint!("\x1b[90m⠋ 正在思考...\x1b[0m");

        // 非流式调用 AI（用于 Agent 中间轮次）
        let req = ChatCompletionsRequest {
            model: DEFAULT_MODEL.to_string(),
            stream: false,
            messages: messages.clone(),
        };

        let resp = client.ai_chat(&req).await?;

        // 清除等待提示
        eprint!("\r\x1b[K");

        let Some(choice) = resp.choices.first() else {
            anyhow::bail!("AI 返回空响应");
        };
        let ai_content = &choice.message.content;

        // 解析 AI 响应中的动作指令
        let action = parse_action(ai_content);

        match action {
            None => {
                // AI 给出最终回答
                if stream_output {
                    // 用流式输出重新请求最终回答
                    return stream_final_answer(client, &messages).await;
                }
                // 非流式：直接输出
                println!("{ai_content}");
                return Ok(());
            }
            Some(Action::GetApiDoc(doc_ref)) => {
                eprintln!("\x1b[90m[获取文档] {doc_ref}\x1b[0m");

                // 将 AI 响应加入消息历史
                messages.push(ChatMessage::assistant(ai_content));

                // 查询文档并追加结果
                let doc = match get_api_doc(&doc_ref) {
                    Ok(d) => d,
                    Err(e) => format!("错误：{e}"),
                };
                messages.push(ChatMessage::user(&format!(
                    "以下是 {doc_ref} 的详细 API 文档:\n\n{doc}"
                )));
            }
            Some(Action::Curl(curl_cmd)) => {
                // 截断显示
                let display = if curl_cmd.len() > 200 {
                    format!("{}...", &curl_cmd[..200])
                } else {
                    curl_cmd.clone()
                };
                eprintln!("\x1b[90m[执行] {display}\x1b[0m");

                // 将 AI 响应加入消息历史
                messages.push(ChatMessage::assistant(ai_content));

                // 执行 curl 命令
                let result = exec_curl(client.http_client(), &curl_cmd, &curl_vars).await;

                if result.success {
                    eprintln!("\x1b[90m[执行结果] 成功\x1b[0m");
                } else if let Some(ref err) = result.error {
                    eprintln!("\x1b[90m[执行结果] 失败: {err}\x1b[0m");
                }

                let result_json = serde_json::to_string_pretty(&result.data)
                    .unwrap_or_else(|_| "null".to_string());
                messages.push(ChatMessage::user(&format!(
                    "curl 执行结果:\n{result_json}"
                )));
            }
        }
    }

    eprintln!("已达到最大调用轮次，请尝试简化请求。");
    Ok(())
}

/// 对最终回答使用流式输出
async fn stream_final_answer(
    client: &CnbClient,
    messages: &[ChatMessage],
) -> Result<()> {
    let req = ChatCompletionsRequest {
        model: DEFAULT_MODEL.to_string(),
        stream: true,
        messages: messages.to_vec(),
    };

    let resp = client.ai_chat_stream(&req).await?;
    print_stream(resp).await
}
