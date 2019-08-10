use diesel::PgConnection;
use chrono::NaiveDateTime;
use crate::models::user::User;
use crate::schema::{categories, users};


#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>, 
}


#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, Associations ,PartialEq)]
#[belongs_to(User)]
#[table_name="categories"]
pub struct NewCategory {
    pub title: String,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>, 
}


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CategoryByUser {
    pub id: i32,
    pub title: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
}


type Columns = (
    categories::id,
    categories::title,
    users::fullname,
    categories::created_at,
    
);

const COLUMNS: Columns = (
    categories::id,
    categories::title,
    users::fullname,
    categories::created_at, 
);


#[derive(Serialize, Deserialize)]
pub struct Categorieslist(pub Vec<CategoryByUser>);


impl Categorieslist {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        // use crate::schema::categories::dsl::*;

        let result = 
            users::table
                .inner_join(categories::table)
                .select(COLUMNS)
                .load::<CategoryByUser>(connection)           
                .expect("Error loading data");
        Categorieslist(result)
    }
}


impl NewCategory {
     pub fn create(&self, connection: &PgConnection) ->
        Result<Category, diesel::result::Error> {
            use diesel::RunQueryDsl;

                diesel::insert_into(categories::table)
                    .values(self)
                    .get_result(connection)
        }
}

impl Category {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Category, diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;

        categories::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::categories::dsl;

        diesel::delete(dsl::categories
        .find(id))
        .execute(connection)?;
        Ok(())
    }


    pub fn update(id: &i32,
                  new_category : &NewCategory, 
                  connection: &PgConnection) 
                  -> Result<(), diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::categories::dsl;

        diesel::update(dsl::categories.find(id))
            .set(new_category)
            .execute(connection)?;
            Ok(())
    }   

}