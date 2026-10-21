use lemmy_utils::error::LemmyResult;
use pttbbs_utils::settings::{BRDNAME_BLACKLIST_MAP, BRDNAME_WHITELIST_MAP, SETTINGS};
use std::{thread, time::Duration};
use tokio;
use tokio::signal::unix::SignalKind;

pub async fn retry_load_popular_boards() {
    if let Err(err) = BRDNAME_BLACKLIST_MAP.try_refresh() {
        println!("retry_load_popular_boards: unable to refresh BRDNAME_BLACKLIST_MAP e: {err:#?}");
        return;
    }
    if let Err(err) = BRDNAME_WHITELIST_MAP.try_refresh() {
        println!("retry_load_popular_boards: unable to refresh BRDNAME_WHITELIST_MAP e: {err:#?}");
        return;
    }

    let mut interrupt = tokio::signal::unix::signal(SignalKind::interrupt()).unwrap();
    let mut terminate = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();

    loop {
        tokio::select! {
            ret = load_popular_boards() => {
                println!("load popular boards: ret: {ret:?}");

                thread::sleep(Duration::from_secs(
                    SETTINGS.pttbbs.sleep_retry_load_popular_boards_ts,
                ));

            }
            _ = tokio::signal::ctrl_c() => {
                println!("received ctrl-c");
                break;
            }

            _ = interrupt.recv() => {
                tracing::warn!("Received interrupt, shutting down gracefully...");
                break;
            }
            _ = terminate.recv() => {
                tracing::warn!("Received terminate, shutting down gracefully...");
                break;
            }
        }
    }
}

async fn load_popular_boards() -> LemmyResult<()> {
    println!("load_popular_boards: start");
    Ok(())
}
