//! 并发执行辅助工具
//!
//! 提供并发执行和错误统计的通用功能，避免重复代码。

use futures::stream::{self, StreamExt};
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 并发执行多个操作并统计失败数量。
///
/// 该函数只负责并发调度和计数，不负责输出错误日志，
/// 以便调用方保留更完整的业务上下文。
///
/// # 参数
/// - `items`: 待处理的项目列表
/// - `concurrency`: 最大并发数
/// - `operation`: 对每个项目执行的操作，返回 `Result<()>`
///
/// # 返回
/// 失败的操作数量
///
/// # 示例
/// ```ignore
/// let failures = run_concurrent(&items, 10, |item| async move {
///     client.delete(item).await
/// }).await;
/// ```
pub async fn run_concurrent<T, F, Fut, E>(items: &[T], concurrency: usize, operation: F) -> usize
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = Result<(), E>>,
{
    let failures = Arc::new(AtomicUsize::new(0));
    let operation = &operation;

    stream::iter(items.iter())
        .for_each_concurrent(concurrency, |item| {
            let failures = failures.clone();
            async move {
                if operation(item).await.is_err() {
                    failures.fetch_add(1, Ordering::SeqCst);
                }
            }
        })
        .await;

    failures.load(Ordering::SeqCst)
}

/// 并发执行多个操作并统计成功/失败数量。
///
/// 与 `run_concurrent` 类似，但提供成功和失败两个计数器，
/// 适用于需要同时报告成功数量的场景。
///
/// # 参数
/// - `items`: 待处理的项目列表
/// - `concurrency`: 最大并发数
/// - `operation`: 对每个项目执行的操作，返回 `Result<()>`
///
/// # 返回
/// (成功数量, 失败数量)
pub async fn run_concurrent_with_success_count<T, F, Fut, E>(
    items: &[T],
    concurrency: usize,
    operation: F,
) -> (usize, usize)
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = Result<(), E>>,
{
    let successes = Arc::new(AtomicUsize::new(0));
    let failures = Arc::new(AtomicUsize::new(0));
    let operation = &operation;

    stream::iter(items.iter())
        .for_each_concurrent(concurrency, |item| {
            let successes = successes.clone();
            let failures = failures.clone();
            async move {
                match operation(item).await {
                    Ok(()) => {
                        successes.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(_) => {
                        failures.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        })
        .await;

    (
        successes.load(Ordering::SeqCst),
        failures.load(Ordering::SeqCst),
    )
}

#[cfg(test)]
mod tests {
    use super::{run_concurrent, run_concurrent_with_success_count};

    #[test]
    fn run_concurrent_counts_failures() {
        let items = [1, 2, 3, 4];
        let failed = futures::executor::block_on(run_concurrent(&items, 2, |item| {
            let should_fail = *item % 2 == 0;
            async move {
                if should_fail {
                    anyhow::bail!("boom");
                }
                Ok(())
            }
        }));

        assert_eq!(failed, 2);
    }

    #[test]
    fn run_concurrent_with_success_count_tracks_both_results() {
        let items = [1, 2, 3];
        let (success, failed) =
            futures::executor::block_on(run_concurrent_with_success_count(&items, 3, |item| {
                let should_fail = *item == 2;
                async move {
                    if should_fail {
                        anyhow::bail!("boom");
                    }
                    Ok(())
                }
            }));

        assert_eq!(success, 2);
        assert_eq!(failed, 1);
    }
}
