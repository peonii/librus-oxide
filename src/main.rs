use librus::client::Librus;
use chrono::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut librus = Librus::default();

    let credentials = librus::client::LibrusCredentials {
        email: "".to_string(),
        password: "".to_string(),
    };

    librus.login(&credentials).await?;

    let timetable = librus.fetch_day(Local::now()).await?;

    timetable
        .iter()
        .filter(|hour| !hour.is_empty())
        .map(|hour| hour[0].subject.name.clone())
        .for_each(|name| println!("{}", name));

    Ok(())
}
