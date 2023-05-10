extern crate chrono;
extern crate regex;

use std::process::Command;
use std::path::{Path};
use std::fs;
use regex::Regex;
use chrono::{Utc, DateTime};
use chrono::prelude::*;
use chrono::offset::LocalResult;

use crate::gen_merge::gen_merge;
use crate::dateinname_merge::dateinname_merge;
use crate::celldatename_merge::celldatename_merge;

pub fn mergepressm (fromstr: String, tostr: String, fromdir: String, todir: String, struse: String, _strdt: String, strab: String) -> (u32, String) {
        let mut errcode: u32 = 0;
        let mut errstring: String = " ".to_string();
        let mut bolok = true;
        let mut bolusenum = false;
        let str_cur_dirfrom: String;
        let mut fromfilename = format!(" ");
        let mut fromdisplaydate = format!(" ");
        let str_cur_dirto: String;
        let mut tofilename = format!(" ");
//        let mut filenamestr = format!(" ");
        let mut filenameother = format!(" ");
        let mut prefixstr = format!(" ");
        let mut dateto: LocalResult<DateTime<Utc>> = chrono::LocalResult::Single(Utc.with_ymd_and_hms(2000,1,1,1,1,1).unwrap());
        let mut dateother: LocalResult<DateTime<Utc>> = chrono::LocalResult::Single(Utc.with_ymd_and_hms(2000,1,1,1,1,1).unwrap());
        let mut datenumto = 0;
        let mut datenumother = 9999;
        let mut dateyr = 0;
        let mut datemo = 0;
        let mut dateday = 0;
        let mut datehr = 0;
        let mut datemin = 0;
        let mut datesec = 0;
// check if both directories exist and they are not equal
        str_cur_dirfrom = fromdir.to_string();
        str_cur_dirto = todir.to_string();
        if str_cur_dirto == str_cur_dirfrom {
            errstring = "********* Merge: FROM DIR AND TO DIR ARE THE SAME DIRECTORY **********".to_string();
            errcode = 1;
            bolok = false;
        } else {
            if !Path::new(&str_cur_dirfrom).exists() {
                errstring = "********* Merge: FROM DIR DOES NOT EXIST **********".to_string();
                errcode = 2;
                bolok = false;
            } else {
                if !Path::new(&str_cur_dirto).exists() {
                    errstring = "********* Merge: TO DIR DOES NOT EXIST **********".to_string();
                    errcode = 3;
                    bolok = false;
                }
            }
        }

// check a from file has been selected and that there is only one selection
        if bolok {
            let lineparse: Vec<&str> = fromstr[0..].split(" | ").collect();
            fromfilename = lineparse[0].clone().to_string();
            let fromdisplaydatea = lineparse[1].clone().to_string();
            fromdisplaydate = fromdisplaydatea[3..].to_string();
                println!("fromfilename: -{}-", fromfilename);
                println!("fromdisplayname: -{}-", fromdisplaydate);
//                let msgstr = format!("{} is the row name selected in From directory", fromfilename);
//                messageval_label.set_text(&msgstr);
            let fullfrom = str_cur_dirfrom.clone() + "/" + &fromfilename.clone();
            if !Path::new(&fullfrom).exists() {
                errstring = format!("*********  ERROR from file {} does not exist **********",fullfrom);
                errcode = 3;
                bolok = false;
            } else {
                tofilename = tostr;
                println!("tofilename: -{}-", tofilename);
               let fullto= str_cur_dirto.clone() + "/" + &tofilename.clone();
               if !Path::new(&fullto).exists() {
                    errstring = format!("********* ERROR to file {} does not exist **********",fullto);
                    errcode = 4;
                    bolok = false;
               }
            }
        }

// get list of file in to directory and get before or after file name
        if bolok {
            let mut numentry = 0;
            let mut listitems: Vec<String> = Vec::new();
            for entry1 in fs::read_dir(&str_cur_dirto).unwrap() {
                 let entry = entry1.unwrap();
                 if let Ok(metadata) = entry.metadata() {
                     if let Ok(file_name) = entry.file_name().into_string() {
                         if metadata.is_file() {
                             if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                                file_name.ends_with(".png") |file_name.ends_with(".PNG") {
                                 listitems.push(file_name);
                                 numentry = numentry + 1;
                             }
                         }
                     }
                 }
            }
            if numentry < 1 {
                errstring = "********* ERROR getting list of files in to directory **********".to_string();
                errcode = 5;
                bolok = false;
            } else {
                listitems.sort();
                let listitemlen = listitems.len();
                let newtoi = listitemlen as i32 ;
                let tofilenamex = tofilename.clone();
                let mut nop: bool;
                let mut noa = true;
                let mut namep = " ";
                let mut namec = " ";
                let mut namea = " ";
                let mut found = 0;
                if newtoi < 2 {
                    if tofilenamex == listitems[0] {
                        namec = &listitems[0];
                        nop = true;
                        found = 1;
                    } else {
                        namep = &listitems[0];
                        nop = false;
                    }
                } else if newtoi < 3 {
                    namep = &listitems[0];
                    nop = false;
                    namec = &listitems[1];
                    if tofilenamex == namep {
                        namea = namec;
                        noa = false;
                        namec = namep;
                        namep = " ";
                        nop = true;
                        found = 1;
                    } else if tofilenamex == namec {
                        found = 1;
                    }
                } else {
                    namep = &listitems[0];
                    nop = false;
                    namec = &listitems[1];
                    if tofilenamex == namep {
                        namea = namec;
                        noa = false;
                        namec = namep;
                        namep = " ";
                        nop = true;
                        found = 1;
                    } else {
                        for indexi in 2..newtoi {
                             if found == 0 {
                                 namea = &listitems[indexi as usize];
                                 noa = false;
                                 if tofilenamex == namec {
                                     found = 1;
                                 } else {
                                     namep = namec;
                                     nop = false;
                                     namec = namea;
                                     namea = " ";
                                     noa = true;
//                                     println!("333 tofilename: {}  namea: {}", tofilename, namea);
                                 }
                             }
                        }
                        if found == 0 {
                            if tofilenamex == namec {
//                                println!("222 tofilename: {}  namec: {}", tofilename, namec);
                                found = 1;
                            }
                        }
                   }
                }
                if found == 0 {
                    errstring = format!("pick {:?},  p {:?}.  c {:?}, a {:?}  not found", tofilenamex, namep, namec, namea);
                    errcode = 6;
                    bolok = false;
                } else {
                    if strab == "before" {
                        if !nop {
                            datenumother = 0;
                            filenameother = namep.to_string();
                        }
                    } else {
                        if !noa {
                            datenumother = 0;
                            filenameother = namea.to_string();
                        }
                    }
                    errstring = format!("pick {:?} - {:?} ,  other {:?} - {:?} found", tofilenamex, datenumto, filenameother, datenumother);
                    println!("a tofilename: {}  filenameother: {}", tofilename, filenameother);
                }
            }
        }

// get dates and validate for to filename and previous or after filename
        if bolok {
            let tofilenamex = tofilename.clone();
// date in name start
            let date1ar2: Vec<&str> = tofilenamex[0..23].split("_").collect();
            let lendat2 = date1ar2.len();
            let mut baddate1 = 0;
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
                        6 => datenumto = date_int as i32,
                        _ => baddate1 = 1,
                     }
                 }
            }
//                    println!("b tofilename: {}  ", tofilename);
            if baddate1 == 0 {
//                     println!("c tofilename: {}  ", tofilename);

                let datexx = Local.with_ymd_and_hms(dateyr, datemo, dateday,1,1,1);
                if datexx == LocalResult::None {
                    baddate1 = 1;
                } else {
                    if (datenumto < 0) | (datenumto > 999) {
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
            if baddate1 == 0 {
//                    println!("d tofilename: {}  ", tofilename);

                dateto = Utc.with_ymd_and_hms(dateyr, datemo, dateday, datehr as u32, datemin as u32, datesec as u32);
                let mut dateyro = 0;
                let mut datemoo = 0;
                let mut datedayo = 0;
                let mut datehro = 0;
                let mut datemino = 0;
                let mut dateseco = 0;
                if datenumother < 1000 {
//                    let filelno = filenameother.len();
//                    let fileendo = filelno - 1;
//                    let filestarto = 1;
//                    let filenamexo = filenameother.get(filestarto..fileendo).unwrap();
                    let filenamexo = filenameother.clone();
                    let date1ar2o: Vec<&str> = filenamexo[0..23].split("_").collect();
                    let lendat2o = date1ar2o.len();
                    for indlo in 0..lendat2o {
                         let date_into: i32 = date1ar2o[indlo].clone().parse().unwrap_or(-9999);
                         if date_into == -9999 {
                             baddate1 = 1;
                         } else {
                             match indlo {
                               0 => dateyro = date_into,
                               1 => datemoo = date_into as u32,
                               2 => datedayo = date_into as u32,
                               3 => datehro = date_into as i32,
                               4 => datemino = date_into as i32,
                               5 => dateseco = date_into as i32,
                               6 => datenumother = date_into as i32,
                               _ => baddate1 = 1,
                             }
                         }
                    }
                    if baddate1 == 0 {
//                       println!("e tofilename: {}  ", tofilename);
                        let dateyy = Local.with_ymd_and_hms(dateyro, datemoo, datedayo,1,1,1);
                        if dateyy == LocalResult::None {
                            baddate1 = 1;
                        } else {
                            if (datenumother < 0) | (datenumother > 999) {
                                baddate1 = 1;
                            } else if (datehro < 0) | (datehro > 23) {
                                baddate1 = 1;
                            } else if (datemino < 0) | (datemino > 59) {
                                baddate1 = 1;
                            } else if (dateseco < 0) | (dateseco > 59) {
                                baddate1 = 1;
                            }
                        }
                    }
                    if baddate1 == 0 {
                        dateother = Utc.with_ymd_and_hms(dateyro, datemoo, datedayo, datehro as u32, datemino as u32, dateseco as u32);
                    }
                }
            }
            if baddate1 == 1 {
                errstring = format!("pick {:?},  num {:?}.  other {:?},  num {:?}  bad date", tofilename, datenumto, filenameother, datenumother);
                errcode = 7;
                bolok = false;
            }
        }
// dateto, datenumto, datenumother < 1000 then dateother
//  separate calls for each option gen, date in name, phone date, displayed date
// gen: datenumother, mbeforebox, datenumto--, bolusenum--, dateto--, dateother
// date in name: extract date from fromfilename determine if right location and can be inserted set datenumto dateto and bolusenum
// phone date: extract date varing column start ie either yyyymmdd-hhmmss or img-yyyymmdd-hhmmss determine right location and can be inserted set datenumto dateto and bolusenum
// displayed date: extract date from fromdisplaydate variable determine right location and can be inserted set datenumto dateto and bolusenum
        if bolok {
//                    println!("f tofilename: {}  ", tofilename);
            let mut bolbefore = false;
            if strab == "before" {
                bolbefore = true;
            }
            if struse == "gen" {
//         	    println!("mgendate active");
                let (errcodea, errstringa, bolusenumx, datenumtox, datetox) = gen_merge(datenumother, bolbefore, datenumto, dateto, dateother);
                if errcodea == 0 {
                    bolusenum = bolusenumx;
                    dateto = datetox;
                    datenumto = datenumtox;
                } else {
                    errstring = errstringa;
                    errcode = errcodea;
                    bolok = false;
                }
            } else if struse == "din" {
//         	    println!("mdateinname active");
                let fromfilenamex = fromfilename.clone();
                let (errcodea, errstringa, bolusenumx, datenumtox, datetox) = dateinname_merge(fromfilenamex, datenumother, bolbefore, datenumto, dateto, dateother);
                if errcodea == 0 {
                    bolusenum = bolusenumx;
                    dateto = datetox;
                    datenumto = datenumtox;
                } else {
                    errstring = errstringa;
                    errcode = errcodea;
                    bolok = false;
                }
            } else if struse == "pdn" {
//         	    println!("mcelldatename active");
                let fromfilenamex = fromfilename.clone();
                let (errcodea, errstringa, bolusenumx, datenumtox, datetox) = celldatename_merge(fromfilenamex, datenumother, bolbefore, datenumto, dateto, dateother);
                if errcodea == 0 {
                    bolusenum = bolusenumx;
                    dateto = datetox;
                    datenumto = datenumtox;
                } else {
                    errstring = errstringa;
                    errcode = errcodea;
                    bolok = false;
                }
            } else if struse == "ddt" {
//         	    println!("mdisplaydate active");
//         	    println!("fromdisplaydate: {}", fromdisplaydate);
                let dateyr = fromdisplaydate.get(0..4).unwrap().to_string();
                let datemo = fromdisplaydate.get(5..7).unwrap().to_string();
                let dateday = fromdisplaydate.get(8..10).unwrap().to_string();
                let datehr = fromdisplaydate.get(11..13).unwrap().to_string();
                let datemin = fromdisplaydate.get(14..16).unwrap().to_string();
                let datesec = fromdisplaydate.get(17..19).unwrap().to_string();
                let displaynamex = format!("{}{}{}_{}{}{}.jpg", dateyr, datemo, dateday, datehr, datemin, datesec);
                let (errcodea, errstringa, bolusenumx, datenumtox, datetox) = celldatename_merge(displaynamex, datenumother, bolbefore, datenumto, dateto, dateother);
                if errcodea == 0 {
                    bolusenum = bolusenumx;
                    dateto = datetox;
                    datenumto = datenumtox;
                } else {
                    errstring = errstringa;
                    errcode = errcodea;
                    bolok = false;
                }
            } else {
                errstring = "********* ERROR no generation type selected: radio error **********".to_string();
                errcode = 8;
                bolok = false;
            }
        }
        if bolok {
//                    println!("g tofilename: {}  ", tofilename);
            if !bolusenum {
                let mut baddate2 = 0;
                let datestr = format!("{}",dateto.unwrap().format("%Y:%m:%d:%T"));
                let date1ar2d: Vec<&str> = datestr.split(":").collect();
                let lendat2d = date1ar2d.len();
                let mut dateyrd = 0;
                let mut datemod = 0;
                let mut datedayd = 0;
                let mut datehrd = 0;
                let mut datemind = 0;
                let mut datesecd = 0;
                for indld in 0..lendat2d {
                     let date_intd: i32 = date1ar2d[indld].clone().parse().unwrap_or(-9999);
                     if date_intd == -9999 {
                         baddate2 = 1;
                     } else {
                         match indld {
                            0 => dateyrd = date_intd,
                            1 => datemod = date_intd as u32,
                            2 => datedayd = date_intd as u32,
                            3 => datehrd = date_intd as u32,
                            4 => datemind = date_intd as u32,
                            5 => datesecd = date_intd as u32,
                            _ => baddate2 = 1,
                         }
                    }
                }
                if baddate2 == 1 {
                    errstring = format!("pick {:?},  num {:?}.  date {:?} bad date format", tofilename, datenumto, dateto.unwrap().format("%Y:%m:%d:%T"));
                    errcode = 9;
                    bolok = false;
                } else {
                    prefixstr = format!("{}_{:02}_{:02}_{:02}_{:02}_{:02}_{:03}_", dateyrd, datemod, datedayd, datehrd, datemind, datesecd, datenumto);
                    errstring = format!("pick {:?},  num {:?}.  date {:?} good format", tofilename, datenumto, prefixstr);
                }
            } else {
//                    println!("h tofilename: {}  ", tofilename);
                let datesubstr = &tofilename[0..19];
                prefixstr = format!("{}_{:03}_", datesubstr, datenumto);
                errstring = format!("pick {:?},  num {:?}.  date {:?} good format", tofilename, datenumto, prefixstr);
            }
        }
        if bolok {
// determine file trailer length based on to file name.
//                    println!("i tofilename: {}  ", tofilename);

            let fileln = tofilename.len();
//            let fileend = fileln - 2;
            let datesubstr = &tofilename[24..fileln];
            let strlento = datesubstr.len();
            let filelnf = fromfilename.len();
//            let fileendf = filelnf - 2;
            let mut datesubstrf: String = fromfilename[0..filelnf].to_owned();
            let strlenfrom = datesubstrf.len();
            if strlenfrom < strlento {
                let mut prefixx: String = "x".to_owned();
                for _numx in 0..(strlento - strlenfrom - 1) {
                     prefixx.push_str("x");
                }
                datesubstrf = format!("{}{}", &prefixx, &datesubstrf);
            } else {
                datesubstrf = datesubstrf[(strlenfrom - strlento)..].to_string();
            }
            let re = Regex::new(r"[^A-Za-z0-9.]").unwrap();
            let after = re.replace_all(&datesubstrf, "_");
            let datesubstrfx = after.to_string();
//            let filelnxx = fromfilename.len();
//            let fileendxx = filelnxx - 2;
//            let filestartxx = 6;
//            let fromfilenamexx = fromfilename.get(filestartxx..fileendxx).unwrap();
            let msgstr = format!("copied {} to {}{}", fromfilename, prefixstr, datesubstrfx);            
            let fullfrom = str_cur_dirfrom + "/" + &fromfilename;
            if !Path::new(&fullfrom).exists() {
                errstring = format!("********* Merge Merge: ERROR {} does not exist **********", fullfrom);
                errcode = 10;
            } else {
                let fullto = str_cur_dirto.clone() + "/" + &prefixstr + &datesubstrfx;
                if Path::new(&fullto).exists() {
                    errstring = format!("********* Merge Merge: ERROR {} already exists **********", fullto);
                    errcode = 11;
                } else {
                    let _output = Command::new("cp")
                                  .arg("-p")
                                  .arg(&fullfrom)
                                  .arg(&fullto)
                                  .output()
                                  .expect("failed to execute process");
                    errstring = msgstr;
                }
            }
        }
        (errcode, errstring)
}
