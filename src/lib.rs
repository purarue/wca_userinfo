use std::error::Error;

use scraper::{Html, Selector};
use serde::Serialize;
use simple_error::bail;

pub type ParseResult<T> = Result<T, Box<dyn Error>>;

static EXPECT_EVENT_TDS: usize = 10;

// CSS selectors
static PERSON_INFORMATION_SELECTOR_STR: &str = "div.details table.table tbody tr td";
static RECORD_ROWS_SELECTOR_STR: &str = "div.personal-records table.table tbody tr";
static TD_SELECTOR_STR: &str = "td";

// Helper functions for parsing/validating RecordGroups

/// parse time into a string or nothing, if its empty
fn parse_time(val: &str) -> Option<String> {
    if val.is_empty() {
        None
    } else {
        Some(val.to_owned())
    }
}

/// is string is empty, that represents user doesn't have a record for it
/// else, try and parse the record or error out on IntParseError
fn parse_int(val: &str) -> ParseResult<Option<u32>> {
    if val.is_empty() {
        Ok(None)
    } else {
        match val.parse::<u32>() {
            Ok(int) => Ok(Some(int)),
            Err(_) => bail!("Could not parse integer: {}", val),
        }
    }
}

/// A group for Single/Average results for an event
#[derive(Serialize)]
pub struct RecordGroup {
    pub time: Option<String>, // use String instead of float to maintain precision
    pub national: Option<u32>,
    pub continent: Option<u32>,
    pub world: Option<u32>,
}

impl RecordGroup {
    /// constructor
    pub fn from_str_values(
        time: &str,
        national: &str,
        continent: &str,
        world: &str,
    ) -> ParseResult<RecordGroup> {
        Ok(RecordGroup {
            time: parse_time(&String::from(time)),
            national: parse_int(national)?,
            continent: parse_int(continent)?,
            world: parse_int(world)?,
        })
    }
}

/// the name, single and average information for one cubing event
#[derive(Serialize)]
pub struct Event {
    pub name: String,
    pub single: RecordGroup,
    pub average: RecordGroup,
}

impl Event {
    /// Parse a tr row for an event into the Event struct
    pub fn from_tr_vector(tr_vec: &[String]) -> ParseResult<Event> {
        // make sure number of td's is the expected amount
        if tr_vec.len() != EXPECT_EVENT_TDS {
            bail!("Parse Error: Didn't find expected amount of td's in an event row. Expected {}, found {}", EXPECT_EVENT_TDS, tr_vec.len())
        }

        Ok(Event {
            name: tr_vec[0].to_owned(),
            single: RecordGroup::from_str_values(&tr_vec[4], &tr_vec[1], &tr_vec[2], &tr_vec[3])?,
            average: RecordGroup::from_str_values(&tr_vec[5], &tr_vec[8], &tr_vec[7], &tr_vec[6])?,
        })
    }
}

#[derive(Serialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Gender {
    pub fn from_string(gender_str: &str) -> Gender {
        match gender_str {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Other,
        }
    }
}

/// information scraped from wca; serializes to JSON response
#[derive(Serialize)]
pub struct UserInfo {
    pub country: String,
    pub wca_id: String,
    pub gender: Option<Gender>,
    pub competitions: u32,
    pub completed_solves: u32,
    pub events: Vec<Event>,
}

/// makes a request to url and returns a wrapped response
pub fn get_page_contents(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<String, reqwest::Error> {
    let response = client.get(url).send()?.error_for_status();
    match response {
        Ok(resp) => Ok(resp.text()?),
        Err(err) => Err(err),
    }
}

/// parses the HTML page into a UserInfo struct
pub fn parse_html(body: &str) -> ParseResult<UserInfo> {
    let document = Html::parse_document(body);

    // selectors
    let person_information_selector = Selector::parse(PERSON_INFORMATION_SELECTOR_STR).unwrap();
    let record_rows_selector = Selector::parse(RECORD_ROWS_SELECTOR_STR).unwrap();
    let td_selector = Selector::parse(TD_SELECTOR_STR).unwrap();

    // parse user information
    let mut user_information = document
        .select(&person_information_selector)
        .map(|td| {
            td.text() // text iterator
                .next() // text element
                .unwrap_or("")
                .trim() // remove extra space (from icons)
        })
        .collect::<Vec<&str>>();

    // make sure that we got information from each td
    for info in &user_information {
        if info.is_empty() {
            bail!("Parse Error: Did not get user information for one or more items.")
        }
    }

    // make sure we got the correct amount of items
    // if gender is missing, there is no td for it
    if user_information.len() < 4 || user_information.len() > 5 {
        bail!("Parse Error: Didn't find expected amount of td's as user information. Expected 4 or 5, found {}", user_information.len())
    }

    // parse event records
    let event_trs: ParseResult<Vec<Event>> = document
        .select(&record_rows_selector)
        .map(|tr| // for each tr (event)
        tr.select(&td_selector).map( // for each td (item in row)
            |td|
            // iterate and join child text nodes
            td.text().collect::<Vec<&str>>().join("").trim().to_owned()
        ).collect::<Vec<String>>())
        .map(|tr_vec| // convert vec to events
        Event::from_tr_vector(&tr_vec))
        .collect();

    // check if we failed to parse into Events
    if let Err(e) = event_trs {
        return Err(e);
    };

    let gender: Option<Gender> = match user_information.len() {
        5 => Some(Gender::from_string(&user_information.remove(2))),
        _ => None,
    };

    Ok(UserInfo {
        country: user_information[0].to_owned(),
        wca_id: user_information[1].to_owned(),
        gender,
        // safe to unwrap since we checked length of strings/validated above
        competitions: parse_int(user_information[2])?.unwrap(),
        completed_solves: parse_int(user_information[3])?.unwrap(),
        events: event_trs.ok().unwrap(),
    })
}

//  ispell
//  LocalWords:  reqwest Html TDS usize fn url str Ok dyn html tbody td Vec len
//  LocalWords:  td's trs vec wca ispell enum JSON
//  LocalWords:  TDS usize struct impl fn str IntParseError Ok vec td's len wca
