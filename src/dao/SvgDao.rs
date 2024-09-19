// src/dao/SvgDao.rs
use diesel::prelude::*;
use crate::schema::svgs;
use crate::entity::svg::Svg;
use crate::pool::app_state::DbPool;

pub struct SvgDao;

impl SvgDao {
    pub fn create_svg(pool: &DbPool, svg: &Svg) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::insert_into(crate::schema::svgs::table)
            .values(svg)
            .execute(&mut conn)
    }

    pub fn get_svg_by_id(pool: &DbPool, svg_id: i32) -> QueryResult<Svg> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        svgs::table.find(svg_id).first(&mut conn)
    }

    pub fn update_svg(pool: &DbPool, svg_id: i32, svg: &Svg) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::update(svgs::table.find(svg_id))
            .set(svgs::svg_path.eq(&svg.svg_path))
            .execute(&mut conn)
    }

    pub fn delete_svg(pool: &DbPool, svg_id: i32) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::delete(svgs::table.find(svg_id)).execute(&mut conn)
    }
}