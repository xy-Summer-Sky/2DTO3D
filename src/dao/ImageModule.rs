use crate::models::entity::image::Image;
use crate::models::entity::image::NewImage;
use crate::pool::app_state::DbPool;
use crate::schema::images;
use diesel::prelude::*;

pub struct ImageDao;

impl ImageDao {
    pub async fn create_image_and_get_id(
        pool: &DbPool,
        image: &crate::models::entity::image::NewImage,
    ) -> QueryResult<i32> {
        let mut conn = pool
            .get()
            .expect("Failed to get a connection from the pool");
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(images::table)
                .values(image)
                .execute(conn)?;
            let image_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "LAST_INSERT_ID()",
            ))
            .get_result(conn)?;
            Ok(image_id)
        })
    }

    pub fn get_image_by_id(pool: &DbPool, image_id: i32) -> QueryResult<Image> {
        let mut conn = pool
            .get()
            .expect("Failed to get a connection from the pool");
        images::table.find(image_id).first(&mut conn)
    }
}
