use crate::db::schema::users;
use crate::app::models::{User, UserRole, RegisterRequest};
use diesel::prelude::*;
#[cfg(feature = "ssr")]
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc, NaiveDateTime};
#[cfg(feature = "ssr")]
use uuid::Uuid;
#[cfg(feature = "ssr")]
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct SqlUser {
    pub id: i32,
    pub uuid: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uuid: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[cfg(feature = "ssr")]
impl SqlUser {
    pub fn create_user(
        conn: &mut PgConnection,
        register_req: RegisterRequest,
    ) -> Result<SqlUser, diesel::result::Error> {
        let password_hash = hash(&register_req.password, DEFAULT_COST)
            .map_err(|_| diesel::result::Error::RollbackTransaction)?;
        
        let now = Utc::now().naive_utc();
        let new_user = NewUser {
            uuid: Uuid::new_v4().to_string(),
            email: register_req.email,
            password_hash,
            first_name: register_req.first_name,
            last_name: register_req.last_name,
            phone: register_req.phone,
            role: "customer".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .first(conn)
    }

    pub fn get_user_by_email(
        conn: &mut PgConnection,
        email: &str,
    ) -> Result<SqlUser, diesel::result::Error> {
        users::table
            .filter(users::email.eq(email))
            .first(conn)
    }

    pub fn get_user_by_id(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<SqlUser, diesel::result::Error> {
        users::table
            .filter(users::id.eq(user_id))
            .first(conn)
    }

    pub fn get_all_users(
        conn: &mut PgConnection,
    ) -> Result<Vec<SqlUser>, diesel::result::Error> {
        users::table
            .filter(users::is_active.eq(true))
            .load(conn)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    pub fn update_user(
        conn: &mut PgConnection,
        user_id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
    ) -> Result<(), diesel::result::Error> {
        let now = Utc::now().naive_utc();
        
        if let Some(fname) = first_name {
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set((users::first_name.eq(fname), users::updated_at.eq(now)))
                .execute(conn)?;
        }
        
        if let Some(lname) = last_name {
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set((users::last_name.eq(lname), users::updated_at.eq(now)))
                .execute(conn)?;
        }
        
        if let Some(phone_num) = phone {
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set((users::phone.eq(phone_num), users::updated_at.eq(now)))
                .execute(conn)?;
        }

        Ok(())
    }

    pub fn deactivate_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<(), diesel::result::Error> {
        let now = Utc::now().naive_utc();
        
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((users::is_active.eq(false), users::updated_at.eq(now)))
            .execute(conn)?;

        Ok(())
    }

    pub fn to_app_model(self) -> User {
        User {
            id: self.id,
            uuid: self.uuid,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
            role: UserRole::from(self.role),
            is_active: self.is_active,
            created_at: DateTime::from_naive_utc_and_offset(self.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(self.updated_at, Utc),
        }
    }

    pub fn to_app_models(users: Vec<SqlUser>) -> Vec<User> {
        users.into_iter().map(|u| u.to_app_model()).collect()
    }
}

// Client-side implementation (no database operations)
#[cfg(not(feature = "ssr"))]
impl SqlUser {
    pub fn to_app_model(self) -> User {
        User {
            id: self.id,
            uuid: self.uuid,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
            role: UserRole::from(self.role),
            is_active: self.is_active,
            created_at: DateTime::from_naive_utc_and_offset(self.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(self.updated_at, Utc),
        }
    }

    pub fn to_app_models(users: Vec<SqlUser>) -> Vec<User> {
        users.into_iter().map(|u| u.to_app_model()).collect()
    }
} 