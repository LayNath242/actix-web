// use diesel::PgConnection;
// use diesel::BelongingToDsl;
use chrono::NaiveDate;
// use crate::schema;
use crate::schema::courses;
// use crate::schema::user_courses;
// use crate::db_connection::PgPooledConnection;



#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[table_name="courses"]
pub struct Course {
    pub id: i32,
    pub title: String,
    pub thumbnail: Option<String>,
    pub video_url: Option<String>,
    pub description: Option<String>,
    pub cate_id: i32,
    pub price: f64,
    pub created_at: NaiveDate,   
}



#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name="courses"]
pub struct NewCourse {
    pub title: String,
    pub thumbnail: Option<String>,
    pub video_url: Option<String>,
    pub description: Option<String>,
    pub cate_id: Option<i32>,
    pub price: f64,
    pub created_at: NaiveDate, 
}
