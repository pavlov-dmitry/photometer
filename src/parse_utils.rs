use time;

#[allow(dead_code)]
// возвращает итератор "указателей" на последовательности которые ограничены с двух сторон другой последовательностью
pub fn boundary<'a, 'b, T:'a + 'b + PartialEq>( data: &'a [T], sequence: &'b [T] ) -> BoundaryIter<'a, 'b, T> {
    //ищем начало
    for start in 0 .. data.len() {
        if data[start ..].starts_with( sequence ) {
            return BoundaryIter {
                data : &data[start + sequence.len() ..],
                sequence : sequence
            }
        }
    }
    //елис не найдено то типа закончили
    BoundaryIter {
        data: &data[0..0],
        sequence: sequence,
    }
}
pub struct BoundaryIter<'a, 'b, T:'a + 'b + PartialEq> {
    data: &'a [T],
    sequence: &'b [T]
}
impl<'a, 'b, T:'a + 'b + PartialEq> Iterator for BoundaryIter<'a, 'b, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        let mut result = None;
        for pos in 0 .. self.data.len() + 1 {
            if self.data[.. pos].ends_with( self.sequence ) {
                result = Some( &self.data[.. pos - self.sequence.len()] );
                self.data = &self.data[pos ..];
                break;
            }
        }
        result
    }
}


// возвращает итератор индесков (от, до) на последовательности которые ограничены с двух сторон другой последовательностью
pub fn boundary_idx<'a, T:'a + PartialEq>( data: &'a [T], sequence: &'a [T] ) -> BoundaryIdxIter<'a, T> {
    //ищем начало
    for start in 0 .. data.len() {
        if data[start ..].starts_with( sequence ) {
            return BoundaryIdxIter {
                data : data,
                sequence : sequence,
                idx :  start + sequence.len()
            }
        }
    }
    //елис не найдено то типа закончили
    BoundaryIdxIter {
        data: data,
        sequence: sequence,
        idx: data.len()
    }
}
pub struct BoundaryIdxIter<'a, T:'a + PartialEq> {
    data: &'a [T],
    sequence: &'a [T],
    idx: usize
}
impl<'a, T:'a + PartialEq> Iterator for BoundaryIdxIter<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let mut result = None;
        for pos in self.idx .. self.data.len() + 1 {
            if self.data[self.idx .. pos].ends_with( self.sequence ) {
                result = Some( ( self.idx, pos - self.sequence.len() ) );
                self.idx = pos;
                break;
            }
        }
        result
    }
}

#[allow(dead_code)]
// делит последовательность на (до, после) опеределённой последовательности
pub fn split_seq<'a, T:'a + PartialEq>( data: &'a [T], sequence: &'a [T] ) -> Option<(&'a [T], &'a [T])> {
    for pos in 0 .. data.len() {
        if data[pos ..].starts_with( sequence ) {
            return Some( (&data[.. pos], &data[pos + sequence.len() ..]) )
        }
    }
    None
}

// делит последовательность на (до, после) опеределённой последовательности или её альтернативой
pub fn split_seq_alt<'a, T:'a + PartialEq>( data: &'a [T], seq: &'a [T], alt: &'a [T] ) -> Option<(&'a [T], &'a [T])> {
    for pos in 0 .. data.len() {
        let current_slice = &data[pos ..];
        if current_slice.starts_with( seq ) {
            return Some( (&data[.. pos], &data[pos + seq.len() ..] ) )
        }
        else if current_slice.starts_with( alt ) {
            return Some( (&data[.. pos], &data[pos + alt.len() ..] ) )
        }
    }
    None
}

// находит подстроку находящуюся между двумя подстроками
pub fn str_between<'a>( source: &'a str, start: &'a str, end: &'a str ) -> Option<&'a str> {
    source
        .find( start )
        .and_then( |start_pos| {
            let after_start = &source[start_pos + start.len() ..];
            after_start
                .find( end )
                .and_then( |end_pos| Some( &after_start[.. end_pos] ) )
        } )
}

static TIME_FORMAT: &'static str = "%Y.%m.%d %k:%M:%S";

pub fn parse_timespec( s: &str ) -> Result<time::Timespec, time::ParseError> {
    time::strptime( s, TIME_FORMAT )
          .map( |t| t.to_timespec() )
}

pub fn timespec_string( timespec: time::Timespec ) -> String {
    let tm = time::at( timespec );
    time::strftime( TIME_FORMAT, &tm )
        .expect( &format!( "Invalid convertion from timespec to string with format: {}",
                            TIME_FORMAT ) )
}
