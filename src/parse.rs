use std::error::Error;

use reqwest;
use scraper::{Html, Selector};
use simple_error::*;

use crate::model;

static EXPECT_USER_INFO_TDS: usize = 5;

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

pub type ParseResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_html(body: &str) -> ParseResult<model::UserInfo> {
    let document = Html::parse_document(&body);

    // selectors
    let person_information_selector =
        Selector::parse("div.details table.table tbody tr td").unwrap();
    let record_rows_selector =
        Selector::parse("div.personal-records table.table tbody tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    // parse user information
    let user_information: Vec<String> = document
        .select(&person_information_selector)
        .map(|td| {
            String::from(
                td.text() // text iterator
                    .next() // text element
                    .unwrap_or("")
                    .trim(), // remove extra space (from icons)
            )
        })
        .collect();

    // make sure that we got information from each td
    for info in &user_information {
        if info.trim().is_empty() {
            bail!("Parse Error: Did not get user information for one or more items.")
        }
    }

    // make sure we got the correct amount of items
    if user_information.len() != EXPECT_USER_INFO_TDS {
        bail!(format!("Parse Error: Didn't find expected amount of td's as user information. Expected {}, found {}", EXPECT_USER_INFO_TDS, user_information.len()))
    }

    // parse event records
    let event_trs: ParseResult<Vec<model::Event>> = document
        .select(&record_rows_selector)
        .map(|tr| // for each tr (event)
        tr.select(&td_selector).map( // for each td (item in row)
            |td|
            // iterate and join child text nodes
            String::from(
                td.text().collect::<Vec<&str>>().join("").trim()
            )
        ).collect::<Vec<String>>())
        .map(|tr_vec| // convert vec to events
        model::Event::from_tr_vector(&tr_vec))
        .collect();

    // check if we failed to parse into Events
    if let Err(e) = event_trs {
        return Err(e);
    };

    Ok(model::UserInfo {
        country: user_information[0].to_owned(),
        wca_id: user_information[1].to_owned(),
        gender: model::Gender::from_string(&user_information[2]),
        competitions: user_information[3].parse::<u32>().unwrap(),
        completed_solves: user_information[4].parse::<u32>().unwrap(),
        events: event_trs.ok().unwrap(),
    })
}

//  ispell
//  LocalWords:  reqwest Html TDS usize fn url str Ok dyn html tbody td Vec len
//  LocalWords:  td's trs vec wca
