extern crate exif;
use std::fs;
use exif::{Reader, In, Tag};
use chrono::prelude::*;
use std::fs::File;
use crate::dump_file::dump_file;
use std::path::{PathBuf};
use std::io::BufReader;

//  Use to get list of sorted files in the directory list in vector format
// input is the directory and output is error number, error string and model
pub fn get_fromdirlistm (current_dir: PathBuf) -> (u32, String, Vec<String>) {
    let errcode: u32;
    let errstring: String;
    let mut listitems: Vec<String> = Vec::new();
    let mut numentry = 0;
    for entry1 in fs::read_dir(&current_dir).unwrap() {
         let entry = entry1.unwrap();
         if let Ok(metadata) = entry.metadata() {
             if let Ok(file_name) = entry.file_name().into_string() {
                 if metadata.is_file() {
                     if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                         file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                         file_name.ends_with(".png") |file_name.ends_with(".PNG") { 
// get the date (dt date taken, id image date, fd file date) and add to name
                          let datetime: DateTime<Local> = metadata.modified().unwrap().into();
                          let mut file_date = format!("{}", datetime.format("%Y-%m-%d %T"));
                          let mut date_from = format!("fd");
                          let file_path = entry.path();
                          if let Err(_e) = dump_file(&file_path) {
                          } else {
                              let file = File::open(file_path).unwrap();
                              let reader = Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                              if let Some(field1) = reader.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                                  file_date = format!("{}",field1.value.display_as(field1.tag));
                                  date_from = format!("dt");
                              } else {
                                  if let Some(field2) = reader.get_field(Tag::DateTime, In::PRIMARY) {
                                      file_date = format!("{}",field2.value.display_as(field2.tag));
                                      date_from = format!("id");
                                  }
                              }
                          }
                          let listival = file_name + " | " + &date_from + "-" + &file_date;
                          listitems.push(listival);
                          numentry = numentry + 1;
                    }
                 }
             }
         }
    }
    if numentry > 0 {
        listitems.sort();
        errstring = format!("{} files in directory ", numentry);
        errcode = 0;
    } else {
        errstring = "********* Directory 1: directory has no image files (jpg or png) **********".to_string();
        errcode = 1;
    }
    (errcode, errstring, listitems)
}

