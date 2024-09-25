use diesel::prelude::*;
use crate::schema::files;
use crate::models::entity::file::File;
use crate::pool::app_state::DbPool;

pub struct FileDao;

impl FileDao {



    pub fn create_file(pool: &DbPool, file: &File) -> QueryResult<usize> {

        let mut conn = pool.get().expect("Failed to get a connection from the pool");

        diesel::insert_into(files::table)

            .values(file)

            .execute(&mut conn)

    }

    pub fn get_file_by_id(pool: &DbPool, file_id: i32) -> QueryResult<File> {



        let mut conn = pool.get().expect("Failed to get a connection from the pool");



        files::table.find(file_id).first(&mut conn)



    }



    pub fn update_file(pool: &DbPool, file_id: i32, file: &File) -> QueryResult<usize> {



        let mut conn = pool.get().expect("Failed to get a connection from the pool");



        diesel::update(files::table.find(file_id))



            .set((
                files::path.eq(&file.path),
                files::file_type.eq(&file.file_type),
                files::size.eq(&file.size),
                files::created_at.eq(&file.created_at),
                files::updated_at.eq(&file.updated_at),
                files::permissions.eq(&file.permissions),
            ))

            .execute(&mut conn)



    }



    pub fn delete_file(pool: &DbPool, file_id: i32) -> QueryResult<usize> {



        let mut conn = pool.get().expect("Failed to get a connection from the pool");



        diesel::delete(files::table.find(file_id)).execute(&mut conn)



    }


   }
