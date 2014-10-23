// описывает события к которым может быть привязано фото
pub enum PhotoEvent {
    Weekly {
        pub year : uint,
        pub week : uint
    }
}

pub fn create_weekly( year: uint, week: uint ) -> PhotoEvent {
    Weekly{ 
        year: year,
        week: week  
    }
}