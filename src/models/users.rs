use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use juniper::{FieldError, FieldResult, graphql_value};
use crate::GraphQLContext;
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub authorization_id: i32,
    pub name: String,
    pub password: String,
}

#[juniper::graphql_object]
#[graphql(description = "User of the application")]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn authorization_id(&self) -> i32 {
        self.authorization_id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn password(&self) -> &str {
        self.password.as_str()
    }

    fn authorization(&self) -> Vec<Authorization> {
        use crate::get_pool;
        use crate::schema::authorizations::dsl::*;
        use diesel::prelude::*;

        let query = authorizations
            .filter(id.eq_all(self.authorization_id))
            .limit(3)
            .load::<Authorization>(&get_pool().get().expect("Database not working"))
            .expect("Error loading Authorizations");

        query
    }
}

#[derive(Queryable, Serialize, GraphQLObject)]
pub struct Authorization {
    pub id: i32,
    pub level: String,
}


#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub authorization_id: &'a i32,
    pub name: &'a String,
    pub password: &'a String
}

#[derive(Debug, Serialize, GraphQLInputObject)]
pub struct Authenticate {
    pub name: String,
    pub password: String
}

#[derive(Debug, Serialize, GraphQLInputObject)]
#[graphql(description = "User of the application")]
pub struct UserInput {
    pub authorization_id: i32,
    pub name: String,
    pub password: String,
}


pub struct UserQuery;

#[juniper::graphql_object(Context = GraphQLContext)]
impl UserQuery {
    #[graphql(description = "Fetch a user")]
    pub fn fetch(&self, context: &GraphQLContext, user_id: i32) -> FieldResult<User> {
        let conn = context.pool.get()?;

        let user = users.find(user_id).first(&conn);

        if user.is_ok() {
            Ok(user.unwrap())
        } else {
            Err(FieldError::new(
                "User does not not exist",
                graphql_value!({ "authentication_error": "Invalid value" }),
            ))
        }
    }

    #[graphql(description = "Get user Authorization")]
    fn authorization(context: &GraphQLContext) -> FieldResult<Vec<Authorization>> {
        use crate::schema::authorizations::dsl::*;
        use diesel::prelude::*;

        let conn = context.pool.get().expect("Data pool");

        let query = authorizations
            .limit(3)
            .load::<Authorization>(&conn)
            .expect("Error loading Authorizations");

        Ok(query)
    }

}

pub struct UserMutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl UserMutation {
    #[graphql(description = "create a new user")]
    fn create(context: &GraphQLContext, input: UserInput) -> FieldResult<User> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;

        // Random salt for each password for better security
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2
            .hash_password((input.password).as_bytes(), &salt)?
            .to_string();

        let new_user: NewUser = NewUser {
            authorization_id: &input.authorization_id,
            name: &input.name,
            password: &password_hash,
        };

        let conn = context.pool.get()?;

        let created_user = diesel::insert_into(users)
            .values(new_user)
            .get_result(&conn);

        Ok(created_user.expect("Could not create user"))
    }

    #[graphql(description = "Authenticate a user")]
    fn authenticate_user(context: &GraphQLContext, input: Authenticate) -> FieldResult<User> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;

        let conn = context.pool.get().expect("Data pool");

        let user: User = users
            .filter(name.eq_all(input.name))
            .first::<User>(&conn)
            .expect("No user found with this credentials");

        let parsed_hash = PasswordHash::new(&user.password)?;

        if Argon2::default()
            .verify_password((input.password).as_bytes(), &parsed_hash)
            .is_ok()
        {
            Ok(user)
        } else {
            Err(FieldError::new(
                "Could not match the name/password combination",
                graphql_value!({ "authentication_error": "Invalid value" }),
            ))
        }
    }
}