use std::borrow::Borrow;
use crate::schema::{clients, clients::dsl::*};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use diesel_json::Json;
use futures::SinkExt;
use juniper::{FieldError, FieldResult, graphql_value};
use crate::GraphQLContext;

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub address: Json<Address>,
    pub interlocutors: Option<Json<Vec<Interlocutor>>>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Address {
    pub street: String,
    pub street_number: i32,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Interlocutor {
    pub name: String,
    pub position: String,
}

#[juniper::graphql_object]
impl Client {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn address(&self) -> &Address {
        self.address.as_ref()
    }

    fn interlocutors(&self) -> &Vec<Interlocutor> {
        self.interlocutors.as_deref().unwrap()
    }

    fn created_at(&self) -> String {
        self.created_at.format("%d-%m-%Y %M:%S:%f").to_string()
    }

    fn edited_at(&self) -> String {
        self.edited_at.format("%d-%m-%Y %M:%S:%f").to_string()
    }
}

pub struct ClientQuery;

#[juniper::graphql_object(Context = GraphQLContext)]
impl ClientQuery {
    #[graphql(description = "Fetch a client")]
    fn fetch(context: &GraphQLContext, client_id: i32) -> FieldResult<Client> {
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
    fn fetch_all(context: &GraphQLContext, mut offset: i32) -> FieldResult<Vec<Client>> {
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
}

#[derive(Debug, Insertable)]
#[table_name = "clients"]
pub struct NewClient<'a> {
    pub name: &'a String,
    pub address: &'a Json<AddressInput>,
    pub interlocutors: Option<Json<&'a Vec<InterlocutorInput>>>,
    pub created_at: &'a NaiveDateTime,
    pub edited_at: &'a NaiveDateTime,
}

#[derive(Debug, GraphQLInputObject)]
pub struct ClientInput {
    pub name: String,
    pub address: Option<AddressInput>,
    pub interlocutors: Option<Vec<InterlocutorInput>>,
}


#[derive(Debug, GraphQLInputObject, Serialize)]
pub struct AddressInput {
    pub street: String,
    pub street_number: i32,
}

#[derive(Debug, GraphQLInputObject, Serialize, Clone)]
pub struct InterlocutorInput {
    pub name: String,
    pub position: String,
}

pub struct ClientMutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl ClientMutation {
    #[graphql(description = "create a new client")]
    fn create(context: &GraphQLContext, input: ClientInput) -> FieldResult<Client> {
        let received_interlocutors: Option<Json<&Vec<InterlocutorInput>>> = if input.interlocutors.is_none() {
            None
        } else {
            Some(Json::new(input.interlocutors.as_ref().unwrap()))
        };

        let new_client: NewClient = NewClient {
            name: &input.name,
            address: &Json::new(input.address.unwrap()),
            interlocutors:  received_interlocutors,
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
}

