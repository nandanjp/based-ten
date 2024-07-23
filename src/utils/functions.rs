pub fn page_limit(num_rows: i64, limit: Option<i64>, page: Option<i64>) -> (i64, i64) {
    let limit = limit.unwrap_or_default();
    let page = page.unwrap_or_default();
    let limit = if limit == 0 || limit > 100 {
        100
    } else {
        limit
    };
    let pages = num_rows / limit + if num_rows % limit > 0 { 1 } else { 0 };
    let page = if page + 1 > pages { pages } else { page + 1 } - 1;

    (limit, page)
}
