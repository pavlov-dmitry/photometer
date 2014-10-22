#[allow(dead_code)]
// возвращает итератор "указателей" на последовательности которые ограничены с двух сторон другой последовательностью
pub fn boundary<'a, 'b, T:'a + 'b + PartialEq>( data: &'a [T], sequence: &'b [T] ) -> BoundaryIter<'a, 'b, T> {
    //ищем начало
    for start in range( 0, data.len() ) {
        if data.slice_from( start ).starts_with( sequence ) {
            return BoundaryIter {
                data : data.slice_from( start + sequence.len() ),
                sequence : sequence
            }
        }
    }
    //елис не найдено то типа закончили
    BoundaryIter {
        data: data.slice(0, 0),
        sequence: sequence,
    }
}
pub struct BoundaryIter<'a, 'b, T:'a + 'b + PartialEq> {
    data: &'a [T],
    sequence: &'b [T]
}
impl<'a, 'b, T:'a + PartialEq> Iterator<&'a [T]> for BoundaryIter<'a, 'b, T> {
    fn next(&mut self) -> Option<&'a [T]> {
        let mut result = None;
        for pos in range( 0, self.data.len() + 1 ) {
            if self.data.slice_to( pos ).ends_with( self.sequence ) {
                result = Some( self.data.slice_to( pos - self.sequence.len() ) );
                self.data = self.data.slice_from( pos );
                break;
            }
        }
        result
    }
}


// возвращает итератор индесков (от, до) на последовательности которые ограничены с двух сторон другой последовательностью
pub fn boundary_idx<'a, T:'a + PartialEq>( data: &'a [T], sequence: &'a [T] ) -> BoundaryIdxIter<'a, T> {
    //ищем начало
    for start in range( 0, data.len() ) {
        if data.slice_from( start ).starts_with( sequence ) {
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
    idx: uint
}
impl<'a, T:'a + PartialEq> Iterator<(uint, uint)> for BoundaryIdxIter<'a, T> {
    fn next(&mut self) -> Option<(uint, uint)> {
        let mut result = None;
        for pos in range( self.idx, self.data.len() + 1 ) {
            if self.data.slice( self.idx, pos ).ends_with( self.sequence ) {
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
    for pos in range( 0, data.len() ) {
        if data.slice_from( pos ).starts_with( sequence ) {
            return Some( (data.slice_to( pos ), data.slice_from( pos + sequence.len() ) ) )
        }
    }
    None
}

// делит последовательность на (до, после) опеределённой последовательности или её альтернативой
pub fn split_seq_alt<'a, T:'a + PartialEq>( data: &'a [T], seq: &'a [T], alt: &'a [T] ) -> Option<(&'a [T], &'a [T])> {
    for pos in range( 0, data.len() ) {
        let current_slice = data.slice_from( pos );
        if current_slice.starts_with( seq ) {
            return Some( (data.slice_to( pos ), data.slice_from( pos + seq.len() ) ) )
        } 
        else if current_slice.starts_with( alt ) {
            return Some( (data.slice_to( pos ), data.slice_from( pos + alt.len() ) ) )  
        }
    }
    None
}

// находит подстроку находящуюся между двумя подстроками
pub fn str_between<'a>( source: &'a str, start: &'a str, end: &'a str ) -> Option<&'a str> {
    source
        .find_str( start )
        .and_then( |start_pos| {
            let after_start = source.slice_from( start_pos + start.len() );
            after_start
                .find_str( end )
                .and_then( |end_pos| Some( after_start.slice_to( end_pos ) ) )
        } )
}