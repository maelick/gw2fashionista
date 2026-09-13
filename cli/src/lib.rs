use futures::StreamExt;

pub mod commands;
pub mod environment;
pub mod input;
pub mod output;

pub async fn partition_result<T, E>(
    result_stream: impl futures::Stream<Item = Result<T, E>>,
) -> (Vec<T>, Vec<E>) {
    result_stream
        .fold(
            (Vec::new(), Vec::new()),
            async |(mut created, mut failed), res| {
                match res {
                    Ok(fashion) => created.push(fashion),
                    Err(err) => failed.push(err),
                }
                (created, failed)
            },
        )
        .await
}
