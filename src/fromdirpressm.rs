use native_dialog::FileDialog;
use std::path::{Path};
use crate::get_fromdirlistm;
pub fn fromdirpressm (dirval: String, sizetxt: String) -> (u32, String, String, Vec<String>, i32, u32) {
     let errstring: String;
     let mut new_dirlist: Vec<String> = Vec::new();
     let mut shortto_int1 = 0;
     let mut icon_int1 = 0;
     let mut new_dir: String;
     let errcode: u32;
     if Path::new(&dirval).exists() {
         new_dir = dirval.to_string();
     } else {
         new_dir = "/".to_string();
     }
     let folder = FileDialog::new()
        .set_location(&new_dir)
        .show_open_single_dir()
        .unwrap();
     if folder == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         let current_dir = folder;
         let (errcda, errstra, newlist) = get_fromdirlistm(current_dir.unwrap());
         if errcda != 0 {
             errstring = errstra.to_string();
             errcode = errcda;
         } else {
             new_dirlist = newlist;
             if sizetxt.len() == 0 { 
                 errstring = "********* List: Icon has no value **********".to_string();
                 errcode = 2;
             } else {
                 let icon_int: i32 = sizetxt.parse().unwrap_or(-99);
                 if icon_int > 0 {
                     if (icon_int < 50) | (icon_int > 255) {
                         errstring = "********* List: Icon not between 50 and 255 **********".to_string();
                         errcode = 3;
                     } else {
                         icon_int1 = icon_int;
                         shortto_int1 = new_dirlist.len() as i32 ;
                         errstring = "got directory".to_string();
                         errcode = 0;
                     }
                 } else if icon_int == -99 {
                     errstring = "********* List: Icon is not an integer **********".to_string();
                     errcode = 4;
                 } else {
                     errstring = "********* List: Icon Size not positive integer **********".to_string();
                     errcode = 5;
                 }
             }
         }
     }
     (errcode, errstring, new_dir, new_dirlist, shortto_int1, icon_int1 as u32)
}

