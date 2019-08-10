use crate::schema::user_courses;
use crate::models::course::Course;
use crate::models::user::User;


#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[table_name="user_courses"]
#[belongs_to(User)]
#[belongs_to(Course)]
pub struct UserCourse {
    pub id: i32,
    pub course_id: i32,
    pub user_id: i32,
}


#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, PartialEq)]
#[table_name="user_courses"]
pub struct NewUserCourse {
    pub course_id: Option<i32> ,
    pub user_id: Option<i32> ,
}
