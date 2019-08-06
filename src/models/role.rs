use crate::schema::roles;
use diesel::PgConnection;
use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize)]
pub struct RoleList(pub Vec<Role>);

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub title: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset, Associations)]
#[table_name="roles"]
pub struct NewRole {
    pub title: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl RoleList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::roles::dsl::*;

        let result = 
            roles
                .limit(10)
                .load::<Role>(connection)
                .expect("Error loading roles");

        RoleList(result)
    }
}

impl NewRole {
    pub fn create(&self, connection: &PgConnection) -> Result<Role, diesel::result::Error>{
        use diesel::RunQueryDsl;

        diesel::insert_into(roles::table)
            .values(self)
            .get_result(connection)
    }
}


impl Role {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Role, diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;

        roles::table.find(id).first(connection)
    }

    pub fn update(id: &i32,new_role : &NewRole, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::roles::dsl;

        diesel::update(dsl::roles.find(id))
            .set(new_role)
            .execute(connection)?;
            Ok(())    
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::roles::dsl;

        diesel::delete(dsl::roles.find(id))
        .execute(connection)?;
        Ok(())
                
    }
}
