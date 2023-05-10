use std::fs;
pub fn get_prevafterm(cur_dir: String, tofilename: String) -> (u32, String, String, String) { 
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
    let mut namec = " ";
    let mut namep = " ";
    let mut namea = " ";
    let mut numentry = 0;
    let mut baddate1 = 0;
    let mut listitems: Vec<String> = Vec::new();
    for entry1 in fs::read_dir(&cur_dir).unwrap() {
         if baddate1 == 0 {
             let entry = entry1.unwrap();
             if let Ok(metadata) = entry.metadata() {
                 if let Ok(file_name) = entry.file_name().into_string() {
                     if metadata.is_file() {
                         if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                            file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                            file_name.ends_with(".png") |file_name.ends_with(".PNG") {
                             listitems.push(file_name);
                             numentry = numentry + 1;
                         } else {
                             baddate1 = 1;
                         }
                     }
                 } else {
                     baddate1 = 1;
                 }
             } else {
                 baddate1 = 1;
             }
         }
    }
    if baddate1 == 1 {
        errstring = "<span color=\"#FF000000\">********* Preview: ERROR directory does not conform **********</span>".to_string();
        errcode = 1;
    } else {
        if numentry < 1 {
            errstring = "<span color=\"#FF000000\">********* Preview: directory has no images **********</span>".to_string();
            errcode = 2;
        } else {
            listitems.sort();
            let listitemlen = listitems.len();
            let newtoi = listitemlen as i32 ;
            let mut found = 0;
            for indexi in 0..newtoi {
                 if found == 0 {
                     if namep == " " {
                         namep = &listitems[indexi as usize];
                     } else if namec == " " {
                         namec = &listitems[indexi as usize];
                     } else if namea == " " {
                         namea = &listitems[indexi as usize];
                     } else {
                         if tofilename == namep {
                             namea = namec;
                             namec = namep;
                             namep = " ";
                             found = 1;
                         } else if tofilename == namec {
                             found = 1;
                         } else {
                             namep = namec;
                             namec = namea;
                             namea = &listitems[indexi as usize];
                         }
                     }
                 }
            }
            if found == 0 {
                if tofilename == namec {
                    found = 1;
                } else { 
                    if namea != " " {
                        if tofilename == namea {
                            namep = namec;
                            namea = " ";
                            found = 1;
                        }
                    }
                }
            }
            if found == 0 {
                errstring = "<span color=\"#FF000000\">********* Preview: file not found in directory **********</span>".to_string();
                errcode = 3;
            }
        }
    }
    let namepo = namep.to_string();
    let nameao = namea.to_string();
    (errcode, errstring, namepo, nameao)
}

