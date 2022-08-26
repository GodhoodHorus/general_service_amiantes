use super::context::GraphQLContext;
use crate::models::users::{AuthenticateUser, Authorization, CreateUserInput, NewUser, User};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::models::clients::{Client, ClientInput, Interlocutor, Interlocutors, NewClient, AddressInput, Address};
use crate::models::worksites::{CreateNewWorksite, NewWorksite, Worksite, WorksiteContent};
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext, description = "Query Root")]
impl Query {
    #[graphql(description = "Fetch a user")]
    fn user(context: &GraphQLContext, user_id: i32) -> FieldResult<User> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;

        let conn = context.pool.get()?;

        let user = users.find(user_id).first(&conn);

        if user.is_ok() {
            Ok(user.unwrap())
        } else {
            Err(FieldError::new(
                "Could not get user",
                graphql_value!({ "authentication_error": "Invalid value" }),
            ))
        }

    }

    #[graphql(description = "Fetch a client")]
    fn client(context: &GraphQLContext, client_id: i32) -> FieldResult<Client> {
        use crate::schema::clients::dsl::*;
        use diesel::prelude::*;

        let conn = context.pool.get()?;

        let client = clients.find(client_id).get_result::<Client>(&conn);

        if client.is_ok() {
            Ok(client.unwrap())
        } else {
            Err(FieldError::new(
                "Could not get client",
                graphql_value!({ "authentication_error": "Invalid value" }),
            ))
        }

    }

    #[graphql(description = "Fetch a clients")]
    fn clients(context: &GraphQLContext, mut offset: i32) -> FieldResult<Vec<Client>> {
        use crate::schema::clients::dsl::*;
        use diesel::prelude::*;

        let conn = context.pool.get()?;

        let client = clients
            .limit(10)
            .offset(offset.into())
            .get_results::<Client>(&conn);

        if client.is_ok() {
            Ok(client.unwrap())
        } else {
            Err(FieldError::new(
                "Could not get client",
                graphql_value!({ "authentication_error": "Invalid value" }),
            ))
        }

    }

    #[graphql(description = "Fetch a worksite")]
    fn worksite(context: &GraphQLContext, worksite_id: i32) -> FieldResult<Worksite> {
        use crate::schema::worksites::dsl::*;
        use diesel::prelude::*;

        let conn = context.pool.get()?;

        let worksite_result = worksites.find(worksite_id).first::<Worksite>(&conn);

        if worksite_result.is_ok() {
            Ok(worksite_result.unwrap())
        } else {
            Err(FieldError::new(
                "Could not get worksite",
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

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext, description = "Mutation Root")]
impl Mutation {
    #[graphql(description = "Authenticate a user")]
    fn authenticate_user(context: &GraphQLContext, input: AuthenticateUser) -> FieldResult<User> {
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

    //  ██████╗██████╗ ███████╗ █████╗ ████████╗███████╗██╗   ██╗███████╗███████╗██████╗
    // ██╔════╝██╔══██╗██╔════╝██╔══██╗╚══██╔══╝██╔════╝██║   ██║██╔════╝██╔════╝██╔══██╗
    // ██║     ██████╔╝█████╗  ███████║   ██║   █████╗  ██║   ██║███████╗█████╗  ██████╔╝
    // ██║     ██╔══██╗██╔══╝  ██╔══██║   ██║   ██╔══╝  ██║   ██║╚════██║██╔══╝  ██╔══██╗
    // ╚██████╗██║  ██║███████╗██║  ██║   ██║   ███████╗╚██████╔╝███████║███████╗██║  ██║
    //  ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝ ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝
    #[graphql(description = "create a new user")]
    fn create_user(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<User> {
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

    //  ██████╗██████╗ ███████╗ █████╗ ████████╗███████╗ ██████╗██╗     ██╗███████╗███╗   ██╗████████╗
    // ██╔════╝██╔══██╗██╔════╝██╔══██╗╚══██╔══╝██╔════╝██╔════╝██║     ██║██╔════╝████╗  ██║╚══██╔══╝
    // ██║     ██████╔╝█████╗  ███████║   ██║   █████╗  ██║     ██║     ██║█████╗  ██╔██╗ ██║   ██║
    // ██║     ██╔══██╗██╔══╝  ██╔══██║   ██║   ██╔══╝  ██║     ██║     ██║██╔══╝  ██║╚██╗██║   ██║
    // ╚██████╗██║  ██║███████╗██║  ██║   ██║   ███████╗╚██████╗███████╗██║███████╗██║ ╚████║   ██║
    //  ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝ ╚═════╝╚══════╝╚═╝╚══════╝╚═╝  ╚═══╝   ╚═╝
    #[graphql(description = "create a new client")]
    fn create_client(context: &GraphQLContext, input: ClientInput) -> FieldResult<Client> {
        use crate::schema::clients::dsl::*;
        use diesel::prelude::*;

        // let input_iterators = input.interlocutors;
        //
        // let mut received_interlocutor = vec![];
        //
        // received_interlocutor.extend(input_iterators.unwrap().interlocutors_list.into_iter().map(
        //     |i| Interlocutor {
        //         name: i.name,
        //         position: i.position,
        //     },
        // ));
        //
        // let new_interlocutors = Interlocutors {
        //     interlocutors_list: received_interlocutor,
        // };

        let new_client: NewClient = NewClient {
            name: &input.name,
            address: &diesel_json::Json::new(input.address.unwrap().into()),
            interlocutors:  None,
            // interlocutors: &diesel_json::Json::new(new_interlocutors),
            created_at: &chrono::offset::Utc::now().naive_utc(),
            edited_at: &chrono::offset::Utc::now().naive_utc(),
        };

        let conn = context.pool.get()?;

        let created_client = diesel::insert_into(clients)
            .values(new_client)
            .get_result(&conn);

        if created_client.is_ok() {
            Ok(created_client.unwrap())
        } else {
            Err(FieldError::new(
                "Could not create the client",
                graphql_value!({ "authentication_error": "Invalid value" }),
            ))
        }
    }

    #[graphql(description = "update a client address")]
    fn update_client_address(context: &GraphQLContext, receive_id: i32, input: AddressInput) -> FieldResult<Client> {
        use crate::schema::clients::dsl::*;
        use diesel::prelude::*;

        let conn  = context.pool.get()?;

        let other_address = Address {
            street: input.street,
            street_number: input.street_number,
        };

        let update_client_address = diesel::update(clients)
            .filter(id.eq_all(receive_id))
            .set(address.eq_all(&diesel_json::Json::new(other_address)))
            .get_result::<Client>(&conn);

        if update_client_address.is_ok() {
            Ok(update_client_address.unwrap())
        } else {
            Err(FieldError::new(
                "Could not update the address of the client",
                graphql_value!({ "authentication_error": "Invalid value" }),
            ))
        }


    }

    //  ██████╗██████╗ ███████╗ █████╗ ████████╗███████╗██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗███████╗██╗████████╗███████╗
    // ██╔════╝██╔══██╗██╔════╝██╔══██╗╚══██╔══╝██╔════╝██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝██╔════╝██║╚══██╔══╝██╔════╝
    // ██║     ██████╔╝█████╗  ███████║   ██║   █████╗  ██║ █╗ ██║██║   ██║██████╔╝█████╔╝ ███████╗██║   ██║   █████╗
    // ██║     ██╔══██╗██╔══╝  ██╔══██║   ██║   ██╔══╝  ██║███╗██║██║   ██║██╔══██╗██╔═██╗ ╚════██║██║   ██║   ██╔══╝
    // ╚██████╗██║  ██║███████╗██║  ██║   ██║   ███████╗╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗███████║██║   ██║   ███████╗
    //  ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝ ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝   ╚═╝   ╚══════╝
    #[graphql(description = "Create a new worksite associated with a client id")]
    fn create_worksite(
        context: &GraphQLContext,
        input: CreateNewWorksite,
    ) -> FieldResult<Worksite> {
        use crate::schema::worksites::dsl::*;
        use diesel::prelude::*;

        let new_worksite: WorksiteContent = input.worksite.expect("No worksite content set").into();

        let new_worksite: NewWorksite = NewWorksite {
            client_id: &input.client_id,
            worksite: &diesel_json::Json::new(new_worksite),
            created_at: &chrono::offset::Utc::now().naive_utc(),
            edited_at: &chrono::offset::Utc::now().naive_utc(),
        };

        let conn = context.pool.get()?;

        let created_worksite = diesel::insert_into(worksites)
            .values(new_worksite)
            .get_result(&conn);

        Ok(created_worksite.expect("Could not create a client"))
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}
