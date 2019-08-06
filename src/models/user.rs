use chrono::{NaiveDateTime, Local};
use crate::schema::{users, roles};
use crate::models::role::Role;

use bcrypt::{hash, DEFAULT_COST};
use diesel::PgConnection;
use crate::errors::MyError;


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable , Associations,  PartialEq)]
#[belongs_to(Role)] 
#[table_name = "users"]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub avatar: Option<String>,
    pub biography: Option<String>,
    pub created_at: NaiveDateTime,
    pub role_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub biography: Option<String>,
    pub created_at: NaiveDateTime,
    pub role_id: i32,
}




#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UserRole {
    pub id: i32,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub fullname: String,
    pub avatar: Option<String>,
    pub biography: Option<String>,
    pub role: String,
    pub created_at: NaiveDateTime,
}

type Columns = (
    users::id,
    users::email,
    users::password,
    users::fullname,
    users::avatar,
    users::biography,
    roles::title,
    users::created_at,
    
);

const COLUMNS: Columns = (
    users::id,
    users::email,
    users::password,
    users::fullname, 
    users::avatar,
    users::biography,
    roles::title,
    users::created_at,     
);




impl User {
    pub fn create(register_user: RegisterUser, connection: &PgConnection) ->
     Result<User, MyError> {
        use diesel::RunQueryDsl;

        Ok(diesel::insert_into(users::table)
            .values(NewUser {
                fullname: register_user.fullname,
                email: register_user.email,
                password: Self::hash_password(register_user.password)?,
                avatar: register_user.avatar,
                biography: register_user.biography,
                created_at: Local::now().naive_local(),
                role_id: register_user.role_id,
            })
            .get_result(connection)?)
    }

    pub fn hash_password(plain: String) -> Result<String, MyError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub avatar: Option<String>,
    pub biography: Option<String>,
    pub role_id: i32,
}

impl RegisterUser {
    pub fn validates(self) ->
     Result<RegisterUser, MyError> {
         if self.password == self.password_confirmation {
             Ok(self)
         } else {
             Err(
                 MyError::PasswordNotMatch(
                     "Password and Password Confirmation does not match".to_string()
                 )
             )
         }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

impl AuthUser {
    pub fn login(&self, connection: &PgConnection) ->
     Result<UserRole, MyError> {
        use bcrypt::verify;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::users::dsl::email;

        let mut records =
            users::table 
                    .filter(email.eq(&self.email))
                    .inner_join(roles::table)
                    .select(COLUMNS)
                    .load::<(UserRole)>(connection)?;

        let user =
            records
                .pop()
                .ok_or(MyError::DBError(diesel::result::Error::NotFound))?;

        let verify_password =
            verify(&self.password, &user.password)
                .map_err( |_error| {
                    MyError::WrongPassword(
                        "Wrong password, check again please".to_string()
                    )
                })?;

        if verify_password {
            Ok(user)
        } else {
            Err(MyError::WrongPassword(
                "Wrong password, check again please".to_string()
            ))
        }
    }
}

