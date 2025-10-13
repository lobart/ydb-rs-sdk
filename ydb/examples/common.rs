use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use ydb::YdbResult;
use ydb::{ydb_struct, Bytes, Value};
pub fn get_data_for_it_crowd() -> YdbResult<(Value, Value, Value)> {
    let series_id = Uuid::new_v4().to_string();
    let series = ydb_struct!(
        "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
        "title" => "IT Crowd",
        "series_info" => "The IT Crowd is a British sitcom produced by Channel 4, written by Graham Linehan, produced by \
         Ash Atalla and starring Chris O'Dowd, Richard Ayoade, Katherine Parkinson, and Matt Berry.",
        "release_date" => Value::Date(date("2006-02-21")),
        "comment" => ""
    );
    let seasons_ids: Vec<String> = (0..5).map(|_| Uuid::new_v4().to_string()).collect();

    let seasons = vec![
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[0]))),
            "title"=> "Season 1",
            "first_aired"=> Value::Date(date("2006-02-03")),
            "last_aired"=> Value::Date(date("2006-03-03")),
        ),
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[1]))),
            "title"=> "Season 2",
            "first_aired"=> Value::Date(date("2007-08-24")),
            "last_aired"=> Value::Date(date("2007-09-28")),
        ),
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[2]))),
            "title"=> "Season 3",
            "first_aired"=> Value::Date(date("2008-11-21")),
            "last_aired"=> Value::Date(date("2008-12-26")),
        ),
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[3]))),
            "title"=> "Season 4",
            "first_aired"=> Value::Date(date("2010-06-25")),
            "last_aired"=> Value::Date(date("2010-07-30")),
        ),
    ];
    let episodes_1 = HashMap::from([
        ("Yesterday's Jam", date("2006-02-03")),
        ("Calamity Jen", date("2006-02-03")),
        ("Fifty-Fifty", date("2006-02-10")),
        ("The Red Door", date("2006-02-17")),
        ("The Haunting of Bill Crouse", date("2006-02-24")),
        ("Aunt Irma Visits", date("2006-03-03")),
    ]);
    let episodes_2 = HashMap::from([
        ("The Work Outing", date("2006-08-24")),
        ("Return of the Golden Child", date("2007-08-31")),
        ("Moss and the German", date("2007-09-07")),
        ("The Dinner Party", date("2007-09-14")),
        ("Smoke and Mirrors", date("2007-09-21")),
        ("Men Without Women", date("2007-09-28")),
    ]);
    let episodes_3 = HashMap::from([
        ("From Hell", date("2008-11-21")),
        ("Are We Not Men?", date("2008-11-28")),
        ("Tramps Like Us", date("2008-12-05")),
        ("The Speech", date("2008-12-12")),
        ("Friendface", date("2008-12-19")),
        ("Calendar Geeks", date("2008-12-26")),
    ]);
    let episodes_4 = HashMap::from([
        ("Jen The Fredo", date("2010-06-25")),
        ("The Final Countdown", date("2010-07-02")),
        ("Something Happened", date("2010-07-09")),
        ("Italian For Beginners", date("2010-07-16")),
        ("Bad Boys", date("2010-07-23")),
        ("Reynholm vs Reynholm", date("2010-07-30")),
    ]);
    let episodes_data = vec![episodes_1, episodes_2, episodes_3, episodes_4];
    let mut episodes = Vec::new();

    for i in 0..4 {
        for (k, v) in &episodes_data[i] {
            episodes.push(ydb_struct!(
                "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
                "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[i]))),
                "episode_id" => Value::Bytes(Bytes::from(Uuid::new_v4().to_string())),
                "title" => (*k),
                "air_date" => Value::Date((*v)),
            ))
        }
    }

    let series_example = ydb_struct!(
        "series_id" => Value::Bytes(vec![].into()),
        "title" => "",
        "series_info" => "",
        "release_date" => Value::Date(date("2006-02-21")),
        "comment" => ""
    );

    let season_example = ydb_struct!(
        "series_id" => Value::Bytes(vec![].into()),
        "season_id" => Value::Bytes(vec![].into()),
        "title" => "",
        "first_aired"=> Value::Date(date("2006-02-21")),
        "last_aired"=> Value::Date(date("2006-02-21")),
    );

    let episode_example = ydb_struct!(
        "series_id" => Value::Bytes(vec![].into()),
        "season_id" => Value::Bytes(vec![].into()),
        "episode_id" => Value::Bytes(vec![].into()),
        "title" => "",
        "air_date"=> Value::Date(date("2006-02-21")),
    );

    let list_series = Value::list_from(series_example, vec![series])?;
    let list_seasons = Value::list_from(season_example, seasons)?;
    let list_episodes = Value::list_from(episode_example, episodes)?;

    Ok((list_series, list_seasons, list_episodes))
}

pub fn get_data_for_silicon_valley() -> YdbResult<(Value, Value, Value)> {
    let series_id = Uuid::new_v4().to_string();
    let series = ydb_struct!(
        "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
        "title" => "Silicon Valley",
        "series_info" => "Silicon Valley is an American comedy television series created by Mike Judge, John Altschuler and \
         Dave Krinsky. The series focuses on five young men who founded a startup company in Silicon Valley.",
        "release_date" => Value::Date(date("2014-04-06")),
        "comment" => ""
    );

    let seasons_ids: Vec<String> = (0..5).map(|_| Uuid::new_v4().to_string()).collect();

    let seasons = vec![
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[0]))),
            "title"=> "Season 1",
            "first_aired"=> Value::Date(date("2006-02-03")),
            "last_aired"=> Value::Date(date("2006-03-03")),
        ),
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[1]))),
            "title"=> "Season 2",
            "first_aired"=> Value::Date(date("2007-08-24")),
            "last_aired"=> Value::Date(date("2007-09-28")),
        ),
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[2]))),
            "title"=> "Season 3",
            "first_aired"=> Value::Date(date("2008-11-21")),
            "last_aired"=> Value::Date(date("2008-12-26")),
        ),
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[3]))),
            "title"=> "Season 4",
            "first_aired"=> Value::Date(date("2010-06-25")),
            "last_aired"=> Value::Date(date("2010-07-30")),
        ),
        ydb_struct!(
            "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
            "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[3]))),
            "title"=> "Season 5",
            "first_aired"=> Value::Date(date("2010-06-25")),
            "last_aired"=> Value::Date(date("2010-07-30")),
        ),
    ];

    let episodes_1 = HashMap::from([
        ("Minimum Viable Product", date("2014-04-06")),
        ("The Cap Table", date("2014-04-13")),
        ("Articles of Incorporation", date("2014-04-20")),
        ("Fiduciary Duties", date("2014-04-27")),
        ("Signaling Risk", date("2014-05-04")),
        ("Third Party Insourcing", date("2014-05-11")),
        ("Proof of Concept", date("2014-05-18")),
        ("Optimal Tip-to-Tip Efficiency", date("2014-06-01")),
    ]);
    let episodes_2 = HashMap::from([
        ("Sand Hill Shuffle", date("2015-04-12")),
        ("Runaway Devaluation", date("2015-04-19")),
        ("Bad Money", date("2015-04-26")),
        ("The Lady", date("2015-05-03")),
        ("Server Space", date("2015-05-10")),
        ("Homicide", date("2015-05-17")),
        ("Adult Content", date("2015-05-24")),
        ("White Hat/Black Hat", date("2015-05-31")),
        ("Binding Arbitration", date("2015-06-07")),
        ("Two Days of the Condor", date("2015-06-14")),
    ]);

    let episodes_3 = HashMap::from([
        ("Founder Friendly", date("2016-04-24")),
        ("Two in the Box", date("2016-05-01")),
        ("Meinertzhagen's Haversack", date("2016-05-08")),
        ("Maleant Data Systems Solutions", date("2016-05-15")),
        ("The Empty Chair", date("2016-05-22")),
        ("Bachmanity Insanity", date("2016-05-29")),
        ("To Build a Better Beta", date("2016-06-05")),
        ("Bachman's Earnings Over-Ride", date("2016-06-12")),
        ("Daily Active Users", date("2016-06-19")),
        ("The Uptick", date("2016-06-26")),
    ]);

    let episodes_4 = HashMap::from([
        ("Success Failure", date("2017-04-23")),
        ("Terms of Service", date("2017-04-30")),
        ("Intellectual Property", date("2017-05-07")),
        ("Teambuilding Exercise", date("2017-05-14")),
        ("The Blood Boy", date("2017-05-21")),
        ("Customer Service", date("2017-05-28")),
        ("The Patent Troll", date("2017-06-04")),
        ("The Keenan Vortex", date("2017-06-11")),
        ("Hooli-Con", date("2017-06-18")),
        ("Server Error", date("2017-06-25")),
    ]);

    let episodes_5 = HashMap::from([
        ("Grow Fast or Die Slow", date("2018-03-25")),
        ("Reorientation", date("2018-04-01")),
        ("Chief Operating Officer", date("2018-04-08")),
        ("Tech Evangelist", date("2018-04-15")),
        ("Facial Recognition", date("2018-04-22")),
        ("Artificial Emotional Intelligence", date("2018-04-29")),
        ("Initial Coin Offering", date("2018-05-06")),
        ("Fifty-One Percent", date("2018-05-13")),
    ]);

    let episodes_data = vec![episodes_1, episodes_2, episodes_3, episodes_4, episodes_5];
    let mut episodes = Vec::new();

    for i in 0..5 {
        for (k, v) in &episodes_data[i] {
            episodes.push(ydb_struct!(
                "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
                "season_id" => Value::Bytes(Bytes::from(&(*seasons_ids[i]))),
                "episode_id" => Value::Bytes(Bytes::from(Uuid::new_v4().to_string())),
                "title" => (*k),
                "air_date" => Value::Date(*v),
            ))
        }
    }

    let series_example = ydb_struct!(
        "series_id" => Value::Bytes(vec![].into()),
        "title" => "",
        "series_info" => "",
        "release_date" => Value::Date(date("2006-02-21")),
        "comment" => ""
    );

    let season_example = ydb_struct!(
        "series_id" => Value::Bytes(vec![].into()),
        "season_id" => Value::Bytes(vec![].into()),
        "title" => "",
        "first_aired"=> Value::Date(date("2006-02-21")),
        "last_aired"=> Value::Date(date("2006-02-21")),
    );

    let episode_example = ydb_struct!(
        "series_id" => Value::Bytes(vec![].into()),
        "season_id" => Value::Bytes(vec![].into()),
        "episode_id" => Value::Bytes(vec![].into()),
        "title" => "",
        "air_date"=> Value::Date(date("2006-02-21")),
    );

    let list_series = Value::list_from(series_example, vec![series])?;
    let list_seasons = Value::list_from(season_example, seasons)?;
    let list_episodes = Value::list_from(episode_example, episodes)?;

    Ok((list_series, list_seasons, list_episodes))
}

fn date(date_str: &str) -> SystemTime {
    const DATE_ISO8601: &str = "%Y-%m-%d";
    let datetime = chrono::NaiveDate::parse_from_str(date_str, DATE_ISO8601)
        .unwrap_or_else(|_| panic!("Invalid date format: {}", date_str))
        .and_hms_opt(0, 0, 0)
        .unwrap();

    SystemTime::UNIX_EPOCH + Duration::from_secs(datetime.timestamp() as u64)
}

fn main() {}
