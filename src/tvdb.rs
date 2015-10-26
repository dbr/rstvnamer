extern crate xmltree;

use std::{io, error};
use std::io::Cursor;

use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

use hyper::Url;

use super::utils::intify;


/// Errors in contacting TheTVDB
#[derive(Debug)]
pub enum TvdbError {
    SeriesNotFound,
    CommunicationError{reason: String},
}

#[derive(Debug,Clone)]
pub struct SeriesSearchResult{
    id: i32, // TODO: Is this any different to seriesid?
    seriesname: String,
    language: String,
    overview: Option<String>,
    banner: Option<String>,
    imdb_id: Option<String>,
    //firstaired: Date,
    network: Option<String>,
    zap2it_id: Option<String>,
}


pub struct ConsoleInput;
impl ConsoleInput{
    pub fn new() -> ConsoleInput{
        ConsoleInput
    }
}

pub trait SeriesSelector {
    fn select(self, results: &Vec<SeriesSearchResult>) -> SeriesSearchResult;
}

impl SeriesSelector for ConsoleInput{
    fn select(self, results: &Vec<SeriesSearchResult>) -> SeriesSearchResult{
        return results[0].clone();
    }
}

pub fn series_search<T: SeriesSelector>(series: &str, selector: T) -> Result<SeriesSearchResult, TvdbError>{
    let client = Client::new();

    let formatted_url = format!("http://thetvdb.com/api/GetSeries.php?seriesname={}", series);
    let url = Url::parse(&formatted_url).ok().expect("invalid URL");
    println!("Getting {}", url);

    let mut res = client.get(url)
        .header(Connection::close())
        .send();

    let mut res = match res {
        Err(e) => return Err(TvdbError::CommunicationError{reason: "Error contacting TVDB".to_owned()}), // FIXME: http://stackoverflow.com/questions/28911833/error-handling-best-practices
        Ok(r) => r
    };

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    //println!("{}", body);

    let tree = xmltree::Element::parse(body.as_bytes());


    let mut results : Vec<SeriesSearchResult> = vec![];

    for child in tree.children.iter(){

        fn get_text(child: &xmltree::Element, x: &str) -> Option<String>{
            return match child.get_child(x) {
                Some(id_child) => Some(id_child.text.clone().unwrap_or("".to_owned())),
                None => None,
            }
        }

        let r = SeriesSearchResult{
            id:         intify(&get_text(child, "id").unwrap()),
            seriesname: get_text(child, "SeriesName").expect("Missing SeriesName"),
            language:   get_text(child, "language").expect("Missing language"),
            overview:   get_text(child, "Overview"),
            banner:     get_text(child, "banner"),
            imdb_id:    get_text(child, "IMDB_ID"),
            //firstaired: Date,
            network:    get_text(child, "Network"),
            zap2it_id:  get_text(child, "zap2it_id"),
        };

        results.push(r);
    }

    return Ok(selector.select(&results));
    //return Err(TvdbError::SeriesNotFound);
}
