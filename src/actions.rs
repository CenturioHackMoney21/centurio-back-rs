use diesel::prelude::*;

use crate::models;

/// Run query using Diesel to find user by uid and return it.
pub fn find_cover_by_address(
    address: String,
    conn: &SqliteConnection,
) -> Result<Option<models::Cover>, diesel::result::Error> {
    use crate::schema::covers::dsl::*;

    let cover = covers
        .filter(address.eq(address))
        .first::<models::Cover>(conn)
        .optional()?;

    Ok(cover)
}
