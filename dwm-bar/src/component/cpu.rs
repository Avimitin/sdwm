use super::widget::Block;
use std::time::Duration;
use tokio::fs;
use tokio::time::sleep;

struct CpuStat {
    sum: i32,
    active: i32,
}

async fn get_stats() -> Option<CpuStat> {
    let status = fs::read_to_string("/proc/stat").await.ok()?;
    let mut time = Vec::new();
    for line in status.lines() {
        if line.starts_with("cpu") {
            // remove the "cpu" prefix
            time = line
                .split_whitespace()
                .skip(1)
                .map(|x| {
                    x.parse::<i32>()
                        .expect("Fail to parse the /proc/stat file, please check your system")
                })
                .collect();
            break;
        }
    }

    if time.len() < 8 {
        return None;
    }

    let sum: i32 = time.iter().sum();
    // idle + iowait
    let inactive = time[3] + time[4];

    Some(CpuStat {
        sum,
        active: sum - inactive,
    })
}

pub async fn avg_load() -> Option<Block> {
    let before = get_stats().await?;

    sleep(Duration::from_secs(1)).await;

    let after = get_stats().await?;

    // get total
    let sum = (before.sum - after.sum) as f32;
    // use active time / total
    let avg = ((before.active - after.active) as f32) / sum;

    Some(
        Block::new("﬙", format!("{:.2} %", avg * 100.0))
            .text_fg("#EAEAEA")
            .icon_fg("#EAEAEA"),
    )
}

#[tokio::test]
async fn test() {
    dbg!(avg_load().await);
}
