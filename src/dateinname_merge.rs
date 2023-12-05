extern crate chrono;
use chrono::offset::LocalResult;
use chrono::prelude::*;
use chrono::Utc;
// gen: datenumother, mbeforebox, datenumto--, bolusenum--, dateto--, dateother

// function called by build_ui
//  Use to find date to insert into directory
// input is datenumother, mbeforebox, datenumto, dateto, dateother and output is error number, error string, bolusenum, datenumtox, datetox
pub fn dateinname_merge (fromfilename: String, datenumother: i32, mbeforebox_check: bool, datenumto: i32, dateto: LocalResult<DateTime<Utc>>, dateother:  LocalResult<DateTime<Utc>>) -> (u32, String, bool, i32,  LocalResult<DateTime<Utc>>) {
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
    let mut datenumtox: i32 = 0;
    let bolusenum = false;
    let mut datetox = dateother.clone();
    let mut dateyr = 0;
    let mut datemo = 0;
    let mut dateday = 0;
    let mut datehr = 0;
    let mut datemin = 0;
    let mut datesec = 0;
    let mut datenum = 0;
    let mut datefile: LocalResult<DateTime<Utc>> = chrono::LocalResult::Single(Utc.with_ymd_and_hms(2000,1,1,1,1,1).unwrap());

// extract date from file name
//    let fileln = fromfilename.len();
//    let fileend = fileln - 2;
//    let filestart = 6;
//    let fromfilenamex = fromfilename.get(filestart..fileend).unwrap();
    let mut baddate1 = 0;
    if fromfilename.len() < 25 {
        baddate1 = 1;
    } else {
// date in name start
        let date1ar2: Vec<&str> = fromfilename[0..23].split("_").collect();
        let lendat2 = date1ar2.len();
        for indl in 0..lendat2 {
             let date_int: i32 = date1ar2[indl].parse().unwrap_or(-9999);
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
// date in name end
    if baddate1 != 0 {
        errstring = "selected file name is does not have a valid date in name".to_string();
        errcode = 1
    } else {
        datefile = Utc.with_ymd_and_hms(dateyr, datemo, dateday, datehr as u32, datemin as u32, datesec as u32);
// no before or after file
        if datenumother > 999 {
            if mbeforebox_check {
                if dateto == datefile {
                    if datenum <! datenumto {
                        errstring = "selected file number not less than place of insertion".to_string();
                        errcode = 2;
                    }
                } else {
                    let duration = dateto.unwrap().signed_duration_since(datefile.unwrap());
                    if duration.num_seconds() < 0 {
                        errstring = "selected file date not less than place of insertion".to_string();
                        errcode = 3;
                    }
                }
            } else {
                if dateto == datefile {
                    if datenum >! datenumto {
                        errstring = "selected file number not greater than place of insertion".to_string();
                        errcode = 4;
                    }
                } else {
                    let duration = datefile.unwrap().signed_duration_since(dateto.unwrap());
                    if duration.num_seconds() < 0 {
                        errstring = "selected file date not greater than place of insertion".to_string();
                        errcode = 5;
                    }
                }
            }
// have before or after file
        } else {
            if mbeforebox_check {
                if dateother == datefile {
                    if datenum >! datenumother {
                        errstring = "selected file date number will not go into place of insertion 5".to_string();
                        errcode = 5;
                    } else {
                        if dateto == datefile {
                            if datenum <! datenumto {
                                errstring = "selected file date number will not go into place of insertion 6".to_string();
                                errcode = 6;
                            }
                        } else {
                            let duration = dateto.unwrap().signed_duration_since(datefile.unwrap());
                            if duration.num_seconds() < 0 {
                                errstring = "selected file date number will not go into place of insertion 7".to_string();
                                errcode = 7;
                            }
                        }
                    }
                } else {
                    let duration = datefile.unwrap().signed_duration_since(dateother.unwrap());
                    if duration.num_seconds() < 0 {
                       errstring = "selected file date number will not go into place of insertion 8".to_string();
                       errcode = 8;
                    } else {
                        if dateto == datefile {
                            if datenum <! datenumto {
                                errstring = "selected file date number will not go into place of insertion 9".to_string();
                                errcode = 9;
                            }
                        } else {
                            let duration = dateto.unwrap().signed_duration_since(datefile.unwrap());
                            if duration.num_seconds() < 0 {
                                errstring = "selected file date number will not go into place of insertion 10".to_string();
                                errcode = 10;
                            }
                        }
                    }
                }
            } else {
                if dateto == datefile {
                    if datenum >! datenumto {
                        errstring = "selected file date number will not go into place of insertion 11".to_string();
                        errcode = 11;
                    } else {
                        if dateother == datefile {
                            if datenum <! datenumother {
                                errstring = "selected file date number will not go into place of insertion 12".to_string();
                                errcode = 12;
                            }
                        } else {
                            let duration = dateother.unwrap().signed_duration_since(datefile.unwrap());
                            if duration.num_seconds() < 0 {
                                errstring = "selected file date number will not go into place of insertion 13".to_string();
                                errcode = 13;
                            }
                        }
                    }
                } else {
                    let duration = datefile.unwrap().signed_duration_since(dateto.unwrap());
                    if duration.num_seconds() < 0 {
                       errstring = "selected file date number will not go into place of insertion 14".to_string();
                       errcode = 14;
                    } else {
                        if dateother == datefile {
                            if datenum <! datenumother {
                                errstring = "selected file date number will not go into place of insertion 15".to_string();
                                errcode = 15;
                            }
                        } else {
                            let duration = dateother.unwrap().signed_duration_since(datefile.unwrap());
                            if duration.num_seconds() < 0 {
                                errstring = "selected file date number will not go into place of insertion 16".to_string();
                                errcode = 16;
                            }
                        }
                    }
                }
            }
        }
    }
    if errcode == 0 {
        datenumtox = datenum;
        datetox = datefile;
    }
    (errcode, errstring, bolusenum, datenumtox, datetox)
}
