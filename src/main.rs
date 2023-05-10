use iced::alignment::{self, Alignment};
use iced::widget::scrollable::{Properties};
use iced::theme::{self, Theme};
use iced::widget::{
    button, checkbox, column, row, scrollable, text, horizontal_space,
    image, container, Column, Row, text_input, Radio,
};

use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Length, Settings};

use serde::{Deserialize, Serialize};

extern crate image as create_image;
mod get_winsize;
mod dump_file;
mod fromdirpressm;
mod mergepressm;
mod todirpressm;
mod get_fromdirlistm;
mod gen_merge;
mod dateinname_merge;
mod celldatename_merge;
mod todirrefreshm;
mod get_prevafterm;

use get_fromdirlistm::get_fromdirlistm;
use get_winsize::get_winsize;
use fromdirpressm::fromdirpressm;
use todirpressm::todirpressm;
use mergepressm::mergepressm;
use todirrefreshm::todirrefreshm;
use get_prevafterm::get_prevafterm;

pub fn main() -> iced::Result {
     let mut widthxx: u32 = 1350;
     let mut heightxx: u32 = 750;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho;
         heightxx = heighto;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     ImageList::run(Settings {
        window: window::Settings {
            size: (widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
enum ImageList {
    Loaded(State),
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UseChoice {
    GEN,
    DIN,
    PDN,
    DDT,
}

impl Default for UseChoice {
    fn default() -> Self {
        UseChoice::GEN
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DtChoice {
    DT,
    OD,
    OT,
}

impl Default for DtChoice {
    fn default() -> Self {
        DtChoice::DT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ABChoice {
    After,
    Before,
}

impl Default for ABChoice {
    fn default() -> Self {
        ABChoice::After
    }
}

#[derive(Debug, Default)]
struct State {
    filter: Filter,
    filterf: Filterf,
    images: Vec<ImageItem>,
    files: Vec<File>,
    fromdir_value: String,
    todir_value: String,
    msg_value: String,
    mess_color: Color,
    size_value: String,
    usechoice_value: UseChoice,
    dtchoice_value: DtChoice,
    abchoice_value: ABChoice,
    currexist: bool,
    currrgb: Vec<u8>,
    currwidth: u32,
    currheight: u32,
    prevexist: bool,
    prevrgb: Vec<u8>,
    prevwidth: u32,
    prevheight: u32,
    afterexist: bool,
    afterrgb: Vec<u8>,
    afterwidth: u32,
    afterheight: u32,
}

#[derive(Debug, Clone)]
enum Message {
    FilterChanged(Filter),
    FilterChangedf(Filterf),
    ImageMessage(usize, ImageMessage),
    FileMessage(usize, FileMessage),
    FromDirPressed,
    ToDirPressed,
    PreviewPressed,
    MergePressed,
    SizeChanged(String),
    UseRadioSelected(UseChoice),
    DtRadioSelected(DtChoice),
    ABRadioSelected(ABChoice),
}

impl Application for ImageList {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ImageList, Command<Message>) {

        (
            ImageList::Loaded(State
               {
                filter:Filter::All,
                filterf:Filterf::All,
                images:Vec::<ImageItem>::new(),
                files:Vec::<File>::new(),
                fromdir_value: "no directory".to_string(),
                todir_value: "no directory                                                             ".to_string(),
                mess_color: Color::from([0.0, 0.0, 0.0]),
                msg_value: "no message".to_string(),
                size_value: "140".to_string(),
                usechoice_value:UseChoice::GEN,
                dtchoice_value:DtChoice::DT,
                abchoice_value:ABChoice::After,
                currexist: false,
                currrgb: Vec::<u8>::new(),
                currheight: 0,
                currwidth: 0,
                prevexist: false,
                prevrgb: Vec::<u8>::new(),
                prevwidth: 0,
                prevheight: 0,
                afterexist: false,
                afterrgb: Vec::<u8>::new(),
                afterwidth: 0,
                afterheight: 0,
                }
            ),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Individual Rotation -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            ImageList::Loaded(state) => {

                let command = match message {
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Command::none()
                    }
                    Message::FilterChangedf(filterf) => {
                        state.filterf = filterf;

                        Command::none()
                   }
                   Message::ImageMessage(i, image_message) => {
                        if let Some(image) = state.images.get_mut(i) {

                            image.update(image_message);

                               Command::none()
                        } else {
                            Command::none()
                        }
                    }
                    Message::FileMessage(i, file_message) => {
                        if let Some(file) = state.files.get_mut(i) {

                            file.update(file_message);

                               Command::none()
                        } else {
                            Command::none()
                        }
                    }
                    Message::FromDirPressed => {
                       let (errcode, errstr, newdir, listitems, newtoi, icon_int1) = fromdirpressm(state.size_value.clone());
                       if errcode == 0 {
                           if newtoi != 0 {
                               state.images.clear();                         
                               for indexi in 0..newtoi {
                                    state.fromdir_value = newdir.to_string();
                                    let linestr = listitems[indexi as usize].clone();
                                    let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                                    let filefromx = lineparse[0].clone().to_string();
                                    let fullpath = state.fromdir_value.clone() + "/" + &filefromx;
                                    println!("fullpath -{}- ", fullpath);
                                    let newwidth: u32;
                                    let newheight: u32;
                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                        if iwidth > iheight {
                                            newwidth = icon_int1;
                                            newheight = icon_int1 * iheight / iwidth;
                                        } else {
                                            newheight = icon_int1;
                                            newwidth = icon_int1 * iwidth / iheight;
                                        }
                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                        let rgbconv = imgbuffer.into_vec();
                                        state
                                           .images
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight));
                                    } else {
                                        println!("no wxh fullpath -{}- ", fullpath);
                                    }
                               }
                           }
                       }
                       state.msg_value = errstr.to_string();
                       if errcode == 0 {
                           state.mess_color = Color::from([0.0, 1.0, 0.0]);
                       } else {
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };

                       Command::none()
                    } 
                    Message::ToDirPressed => {
                        let (colorout, errstr, newdir, listitems) = todirpressm();
                        if errstr == "got directory" {
                            state.files.clear();                         
                            state.todir_value = newdir.to_string();
                            let listitemlen = listitems.len();
                            let newtoi = listitemlen as i32 ;
                            for indexi in 0..newtoi {
                                state
                                    .files
                                    .push(File::new(listitems[indexi as usize].clone()));
                            } 
                        }
                        state.msg_value = errstr.to_string();
                        state.mess_color = colorout;
                        Command::none()
                    } 
                    Message::PreviewPressed => {
                        let files_selected = state.files.iter().filter(|fileitem| fileitem.completed).count();
                        if files_selected < 1 {
                            state.msg_value = "no TO image selected".to_string();
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        } else if files_selected > 1 {
                            state.msg_value = "more than 1 TO image selected".to_string();
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        } else {
                            let mut toimagestr: String = " ".to_string();
                            for filesy in state.files.iter() {
                                 if filesy.completed {
                                     toimagestr = filesy.description.clone();
                                 }
                            }
                            let (errcode, errstr, namepo, nameao) = get_prevafterm(state.todir_value.clone(), toimagestr.clone());
                            if errcode != 0 {
                                state.msg_value = errstr.to_string();
                                state.mess_color = Color::from([1.0, 0.0, 0.0]);
                            } else {
                                state.msg_value = format!("preview images is -{}-, -{}-, -{}-", namepo, toimagestr, nameao);
                                state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                let sizetxt = state.size_value.clone();
                                if sizetxt.len() == 0 { 
                                    state.msg_value = "********* List: Icon has no value **********".to_string();
                                    state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                } else {
                                    let icon_int: i32 = sizetxt.parse().unwrap_or(-99);
                                    if icon_int > 0 {
                                        if (icon_int < 50) | (icon_int > 255) {
                                            state.msg_value = "********* List: Icon not between 50 and 255 **********".to_string();
                                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                        } else {
                                            let fullpathc = state.todir_value.clone() + "/" + &toimagestr.clone();
                                            println!("fullpathc -{}- ", fullpathc);
                                            let mut newwidth: u32;
                                            let mut newheight: u32;
                                            if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpathc.clone()) {
                                                if iwidth > iheight {
                                                    newwidth = icon_int.clone() as u32;
                                                    newheight = icon_int as u32 * iheight / iwidth;
                                                } else {
                                                    newheight = icon_int.clone() as u32;
                                                    newwidth = icon_int as u32 * iwidth / iheight;
                                                }
                                                let loadimg = create_image::open(fullpathc.clone()).unwrap();
                                                let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                                state.currrgb = imgbuffer.into_vec();
                                                state.currexist = true;
                                                state.currheight = newheight;
                                                state.currwidth = newwidth;
                                                let mut errset = 0;
                                                if namepo != " " {
                                                    let fullpathp = state.todir_value.clone() + "/" + &namepo;
                                                    println!("fullpathp -{}- ", fullpathp);
                                                    if let Ok((iwidthp, iheightp)) = create_image::image_dimensions(fullpathp.clone()) {
                                                        if iwidthp > iheightp {
                                                            newwidth = icon_int.clone() as u32;
                                                            newheight = icon_int as u32 * iheightp / iwidthp;
                                                        } else {
                                                            newheight = icon_int.clone() as u32;
                                                            newwidth = icon_int as u32 * iwidthp / iheightp;
                                                        }
                                                        let loadimgp = create_image::open(fullpathp.clone()).unwrap();
                                                        let imgbufferp = create_image::imageops::thumbnail(&loadimgp, newwidth, newheight);
                                                        state.prevrgb = imgbufferp.into_vec();
                                                        state.prevexist = true;
                                                        state.prevheight = newheight;
                                                        state.prevwidth = newwidth;
                                                    } else {
                                                        errset = 1;
                                                        state.msg_value = format!("**** error getting previous image {}*****", fullpathp);
                                                        state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                                    }  
                                                } else {
                                                    state.prevexist = false;
                                                } 
                                                if nameao != " " {
                                                    let fullpatha = state.todir_value.clone() + "/" + &nameao;
                                                    println!("fullpatha -{}- ", fullpatha);
                                                    if let Ok((iwidtha, iheighta)) = create_image::image_dimensions(fullpatha.clone()) {
                                                        if iwidtha > iheighta {
                                                            newwidth = icon_int.clone() as u32;
                                                            newheight = icon_int as u32 * iheighta / iwidtha;
                                                        } else {
                                                            newheight = icon_int.clone() as u32;
                                                            newwidth = icon_int as u32 * iwidtha / iheighta;
                                                        }
                                                        let loadimga = create_image::open(fullpatha.clone()).unwrap();
                                                        let imgbuffera = create_image::imageops::thumbnail(&loadimga, newwidth, newheight);
                                                        state.afterrgb = imgbuffera.into_vec();
                                                        state.afterexist = true;
                                                        state.afterheight = newheight;
                                                        state.afterwidth = newwidth;
                                                    } else {
                                                        errset = 2;
                                                        state.msg_value = format!("**** error getting after image {}*****", fullpatha);
                                                        state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                                    }  
                                                } else {
                                                    state.afterexist = false;
                                                } 
                                                if errset == 0 {
                                                    state.msg_value = format!("got preview images {} -- {} -- {}", namepo, toimagestr.clone(), nameao);
                                                    state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                                }
                                            } else {
                                                state.msg_value = format!("**** error getting current image {}*****", fullpathc);
                                                state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                            }
                                        }
                                    } else if icon_int == -99 {
                                            state.msg_value = "********* List: Icon is not an integer **********".to_string();
                                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                    } else {
                                            state.msg_value = "********* List: Icon Size not positive integer **********".to_string();
                                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                    }
                                }
                            }
                        };
                        Command::none()
                     }
                    Message::MergePressed => {
// check if selections
                       let images_selected = state.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_selected < 1 {
                           state.msg_value = "no FROM image selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else if images_selected > 1 {
                           state.msg_value = "more than 1 FROM image selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           let files_selected = state.files.iter().filter(|fileitem| fileitem.completed).count();
                           if files_selected < 1 {
                               state.msg_value = "no TO image selected".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           } else if files_selected > 1 {
                               state.msg_value = "more than 1 TO image selected".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           } else {
                               let mut fromimagestr: String = " ".to_string();
                               for imagesy in state.images.iter() {
                                    if imagesy.completed {
                                       fromimagestr = imagesy.description.clone();
                                    }
                               }
                               let mut toimagestr: String = " ".to_string();
                               for filesy in state.files.iter() {
                                    if filesy.completed {
                                       toimagestr = filesy.description.clone();
                                    }
                               }
                               let struse = match state.usechoice_value {
                                               UseChoice::GEN => "gen".to_string(),
                                               UseChoice::DIN => "din".to_string(),
                                               UseChoice::PDN => "pdn".to_string(),
                                               UseChoice::DDT => "ddt".to_string() };
                               let strdt = match state.dtchoice_value {
                                               DtChoice::DT => "dt".to_string(),
                                               DtChoice::OD => "od".to_string(),
                                               DtChoice::OT => "ot".to_string() };
                               let strab = match state.abchoice_value {
                                               ABChoice::After => "after".to_string(),
                                               ABChoice::Before => "before".to_string()};
                               
                               state.msg_value = format!("got FROM {} and TO {}", fromimagestr, toimagestr);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               let (errcode, errstr) = mergepressm(fromimagestr, toimagestr, state.fromdir_value.clone(),
                                                                state.todir_value.clone(), struse, strdt, strab);
                               if errcode == 0 {
                                   let (errcodea, errstra, listitems) = todirrefreshm(state.todir_value.clone());
                                   if errcodea  == 0 {
                                       state.files.clear();                         
                                       let listitemlen = listitems.len();
                                       let newtoi = listitemlen as i32 ;
                                       for indexi in 0..newtoi {
                                            state
                                              .files
                                              .push(File::new(listitems[indexi as usize].clone()));
                                       } 
                                       state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                   } else {
                                       state.msg_value = errstra.to_string();
                                       state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   }                                
                               } else {
                                   state.msg_value = errstr.to_string();
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               }                                
                           }
                       };
                       Command::none()
                    }
                    Message::UseRadioSelected(xchoice) => {
                        let strx = match xchoice {
                        UseChoice::GEN => "choice gen selected",
                        UseChoice::DIN => "choice date in name selected",
                        UseChoice::PDN => "choice phone name selected",
                        UseChoice::DDT => "choice displayed date selected" };
                       state.usechoice_value = xchoice;
                       state.msg_value = strx.to_string();
                       Command::none()
                    }
                    Message::DtRadioSelected(dchoice) => {
                        let strx = match dchoice {
                        DtChoice::DT => "choice date and time selected",
                        DtChoice::OD => "choice only date selected",
                        DtChoice::OT => "choice only time selected" };
                       state.dtchoice_value = dchoice;
                       state.msg_value = strx.to_string();
                       Command::none()
                    }
                    Message::ABRadioSelected(achoice) => {
                        let strx = match achoice {
                        ABChoice::After => "choice after selected",
                        ABChoice::Before => "choice before selected"};
                       state.abchoice_value = achoice;
                       state.msg_value = strx.to_string();
                       Command::none()
                    }
                    Message::SizeChanged(value) => { state.size_value = value; Command::none() }

                };

                Command::batch(vec![command, Command::none()])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            ImageList::Loaded(State {
                filter,
                filterf,
                files,
                images,
                fromdir_value,
                todir_value,
                msg_value,
                mess_color,
                size_value,
                usechoice_value,
                dtchoice_value,
                abchoice_value,
                currexist,
                currrgb,
                currheight,
                currwidth,
                prevexist,
                prevrgb,
                prevwidth,
                prevheight,
                afterexist,
                afterrgb,
                afterwidth,
                afterheight,
                ..
            }) => {
                let title = text("Merge picture into directory")
                    .width(Length::Fill)
                    .size(20)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(alignment::Horizontal::Center);

                let mut messcol = Column::new().spacing(10);
                messcol = messcol.push(container(row![text("Message:").size(20),
                 text(msg_value).size(20).style(*mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(5)
                    ));

                let mut dirbutshow = Column::new().spacing(10);
                dirbutshow = dirbutshow.push(container(row![button("From Directory Button")
                                                             .on_press(Message::FromDirPressed)
                                                             .style(theme::Button::Secondary),
                                                            text(fromdir_value)
                                                             .size(20),
                                                           ].align_items(Alignment::Center).spacing(100).padding(1),
                 ));
                dirbutshow = dirbutshow.push(container(row![button("To Directory Button")
                                                             .on_press(Message::ToDirPressed)
                                                             .style(theme::Button::Secondary),
                                                            text(todir_value)
                                                             .size(20),
                                                           ].align_items(Alignment::Center).spacing(100).padding(1),
                 ).align_x(alignment::Horizontal::Right).width(Length::Fill));

                let controls = view_controls(images, *filter);
                let filtered_images =
                    images.iter().filter(|imageitem| filter.matches(imageitem));

                let mut imagescol1 = Column::new().spacing(10);
                let mut imagescol2 = Column::new().spacing(10);
                let mut colpos = 0;
                let mut n = 0;
                if filtered_images.clone().count() == 0 {
                    n = 1;
                    imagescol1 = imagescol1.push(container(row![empty_message(match filter {
                        Filter::All => "No directory selected or no files in directory",
                        Filter::Active => "All files have been selected",
                        Filter::Completed => "No files have been selected" 
                    })]));
                } else {
                    for imagesy in images.iter() {
                         if imagesy.completed {
                             if (filter == &Filter::All) || (filter == &Filter::Completed) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                               }
                             }
                         } else {
                             if (filter == &Filter::All) || (filter == &Filter::Active) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                               }
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut imagesrow = Row::new().spacing(20);
                imagesrow = imagesrow.push(container(imagescol1).padding(10).width(Length::Fixed(250.0)));
                if n > 1 {
                    imagesrow = imagesrow.push(container(imagescol2).padding(10).width(Length::Fixed(250.0)));
                }

                let scrollable_content: Element<Message> =
                  Element::from(scrollable(
                    imagesrow
                )
                .height(Length::Fill)
                .width(Length::Fixed(600.0))
                .horizontal_scroll(
                    Properties::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10),
                )); 

                let controlsf = view_controlsf(files, *filterf);
                let filtered_files =
                    files.iter().filter(|file| filterf.matches(file));

                let mut filescol1 = Column::new().spacing(10);
                let mut n = 0;
                if filtered_files.clone().count() == 0 {
                    filescol1 = filescol1.push(container(row![empty_message(match filterf {
                        Filterf::All => "No directory selected or no files in directory",
                        Filterf::Active => "All files have been selected",
                        Filterf::Completed => "No files have been selected" 
                    })]));
                } else {
                    for filesy in files.iter() {
                         if filesy.completed {
                             if (filterf == &Filterf::All) || (filterf == &Filterf::Completed) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         } else {
                             if (filterf == &Filterf::All) || (filterf == &Filterf::Active) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut filesrow = Row::new().spacing(20);
                filesrow = filesrow.push(container(filescol1).padding(10).width(Length::Fixed(500.0)));

                let scrollable_contentf: Element<Message> =
                  Element::from(scrollable(
                    filesrow
                )
                .height(Length::Fill)
                .width(Length::Fixed(500.0))
               .horizontal_scroll(
                    Properties::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10),
                )); 

                let selected_usechoice = Some(usechoice_value);
                let ua = Radio::new(
                         "Generate Date",
                         UseChoice::GEN,
                         selected_usechoice.copied(),
                         Message::UseRadioSelected,
                ).size(15);
                let ub = Radio::new(
                         "Date in Name",
                         UseChoice::DIN,
                         selected_usechoice.copied(),
                         Message::UseRadioSelected,
                ).size(15);
           
                let uc = Radio::new(
                         "Phone Date Name",
                         UseChoice::PDN,
                         selected_usechoice.copied(),
                         Message::UseRadioSelected,
                ).size(15);
           
                let ud = Radio::new(
                           "Displayed Date",
                           UseChoice::DDT,
                           selected_usechoice.copied(),
                           Message::UseRadioSelected
                ).size(15);
           
                let contentuse = row![ua, ub, uc, ud, button("Preview Images").on_press(Message::PreviewPressed),].spacing(80).padding(1);

                let selected_dtchoice = Some(dtchoice_value);
                let da = Radio::new(
                         "Date & Time",
                         DtChoice::DT,
                         selected_dtchoice.copied(),
                         Message::DtRadioSelected,
                ).size(15);
                let db = Radio::new(
                         "Only Date",
                         DtChoice::OD,
                         selected_dtchoice.copied(),
                         Message::DtRadioSelected,
                ).size(15);
           
                let dc = Radio::new(
                         "Only Time",
                         DtChoice::OT,
                         selected_dtchoice.copied(),
                         Message::DtRadioSelected,
                ).size(15);
           
                let contentdt = row![da, db, dc,].spacing(80).padding(1);


                let selected_abchoice = Some(abchoice_value);
                let aa = Radio::new(
                         "After",
                         ABChoice::After,
                         selected_abchoice.copied(),
                         Message::ABRadioSelected,
                ).size(15);
                let ab = Radio::new(
                         "Before",
                         ABChoice::Before,
                         selected_abchoice.copied(),
                         Message::ABRadioSelected,
                ).size(15);
                let contentab = row![button("Merge").on_press(Message::MergePressed), aa, ab, text("             Icon Size: ").size(20), text_input("140", size_value).on_input(Message::SizeChanged).padding(10).size(20),].spacing(80).padding(1);

                let titlefromto = row![horizontal_space(50), text("********* FROM *********"),horizontal_space(350),text("********* TO *********"),].spacing(80).padding(5);


                if *prevexist || *currexist || *afterexist {
                    let mut previewcol = Column::new().spacing(20);
                    if *prevexist {
                        let previmage = image::Handle::from_pixels(prevwidth.clone(), prevheight.clone(), prevrgb.clone()); 
                        previewcol =  previewcol.push(container(image::Viewer::new(previmage).height(Length::Fixed(300.0))));
                    }
                    if *currexist {
                        let currimage = image::Handle::from_pixels(currwidth.clone(), currheight.clone(), currrgb.clone()); 
                        previewcol =  previewcol.push(container(image::Viewer::new(currimage).height(Length::Fixed(300.0))));
                    }
                    if *afterexist {
                        let afterimage = image::Handle::from_pixels(afterwidth.clone(), afterheight.clone(), afterrgb.clone()); 
                        previewcol =  previewcol.push(container(image::Viewer::new(afterimage).height(Length::Fixed(300.0))));
                    }
                    column![title, messcol, dirbutshow, contentuse, contentdt, contentab, titlefromto, row![controls, horizontal_space(400), controlsf], row![scrollable_content, scrollable_contentf, previewcol].spacing(5)]
                         .spacing(5)
                         .max_width(1300)
                         .into()
                } else {
                   column![title, messcol, dirbutshow, contentuse, contentdt, contentab, titlefromto, row![controls, horizontal_space(400), controlsf], row![scrollable_content, scrollable_contentf]]
                         .spacing(1)
                         .max_width(1300)
                         .into()
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ImageItem {
    description: String,
    completed: bool,
    rgbconv: Vec<u8>,
    twidth: u32,
    theight: u32,
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
    Completed(bool),
}

impl ImageItem {

    fn new(description: String, rgbconv: Vec<u8>, twidth:  u32, theight: u32,) -> Self {
        ImageItem {
            description,
            completed: false,
            rgbconv,
            twidth,
            theight,
        }
    }

    fn update(&mut self, message: ImageMessage) {
        match message {
            ImageMessage::Completed(completed) => {
                self.completed = completed;
            }
        }
    }

    fn view(&self, _i: usize) -> Element<ImageMessage> {
        let checkbox = checkbox(
            &self.description,
            self.completed,
            ImageMessage::Completed,
        )
        .width(Length::Fill).text_size(15);
        let newimage = image::Handle::from_pixels(self.twidth.clone(), self.theight.clone(), self.rgbconv.clone()); 

        column![
           container(
        // This should go away once we unify resource loading on native
        // platforms
             image::Viewer::new(newimage)
                 .height(Length::Fixed(300.0)),
           )
           .width(Length::Fill),
            checkbox,
        ]
        .align_items(Alignment::Center)
        .spacing(5)
        .into()

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct File {
    description: String,
    completed: bool,

}

#[derive(Debug, Clone)]
pub enum FileMessage {
    Completed(bool),
}

impl File {

    fn new(description: String) -> Self {
        File {
            description,
            completed: false,
        }
    }

    fn update(&mut self, message: FileMessage) {
        match message {
            FileMessage::Completed(completed) => {
                self.completed = completed;
            }

        }
    }

    fn view(&self, _i: usize) -> Element<FileMessage> {
                let checkbox = checkbox(
                    &self.description,
                    self.completed,
                    FileMessage::Completed,
                )
                .width(Length::Fixed(500.0));

                row![
                    checkbox,

                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into()

    }
}


fn view_controlsf(files: &[File], current_filter: Filterf) -> Element<Message> {
    let files_left = files.iter().filter(|file| file.completed).count();

    let filter_button = |label, filterf, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filterf == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Text
        });

        button.on_press(Message::FilterChangedf(filterf)).padding(8)
    };

        row![        text(format!(
            "{} {} selected",
            files_left,
            if files_left == 1 { "file" } else { "files" }
        ))
        .size(16),

            filter_button("All", Filterf::All, current_filter),
            filter_button("Not Selected", Filterf::Active, current_filter),
            filter_button("Selected", Filterf::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    .align_items(Alignment::Center)
    .into()
}




fn view_controls(images: &[ImageItem], current_filter: Filter) -> Element<Message> {
    let images_left = images.iter().filter(|imageitem| imageitem.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filter == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![horizontal_space(20),
        text(format!(
            "{} {} selected",
            images_left,
            if images_left == 1 { "file" } else { "files" }
        ))
        .size(16),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Not Selected", Filter::Active, current_filter),
            filter_button("Selected", Filter::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

impl Filter {
    fn matches(&self, imageitem: &ImageItem) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !imageitem.completed,
            Filter::Completed => imageitem.completed,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filterf {
    All,
    Active,
    Completed,
}

impl Default for Filterf {
    fn default() -> Self {
        Filterf::All
    }
}

impl Filterf {
    fn matches(&self, file: &File) -> bool {
        match self {
            Filterf::All => true,
            Filterf::Active => !file.completed,
            Filterf::Completed => file.completed,
        }
    }
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .center_y()
    .into()
}
