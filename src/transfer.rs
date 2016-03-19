use std::io::{ Read };
use std::fs::File;
use rustc_serialize::json;
use time::{ self, Timespec };
use parse_utils::GetMsecs;
use events::{ EventId, FullEventInfo, EventState };
use events::feed_types::{ FeedEventState };

use stuff::Stuff;
use types::{ Id, PhotoInfo, ImageType, ShortInfo };
use database::Databaseable;
use db::events::DbEvents;
use db::photos::DbPhotos;
use db::publication::DbPublication;
use db::group_feed::DbGroupFeed;
use db::visited::{ VisitedContent, DbVisited };

use image::{ self, GenericImage };
use exif_reader::{ self, ExifValues };

use std::fs;


#[derive(RustcDecodable, Clone)]
struct ConfigElement {
    username: String,
    subject: String,
    year: String,
    week: String
}


#[derive(RustcDecodable, Debug)]
struct UserForTransfer {
    id: Id,
    old_name: String,
    new_name: String
}

#[derive(RustcDecodable)]
struct ConfigStructure {
    from_weekly_dir: String,
    to_data_dir: String,
    group_id: Id,
    users: Vec<UserForTransfer>,
    photos: Vec<ConfigElement>
}

#[derive(Debug)]
struct OldPhotoInfo {
    user_id: Id,
    old_user_name: String,
    new_user_name: String,
    name: String,
}

#[derive(Debug)]
struct OldEvent {
    year: i64,
    week: i64,
    photos: Vec<OldPhotoInfo>
}

#[derive(Debug)]
struct TransferConfig {
    from_weekly_dir: String,
    to_data_dir: String,
    group_id: Id,
    events: Vec<OldEvent>,
    users: Vec<Id>
}



pub fn transfer( stuff: &mut Stuff, config_filename: &str )
{
    let transfer_config = load_config( config_filename );
    let db = stuff.get_current_db_conn().unwrap();

    let mut feed_id: Id = 2;

    for event in transfer_config.events {
        let event_tm = gen_time( event.year, event.week );
        let event_time = event_tm.to_timespec();

        // создаём событие
        let event_info = FullEventInfo {
            id: EventId::Publication,
            name: gen_event_name( event.year, event.week ),
            start_time: event_time,
            end_time: event_time,
            data: String::new(),
            group: Some( transfer_config.group_id ),
            creator: None
        };
        let event_id = db.add_disabled_event( &event_info ).unwrap();
        db.set_event_state( event_id, EventState::Finished ).unwrap();

        // публикуем фотографии
        for (idx, photo) in event.photos.iter().enumerate() {
            move_photo_with_preview(
                &transfer_config.from_weekly_dir,
                &transfer_config.to_data_dir,
                event.year,
                event.week,
                event_time.msecs(),
                &photo
            );

            let photo_filename = gen_dest_filename(
                &transfer_config.to_data_dir,
                &photo.new_user_name,
                event_time.msecs(),
                false
            );

            // добавляем в галлерею
            let next = db.get_last_upload_gallery_photo( photo.user_id ).unwrap();
            let photo_info = make_photo_info(
                &photo_filename,
                event_time,
                &photo.name,
                photo.user_id,
                &photo.new_user_name,
                next.clone()
            );
            let photo_id = db.add_photo( photo.user_id, &photo_info ).unwrap();
            if let Some( id ) = next {
                db.set_prev_in_gallery( id, photo_id ).unwrap();
            }
            //публикуем
            let prev = db.get_last_pubcation_photo( event_id ).unwrap();
            db.public_photo( event_id,
                             photo.user_id,
                             photo_id,
                             true,
                             event_time.msecs() + idx as u64,
                             prev.clone() ).unwrap();
            if let Some( last_id ) = prev {
                db.set_next_publication_photo( last_id, photo_id ).unwrap();
            }

            // println!( "user_id: {} name: {}", photo.user, photo.name );
        }


        // Добавляем событие в ленту событий
        db.add_to_group_feed( event_time.msecs(),
                              transfer_config.group_id,
                              event_id,
                              FeedEventState::Finish,
                              "" ).unwrap();

        for &id in &transfer_config.users {
            db.set_visited( id, VisitedContent::Feed, &[ feed_id ]).unwrap();
        }
        feed_id += 1;

         println!( "event {}#{} transfered.", event.year, event.week );
    }
}

fn make_photo_info( filename: &str,
                    time: Timespec,
                    photo_name: &str,
                    user_id: Id,
                    user_name: &str,
                    next: Option<Id> ) -> PhotoInfo
{
    let mut file = File::open( filename ).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end( &mut bytes ).unwrap();
    let (width, height) = {
        let image = image::load_from_memory_with_format( &bytes, image::JPEG ).unwrap();
        image.dimensions()
    };
    let exif_data = exif_reader::from_memory( &bytes );
    let exif = exif_data.as_ref();
    PhotoInfo {
        id: 0,
        upload_time: time.msecs(),
        image_type: ImageType::Jpeg,
        width: width,
        height: height,
        name: photo_name.to_owned(),
        iso: exif.and_then( |e| e.iso() ),
        shutter_speed: exif.and_then( |e| e.shutter_speed() ),
        aperture: exif.and_then( |e| e.aperture() ),
        focal_length: exif.and_then( |e| e.focal_length() ),
        focal_length_35mm: exif.and_then( |e| e.focal_length_35mm() ),
        camera_model: exif.and_then( |e| e.camera_model().map( |m| m.to_owned() ) ),
        owner: ShortInfo {
            id: user_id,
            name: user_name.to_owned()
        },
        comments_count: 0,
        unreaded_comments: 0,
        next: next,
        prev: None
    }
}

fn move_photo_with_preview( from: &str, to: &str, year: i64, week: i64, event_time: u64, photo: &OldPhotoInfo )
{
    move_photo( from, to, year, week, event_time, photo, false );
    move_photo( from, to, year, week, event_time, photo, true );
}

fn move_photo( from: &str, to: &str, year: i64, week: i64, event_time: u64, photo: &OldPhotoInfo, is_preview: bool ) {
    let old_filename = gen_source_filename( from, year, week, &photo.old_user_name, is_preview );
    let new_filename = gen_dest_filename( to, &photo.new_user_name, event_time, is_preview );
    println!( "move {} -> {}", old_filename, new_filename );
    fs::rename( old_filename, new_filename ).unwrap()
}

fn gen_time( year: i64, week: i64 ) -> time::Tm {
    let mut tm = time::empty_tm();
    tm.tm_year = ( year - 1900 ) as i32;
    tm.tm_hour = 11;
    let tm = tm + time::Duration::days( week * 7 );
    let days_diff = tm.tm_wday - 2;
    let tm = tm - time::Duration::days( days_diff as i64 );
    tm
}

fn gen_event_name( year: i64, week: i64 ) -> String {
    format!( "{}#{:0>2}", year, week )
}

fn gen_dest_filename( to_dir: &str,
                      username: &str,
                      time: u64,
                      is_preview: bool ) -> String
{
    format!( "{dir}/photos/{user}/gallery/{time}{postfix}",
               dir = to_dir,
               user = username,
               time = time,
               postfix = if is_preview { "_preview.png" } else { ".jpg" } )
}

fn gen_source_filename( from_dir: &str,
                        year: i64,
                        week: i64,
                        username: &str,
                        is_preview: bool ) -> String
{
    format!( "{dir}/{user}/{year}_{week:0>2}{postfix}.jpg",
               dir = from_dir,
               user = username,
               year = year,
               week = week,
               postfix = if is_preview { "_preview" } else { "" } )
}

fn load_config( config_filename: &str ) -> TransferConfig {
    let mut file = File::open( config_filename ).unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content).unwrap();
    let config = json::decode::<ConfigStructure>( &content ).unwrap();

    let mut config_elems_grouped: Vec<Vec<ConfigElement>> = Vec::new();
    {
        let mut events = config.photos.clone();
        while events.is_empty() == false {
            let week = events[0].week.clone();
            let year = events[0].year.clone();
            let (part, other) = events.into_iter()
                .partition( |e| e.year == year && e.week == week );
            config_elems_grouped.push( part );
            events = other;
        }
    }

    let events: Vec<OldEvent> = config_elems_grouped.into_iter()
        .map( |part| {
            let mut event = {
                let year: &str = &part[0].year;
                let week: &str = &part[0].week;
                OldEvent {
                    year: year.parse().unwrap(),
                    week: week.parse().unwrap(),
                    photos: Vec::new()
                }
            };
            let photos: Vec<OldPhotoInfo> = part.into_iter()
                .filter_map( |photo| {
                    config.users.iter()
                        .find( |u| u.old_name == photo.username )
                        .map( |u| OldPhotoInfo{
                            user_id: u.id,
                            old_user_name: photo.username.clone(),
                            new_user_name: u.new_name.clone(),
                            name: photo.subject.clone()
                        })
                })
                .collect();

            event.photos = photos;
            event
        })
        .collect();

    TransferConfig {
        from_weekly_dir: config.from_weekly_dir,
        to_data_dir: config.to_data_dir,
        group_id: config.group_id,
        events: events.into_iter().rev().collect(),
        users: config.users.into_iter().map( |u| u.id ).collect()
    }
}
