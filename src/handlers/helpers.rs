use answer_types::{ PaginationInfo };

pub fn make_pagination( current: u32, all_count: u32, in_page_count: u32 ) -> PaginationInfo {
    let mut pages_count = all_count / in_page_count;
    if ( all_count % in_page_count ) != 0 {
        pages_count += 1;
    }
    PaginationInfo {
        current: current,
        count: pages_count
    }
}
