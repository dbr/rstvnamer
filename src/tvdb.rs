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
    DataError{reason: String},
    Cancelled,
}

#[derive(Debug,Clone)]
pub struct SeriesSearchResult{
    pub id: i32, // TODO: Is this any different to seriesid?
    pub seriesname: String,
    pub language: String,
    pub overview: Option<String>,
    pub banner: Option<String>,
    pub imdb_id: Option<String>,
    //pub firstaired: Date,
    pub network: Option<String>,
    pub zap2it_id: Option<String>,
}


pub struct ConsoleInput;
impl ConsoleInput{
    pub fn new() -> ConsoleInput{
        ConsoleInput
    }
}

pub trait SeriesSelector {
    fn select(self, results: &Vec<SeriesSearchResult>) -> Option<SeriesSearchResult>;
}

impl SeriesSelector for ConsoleInput{
    fn select(self, results: &Vec<SeriesSearchResult>) -> Option<SeriesSearchResult>{
        return Some(results[0].clone());
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

    // Parse XML
    let tree = xmltree::Element::parse(body.as_bytes());

    // Convert XML into structs
    let mut results : Vec<SeriesSearchResult> = vec![];

    for child in tree.children.iter(){

        fn get_text(child: &xmltree::Element, x: &str) -> Option<String>{
            return match child.get_child(x) {
                Some(id_child) => Some(id_child.text.clone().unwrap_or("".to_owned())),
                None => None,
            }
        }

        //try!(get_text(child, "bannana").ok_or(TvdbError::DataError{reason: "No child 'blah' found".to_owned()}));
        let r = SeriesSearchResult{
            id:         intify(&get_text(child, "id").expect("Search result XML missing 'id' element")),
            seriesname: get_text(child, "SeriesName").expect("Search result XML Missing 'SeriesName' element"),
            language:   get_text(child, "language").expect("Search result XML missing 'language' element"),
            overview:   get_text(child, "Overview"),
            banner:     get_text(child, "banner"),
            imdb_id:    get_text(child, "IMDB_ID"),
            //firstaired: Date,
            network:    get_text(child, "Network"),
            zap2it_id:  get_text(child, "zap2it_id"),
        };

        results.push(r);
    }

    // Select UI
    return selector.select(&results).ok_or(TvdbError::Cancelled);
}
