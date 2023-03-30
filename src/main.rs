use chrono::prelude::*;
use librus::client::Librus;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut librus = Librus::default();

    let credentials = librus::client::LibrusCredentials {
        email: "".to_string(),
        password: "".to_string(),
    };

    librus.login(&credentials).await?;

    let timetable = librus.timetable_fetch_day(Local::now()).await?;

    timetable
        .iter()
        .filter(|hour| !hour.is_empty())
        .map(|hour| hour[0].subject.name.clone())
        .for_each(|_name| { /*println!("{}", name) */ });

    //let calendar = librus.calendar_from_date(Local::now()).await?;
    let next_month_date = Local::now() + chrono::Duration::days(30);
    let calendar_next_month = librus.calendar_from_date(next_month_date).await?;

    //let homeworks = librus.homeworks_from_calendar(calendar).await?;
    let mut homeworks_next_month = librus.homeworks_from_calendar(calendar_next_month).await?;

    // let mut both_homeworks = homeworks
    //     .iter()
    //     .chain(homeworks_next_month.iter())
    //     .collect::<Vec<_>>();

    homeworks_next_month.sort_by(|a, b| a.date.cmp(&b.date));

    let subjects = librus.get_subjects_by_numberids(homeworks_next_month.iter().map(|h| h.subject.clone()).collect()).await?;

    homeworks_next_month
        .iter()
        .for_each(|homework| println!("{} - {}", homework.date, homework.content));


    Ok(())
}
