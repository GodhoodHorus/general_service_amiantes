use crate::schema::users;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub authorization_id: i32,
    pub name: String,
    pub password: String,
}

#[juniper::graphql_object]
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

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub authorization_id: &'a i32,
    pub name: &'a String,
    pub password: &'a String,
}

#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub authorization_id: i32,
    pub name: String,
    pub password: String,
}

#[derive(GraphQLInputObject)]
pub struct AuthenticateUser {
    pub name: String,
    pub password: String,
}

#[derive(Queryable, Serialize)]
pub struct Authorization {
    pub id: i32,
    pub level: String,
}

#[juniper::graphql_object]
impl Authorization {
    fn id(&self) -> i32 {
        self.id
    }

    fn level(&self) -> &str {
        self.level.as_str()
    }
}
