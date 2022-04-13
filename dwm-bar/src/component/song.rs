use mpris::PlayerFinder;
use super::component::Component;

pub fn song_info() -> Option<Component> {
    // TODO: We need to use logging to report error here.
    let player = PlayerFinder::new().ok()?.find_active().ok()?;

    let text_limit = 40;
    let metadata = player.get_metadata().ok()?;

    let artist = metadata.artists()?.join(" ");
    let song = metadata.title()?;

    let output = format!(
        " {} - {} ",
        if !artist.is_empty() {
            artist
        } else {
            "Anonymous".to_string()
        },
        song,
    );

    // trim the text
    let output = if output.len() > text_limit {
        let split = output.chars().take(text_limit).collect::<String>();
        format!("{}...", split)
    } else {
        output
    };

    Some(
        Component::new(" ", output)
            .icon_color("#EAEAEA", "#0C0C0C")
            .text_color("#EAEAEA", "#171617"),
    )
}
