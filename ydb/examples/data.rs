use std::collections::HashMap;
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use ydb::Value;
use ydb::YdbResult;

fn series_data(
    id: &str,
    released: SystemTime,
    title: &str,
    info: &str,
    comment: Option<&str>,
) -> Value {
    let comment_value = match comment {
        Some(c) => {
            Value::optional_from(Value::Text(c.to_string()), Some(Value::Text(c.to_string())))
                .unwrap()
        }
        None => Value::optional_from(Value::Text("".to_string()), None).unwrap(),
    };

    Value::struct_from_fields(vec![
        (
            "series_id".to_string(),
            Value::Bytes(id.as_bytes().to_vec().into()),
        ),
        ("release_date".to_string(), Value::Date(released)),
        ("title".to_string(), Value::Text(title.to_string())),
        ("series_info".to_string(), Value::Text(info.to_string())),
        ("comment".to_string(), comment_value),
    ])
}

fn season_data(
    series_id: &str,
    season_id: &str,
    title: &str,
    first: SystemTime,
    last: SystemTime,
) -> Value {
    Value::struct_from_fields(vec![
        (
            "series_id".to_string(),
            Value::Bytes(series_id.as_bytes().to_vec().into()),
        ),
        (
            "season_id".to_string(),
            Value::Bytes(season_id.as_bytes().to_vec().into()),
        ),
        ("title".to_string(), Value::Text(title.to_string())),
        ("first_aired".to_string(), Value::Date(first)),
        ("last_aired".to_string(), Value::Date(last)),
    ])
}

fn episode_data(
    series_id: &str,
    season_id: &str,
    episode_id: &str,
    title: &str,
    date: SystemTime,
) -> Value {
    Value::struct_from_fields(vec![
        (
            "series_id".to_string(),
            Value::Bytes(series_id.as_bytes().to_vec().into()),
        ),
        (
            "season_id".to_string(),
            Value::Bytes(season_id.as_bytes().to_vec().into()),
        ),
        (
            "episode_id".to_string(),
            Value::Bytes(episode_id.as_bytes().to_vec().into()),
        ),
        ("title".to_string(), Value::Text(title.to_string())),
        ("air_date".to_string(), Value::Date(date)),
    ])
}

pub fn get_data() -> (Vec<Value>, Vec<Value>, Vec<Value>) {
    let mut series_vec = Vec::new();
    let mut seasons_vec = Vec::new();
    let mut episodes_vec = Vec::new();

    let series_data: HashMap<String, fn(&str) -> (Value, Vec<Value>, Vec<Value>)> =
        HashMap::from([
            (
                Uuid::new_v4().to_string(),
                get_data_for_it_crowd as fn(&str) -> (Value, Vec<Value>, Vec<Value>),
            ),
            (Uuid::new_v4().to_string(), get_data_for_silicon_valley),
        ]);

    for (series_id, fill_func) in series_data {
        let (series_data, seasons_data, episodes_data) = fill_func(&series_id);
        series_vec.push(series_data);
        seasons_vec.extend(seasons_data);
        episodes_vec.extend(episodes_data);
    }

    (series_vec, seasons_vec, episodes_vec)
}

fn get_data_for_it_crowd(series_id: &str) -> (Value, Vec<Value>, Vec<Value>) {
    let series = series_data(
        series_id,
        date("2006-02-03"),
        "IT Crowd",
        "The IT Crowd is a British sitcom produced by Channel 4, written by Graham Linehan, produced by \
         Ash Atalla and starring Chris O'Dowd, Richard Ayoade, Katherine Parkinson, and Matt Berry.",
        None, // NULL comment
    );

    let seasons_data = vec![
        SeasonData {
            title: "Season 1",
            first: date("2006-02-03"),
            last: date("2006-03-03"),
            episodes: HashMap::from([
                ("Yesterday's Jam", date("2006-02-03")),
                ("Calamity Jen", date("2006-02-03")),
                ("Fifty-Fifty", date("2006-02-10")),
                ("The Red Door", date("2006-02-17")),
                ("The Haunting of Bill Crouse", date("2006-02-24")),
                ("Aunt Irma Visits", date("2006-03-03")),
            ]),
        },
        SeasonData {
            title: "Season 2",
            first: date("2007-08-24"),
            last: date("2007-09-28"),
            episodes: HashMap::from([
                ("The Work Outing", date("2006-08-24")),
                ("Return of the Golden Child", date("2007-08-31")),
                ("Moss and the German", date("2007-09-07")),
                ("The Dinner Party", date("2007-09-14")),
                ("Smoke and Mirrors", date("2007-09-21")),
                ("Men Without Women", date("2007-09-28")),
            ]),
        },
        SeasonData {
            title: "Season 3",
            first: date("2008-11-21"),
            last: date("2008-12-26"),
            episodes: HashMap::from([
                ("From Hell", date("2008-11-21")),
                ("Are We Not Men?", date("2008-11-28")),
                ("Tramps Like Us", date("2008-12-05")),
                ("The Speech", date("2008-12-12")),
                ("Friendface", date("2008-12-19")),
                ("Calendar Geeks", date("2008-12-26")),
            ]),
        },
        SeasonData {
            title: "Season 4",
            first: date("2010-06-25"),
            last: date("2010-07-30"),
            episodes: HashMap::from([
                ("Jen The Fredo", date("2010-06-25")),
                ("The Final Countdown", date("2010-07-02")),
                ("Something Happened", date("2010-07-09")),
                ("Italian For Beginners", date("2010-07-16")),
                ("Bad Boys", date("2010-07-23")),
                ("Reynholm vs Reynholm", date("2010-07-30")),
            ]),
        },
    ];

    let mut seasons = Vec::new();
    let mut episodes = Vec::new();

    for season in seasons_data {
        let season_id = Uuid::new_v4().to_string();
        seasons.push(season_data(
            series_id,
            &season_id,
            season.title,
            season.first,
            season.last,
        ));

        for (title, date) in season.episodes {
            episodes.push(episode_data(
                series_id,
                &season_id,
                &Uuid::new_v4().to_string(),
                title,
                date,
            ));
        }
    }

    (series, seasons, episodes)
}

fn get_data_for_silicon_valley(series_id: &str) -> (Value, Vec<Value>, Vec<Value>) {
    let series = series_data(
        series_id,
        date("2014-04-06"),
        "Silicon Valley",
        "Silicon Valley is an American comedy television series created by Mike Judge, John Altschuler and \
         Dave Krinsky. The series focuses on five young men who founded a startup company in Silicon Valley.",
        Some("Some comment here"),
    );

    let seasons_data = vec![
        SeasonData {
            title: "Season 1",
            first: date("2006-02-03"),
            last: date("2006-03-03"),
            episodes: HashMap::from([
                ("Minimum Viable Product", date("2014-04-06")),
                ("The Cap Table", date("2014-04-13")),
                ("Articles of Incorporation", date("2014-04-20")),
                ("Fiduciary Duties", date("2014-04-27")),
                ("Signaling Risk", date("2014-05-04")),
                ("Third Party Insourcing", date("2014-05-11")),
                ("Proof of Concept", date("2014-05-18")),
                ("Optimal Tip-to-Tip Efficiency", date("2014-06-01")),
            ]),
        },
        SeasonData {
            title: "Season 2",
            first: date("2007-08-24"),
            last: date("2007-09-28"),
            episodes: HashMap::from([
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
            ]),
        },
        SeasonData {
            title: "Season 3",
            first: date("2008-11-21"),
            last: date("2008-12-26"),
            episodes: HashMap::from([
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
            ]),
        },
        SeasonData {
            title: "Season 4",
            first: date("2010-06-25"),
            last: date("2010-07-30"),
            episodes: HashMap::from([
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
            ]),
        },
        SeasonData {
            title: "Season 5",
            first: date("2018-03-25"),
            last: date("2018-05-13"),
            episodes: HashMap::from([
                ("Grow Fast or Die Slow", date("2018-03-25")),
                ("Reorientation", date("2018-04-01")),
                ("Chief Operating Officer", date("2018-04-08")),
                ("Tech Evangelist", date("2018-04-15")),
                ("Facial Recognition", date("2018-04-22")),
                ("Artificial Emotional Intelligence", date("2018-04-29")),
                ("Initial Coin Offering", date("2018-05-06")),
                ("Fifty-One Percent", date("2018-05-13")),
            ]),
        },
    ];

    let mut seasons = Vec::new();
    let mut episodes = Vec::new();

    for season in seasons_data {
        let season_id = Uuid::new_v4().to_string();
        seasons.push(season_data(
            series_id,
            &season_id,
            season.title,
            season.first,
            season.last,
        ));

        for (title, date) in season.episodes {
            episodes.push(episode_data(
                series_id,
                &season_id,
                &Uuid::new_v4().to_string(),
                title,
                date,
            ));
        }
    }

    (series, seasons, episodes)
}

struct SeasonData {
    title: &'static str,
    first: SystemTime,
    last: SystemTime,
    episodes: HashMap<&'static str, SystemTime>,
}

fn date(date_str: &str) -> SystemTime {
    const DATE_ISO8601: &str = "%Y-%m-%d";
    let datetime = chrono::NaiveDate::parse_from_str(date_str, DATE_ISO8601)
        .unwrap_or_else(|_| panic!("Invalid date format: {}", date_str))
        .and_hms_opt(0, 0, 0)
        .unwrap();

    SystemTime::UNIX_EPOCH + Duration::from_secs(datetime.timestamp() as u64)
}

// Helper function to create Value lists for batch operations
pub fn create_value_list(values: Vec<Value>) -> YdbResult<Value> {
    if values.is_empty() {
        // For empty lists, we need to provide an example type
        Value::list_from(Value::Bool(false), values)
    } else {
        let example_value = values[0].clone();
        Value::list_from(example_value, values)
    }
}

// The fill_tables_with_data function would depend on your specific YDB client implementation
// Here's a template:

/*
async fn fill_tables_with_data(
    client: &YourYdbClientType,
    prefix: &str,
) -> YdbResult<()> {
    let (series, seasons, episodes) = get_data();

    // Convert to Value lists
    let series_list = create_value_list(series)?;
    let seasons_list = create_value_list(seasons)?;
    let episodes_list = create_value_list(episodes)?;

    // Execute queries using your YDB client
    // This part depends on your specific YDB client API

    Ok(())
}
*/
fn main() {}
