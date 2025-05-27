use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;

fn main() {
    let now_utc: DateTime<Utc> = Utc::now();
    let now_jst = now_utc.with_timezone(&Tokyo);

    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP={}",
        now_jst.format("%Y-%m-%d %H:%M:%S")
    );
}
