// FIXME: We should keep the connection

use super::widget::Block;
use anyhow::Result;
use dbus::arg;
use dbus::nonblock::{stdintf::org_freedesktop_dbus::Properties, Proxy, SyncConnection};
use dbus_tokio::connection;
use std::sync::Arc;
use std::time::Duration;

pub async fn song_info() -> Option<Block> {
    let (resource, conn) = connection::new_session_sync().ok()?;

    // keep the connection
    let _conn_handle = tokio::spawn(async {
        resource.await;
    });

    let player_addr = find_active_player_address(conn.clone()).await.ok()?;
    let metadata = get_metadata(conn.clone(), &player_addr).await.ok()?;

    let artist: Option<&Vec<String>> = arg::prop_cast(&metadata, "xesam:artist");
    let artist = artist?.join(" ");
    let song: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
    let song = song?;

    let output = format!(
        " {} - {} ",
        if !artist.is_empty() {
            artist
        } else {
            "Anonymous".to_string()
        },
        song,
    );

    let text_limit = 40;
    // trim the text
    let output = if output.len() > text_limit {
        let split = output.chars().take(text_limit).collect::<String>();
        format!("{}...", split)
    } else {
        output
    };

    Some(
        Block::new(" ", output)
            .icon_color("#EAEAEA", "#0C0C0C")
            .text_color("#EAEAEA", "#171617"),
    )
}

async fn find_active_player_address(conn: Arc<SyncConnection>) -> Result<String> {
    let proxy = Proxy::new(
        "org.freedesktop.DBus",
        "/",
        Duration::from_millis(2000),
        conn,
    );
    let (services,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .await?;

    for service in services {
        if service.contains("mpris") {
            return Ok(service);
        }
    }

    anyhow::bail!("No active mpris player was found")
}

async fn get_metadata(conn: Arc<SyncConnection>, addr: &str) -> Result<arg::PropMap> {
    let proxy = Proxy::new(
        addr,
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(2000),
        conn,
    );
    Ok(proxy
        .get("org.mpris.MediaPlayer2.Player", "Metadata")
        .await?)
}
