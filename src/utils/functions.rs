use super::traits::Error;
use time::macros::format_description;
use time::Date;

pub fn convert_date<T: Error>(created_on: String) -> Result<Date, T> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(created_on.as_str(), &format).map_err(|e| {
        T::new(format!(
            "failed to create anime due to an invalid date string provided: {e:#?}"
        ))
    })
}
