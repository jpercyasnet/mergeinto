extern crate exif;
extern crate chrono;
use std::fs;
use chrono::prelude::*;
use chrono::offset::LocalResult;

pub fn todirrefreshm(new_dir:String) -> (i32, String, Vec<String>) { 
     let errstring: String;
     let mut errcode = 0;
     let mut numentry = 0;
     let mut baddate1 = 0;
     let mut dateyr = 0;
     let mut datemo = 0;
     let mut dateday = 0;
     let mut datehr = 0;
     let mut datemin = 0;
     let mut datesec = 0;
     let mut datenum = 0;
     let mut listitems: Vec<String> = Vec::new();
// loop thru directory looking for jpg and png files
// these files must have names with prefix yyyy_mm_dd_hh_mm_ss_nnn_
     for entry1 in fs::read_dir(&new_dir).unwrap() {
              if baddate1 == 0 {
                  let entry = entry1.unwrap();
                  if let Ok(metadata) = entry.metadata() {
                      if let Ok(file_name) = entry.file_name().into_string() {
                          if metadata.is_file() {
                              if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                 file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                                 file_name.ends_with(".png") |file_name.ends_with(".PNG") {
                                  if file_name.len() < 27 {
                                      baddate1 = 1;
                                  } else { 
// date from name start
//                                   parse the file name and validate its date
                                      let date1ar2: Vec<&str> = file_name[0..23].split("_").collect();
                                      let lendat2 = date1ar2.len();
                                      for indl in 0..lendat2 {
                                           let date_int: i32 = date1ar2[indl].clone().parse().unwrap_or(-9999);
                                           if date_int == -9999 {
                                               baddate1 = 1;
                                           } else {
                                               match indl {
                                                  0 => dateyr = date_int,
                                                  1 => datemo = date_int as u32,
                                                  2 => dateday = date_int as u32,
                                                  3 => datehr = date_int as i32,
                                                  4 => datemin = date_int as i32,
                                                  5 => datesec = date_int as i32,
                                                  6 => datenum = date_int as i32,
                                                  _ => baddate1 = 1,
                                               }
                                           }
                                      }
                                      if baddate1 == 0 {
                                          let datexx = Local.with_ymd_and_hms(dateyr, datemo, dateday,1,1,1);
                                          if datexx == LocalResult::None {
                                              baddate1 = 1;
                                          } else {
                                              if (datenum < 0) | (datenum > 999) {
                                                  baddate1 = 1;
                                              } else if (datehr < 0) | (datehr > 23) {
                                                  baddate1 = 1;
                                              } else if (datemin < 0) | (datemin > 59) {
                                                  baddate1 = 1;
                                              } else if (datesec < 0) | (datesec > 59) {
                                                  baddate1 = 1;
                                              }
                                         }
                                      }
                                  }
// date from name end
                                  if baddate1 == 0 {
                                      listitems.push(file_name);
                                      numentry = numentry + 1;
                                  }
                              } else {
                                  baddate1 = 1; // not a jpeg or png file
                              }
                          }
                      } else {
                          baddate1 = 1; // error getting file name
                      }
                  } else {
                      baddate1 = 1; // error getting metadata
                  }
              }
     } 
// end of for
     if baddate1 == 1 {
         errcode = 1;
         errstring = "********* ERROR File format is not correct **********".to_string();
     } else {
         if numentry > 0 {
// sort the list then output to model
             listitems.sort();
             errstring = format!("to directory updated");
         } else {
             errcode = 2;
             errstring = "********* get_tomodel: directory has no images **********".to_string();
         }
     }
     (errcode, errstring, listitems)
}

