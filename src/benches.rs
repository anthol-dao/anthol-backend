use super::*;
use canbench_rs::bench;

#[bench]
fn test_get_account_data() {
    let _ = get_account_page_data();
}
