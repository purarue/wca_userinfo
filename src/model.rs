use serde::Serialize;

use simple_error::*;

use crate::parse::ParseResult;

static EXPECT_EVENT_TDS: usize = 10;

/// A group for Single/Average results for an event
#[derive(Serialize)]
pub struct RecordGroup {
    pub time: Option<String>, // use String instead of float to maintain precision
    pub national: Option<u32>,
    pub continent: Option<u32>,
    pub world: Option<u32>,
}

impl RecordGroup {
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

    /// constructor
    pub fn from_str_values(
        time: &str,
        national: &str,
        continent: &str,
        world: &str,
    ) -> ParseResult<RecordGroup> {
        Ok(RecordGroup {
            time: RecordGroup::parse_time(&String::from(time)),
            national: RecordGroup::parse_int(&national)?,
            continent: RecordGroup::parse_int(&continent)?,
            world: RecordGroup::parse_int(&world)?,
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
            name: tr_vec[0].clone(),
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
    pub gender: Gender,
    pub competitions: u32,
    pub completed_solves: u32,
    pub events: Vec<Event>,
}

//  ispell
//  LocalWords:  TDS usize struct impl fn str IntParseError Ok vec td's len wca
//  LocalWords:  enum JSON
