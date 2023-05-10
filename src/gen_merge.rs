extern crate chrono;
use chrono::offset::LocalResult;

use chrono::prelude::*;
use chrono::{Duration, Utc};
// gen: datenumother, mbeforebox, datenumto--, bolusenum--, dateto--, dateother

// function called by build_ui
//  Use to find date to insert into directory
// input is datenumother, mbeforebox, datenumto, dateto, dateother and output is error number, error string, bolusenum, datenumtox, datetox
pub fn gen_merge (datenumother: i32, mbeforebox_check: bool, datenumto: i32, dateto: LocalResult<DateTime<Utc>>, dateother:  LocalResult<DateTime<Utc>>) -> (u32, String, bool, i32,  LocalResult<DateTime<Utc>>) {
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
    let mut datenumtox: i32 = 0;
    let mut bolusenum = false;
    let mut datetox = dateother.clone();
// test if no previous or after file
    if datenumother > 999 {
        if mbeforebox_check {
            if datenumto < 2 {
// change to reduce date by one sec and make 500 dateto - Duration::seconds(1)
                datetox = chrono::LocalResult::Single(dateto.unwrap() - Duration::seconds(1));
                datenumtox = 500;
//               errstring = "<span color=\"#FF000000\">********* Merge: selected file number is less than 2, first file and before is checked **********</span>".to_string();
//                errcode = 1
            } else {
                datenumtox = datenumto - (datenumto/2);
                bolusenum = true;
            }
        } else {
// change to add date by one sec and make 500 dateto + Duration::seconds(1)
            if datenumto > 998 {
                datetox = chrono::LocalResult::Single(dateto.unwrap() + Duration::seconds(1));
                datenumtox = 500;
//                errstring = "<span color=\"#FF000000\">********* Merge: selected file number is less than 2, first file and before is checked **********</span>".to_string();
//                errcode = 2;
            } else {
                datenumtox = datenumto + ((1000 - datenumto)/2);
                bolusenum = true;
            }
        }
    } else {
// see if same date
        if dateto == dateother {
            if mbeforebox_check {
                if (datenumto - datenumother) < 2 {
                    errstring = "<span color=\"#FF000000\">********* Merge: before checked and selected file number and previous file number less than 2 apart **********</span>".to_string();
                    errcode = 3;
                } else {
                    datenumtox = datenumto - ((datenumto - datenumother)/2);
                    bolusenum = true;
                }
            } else {
                if (datenumother - datenumto) < 2 {
                    errstring = "<span color=\"#FF000000\">********* Merge: selected file number and next file number less than 2 apart **********</span>".to_string();
                    errcode = 4;
                } else {
                    datenumtox = datenumto + ((datenumother - datenumto)/2);
                    bolusenum = true;
                }
            }
        } else {
// dates not the same
            if mbeforebox_check {
                let duration = dateto.unwrap().signed_duration_since(dateother.unwrap());
                if duration.num_seconds() < 2 {
                    if datenumto > 1 {
                        datenumtox = datenumto - (datenumto/2);
                        bolusenum = true;
                    } else if datenumto == 1 {
                        datenumtox = 0;
                        bolusenum = true;
                    }
                }
            } else {                                
                let durationx = dateother.unwrap().signed_duration_since(dateto.unwrap());
                if durationx.num_seconds() < 2 {
                    if datenumto < 999 {
                        datenumtox = datenumto + ((1000 - datenumto)/2);
                        bolusenum = true;
                    }
                }                               
            }
            if !bolusenum {
                let mut durationn = dateother.unwrap().signed_duration_since(dateto.unwrap());
                if mbeforebox_check {
                    durationn = dateto.unwrap().signed_duration_since(dateother.unwrap());
                }
                if durationn.num_seconds() < 2 {
                    if mbeforebox_check {
                        if datenumother > 998 {
                            errstring = "<span color=\"#FF000000\">********* Merge: before checked and selected file number and previous file number too close **********</span>".to_string();
                            errcode = 5;
                        } else {
                            datetox = dateother;
                            datenumtox = datenumother + ((1000 - datenumother)/2);
                        }
                    } else {
                        if datenumother < 1 {
                            errstring = "<span color=\"#FF000000\">********* Merge: selected file number and after file number too close **********</span>".to_string();
                            errcode = 6;
                        } else {
                            datetox = dateother;
                            if datenumother > 1 {
                                datenumtox = datenumother - (datenumother/2);
                            } else {
                                datenumtox = 0;
                            }
                        }
                    }
                } else {
                    datenumtox = 500;
                    let sec = durationn.num_seconds();
                    if mbeforebox_check {
                        datetox = chrono::LocalResult::Single(dateother.unwrap() + Duration::seconds(sec/2));
                    } else {
                        datetox = chrono::LocalResult::Single(dateother.unwrap() - Duration::seconds(sec/2));
                    }
                }
            }
        }
    }
    (errcode, errstring, bolusenum, datenumtox, datetox)
}
