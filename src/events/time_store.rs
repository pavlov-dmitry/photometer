use time::{ Timespec };
use std::old_io::{ File, IoErrorKind };
use types::{ EmptyResult, CommonResult };

/// хранилище времени на базе файла
pub struct TimeStore {
	store_file_path: Path
}

impl TimeStore {
	/// создание нового хранилища, передаётся путь к файлу, в котором будет храниться время
	pub fn new( path: Path ) -> TimeStore {
		TimeStore {
			store_file_path: path
		}
	}
	/// запоминается текущий момент
	pub fn remember_this_moment(&self) -> EmptyResult {
		match File::create( &self.store_file_path ) {
			Ok( _ ) => Ok( () ),
			Err( e ) => Err( format!( "TimeStore remember_this_moment failed: {}", e ) )
		}
	}
	/// читается время из хранилища которые было раньше запомнено, None елси момент не был еще запомнен
	pub fn get_stored_time(&self) -> CommonResult<Option<Timespec>> {
		match File::open( &self.store_file_path ) {
			Ok( file ) => file.stat().map( |stat| 
				Some( Timespec::new( ( stat.modified / MSECS_IN_SEC ) as i64, ( stat.modified % MSECS_IN_SEC ) as i32 ) )
			),
			Err( ref e ) if e.kind == IoErrorKind::FileNotFound => Ok( None ),
			Err( e ) => Err( e )
		}.map_err( |e| format!( "TimeStore get_stored_time failed: {}", e ) )
	}
}

const MSECS_IN_SEC : u64 = 1000;