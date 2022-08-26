use crate::schema::clients;
use chrono::NaiveDateTime;
use diesel_json::Json;
use std::ops::Deref;

// Define the json structure of the Address
#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Address {
    pub street: String,
    pub street_number: i32,
}

// Define the json structure of a single interlocutor
#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Interlocutor {
    pub name: String,
    pub position: String,
}

// Define the json structure of the interlocutors
#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Interlocutors {
    pub interlocutors_list: Vec<Interlocutor>,
}

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub address: Json<Address>,
    pub interlocutors: Json<Interlocutors>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
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
        self.address.deref().to_owned()
    }

    fn interlocutors(&self) -> &Interlocutors {
        self.interlocutors.deref().to_owned()
    }

    fn created_at(&self) -> String {
        self.created_at.format("%d-%m-%Y %M:%S:%f").to_string()
    }

    fn edited_at(&self) -> String {
        self.edited_at.format("%d-%m-%Y %M:%S:%f").to_string()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "clients"]
pub struct NewClient<'a> {
    pub name: &'a String,
    pub address: &'a Json<Address>,
    pub interlocutors: &'a Json<Interlocutors>,
    pub created_at: &'a NaiveDateTime,
    pub edited_at: &'a NaiveDateTime,
}

#[derive(GraphQLInputObject)]
pub struct CreateClientInput {
    pub name: String,
    pub address: Option<CreateAddressInput>,
    pub interlocutors: Option<CreateInterlocutorsInput>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize)]
pub struct CreateAddressInput {
    pub street: String,
    pub street_number: i32,
}

impl From<CreateAddressInput> for Address {
    fn from(f: CreateAddressInput) -> Self {
        let serialised = serde_json::to_string(&f).unwrap();
        serde_json::from_str::<Address>(&serialised).unwrap()
    }
}

// Define the json structure of a single interlocutor
#[derive(GraphQLInputObject, Serialize, Deserialize, Clone)]
pub struct CreateInterlocutorInput {
    pub name: String,
    pub position: String,
}

impl From<CreateInterlocutorInput> for Interlocutor {
    fn from(f: CreateInterlocutorInput) -> Self {
        let serialised = serde_json::to_string(&f).unwrap();
        serde_json::from_str::<Interlocutor>(&serialised).unwrap()
    }
}

impl Extend<&'static CreateInterlocutorInput> for Vec<Interlocutor> {
    fn extend<T: IntoIterator<Item = &'static CreateInterlocutorInput>>(&mut self, iter: T) {
        for item in iter {
            self.push(Interlocutor {
                name: item.name.to_owned(),
                position: item.position.to_owned(),
            })
        }
    }
}

// Define the json structure of the interlocutors
#[derive(GraphQLInputObject, Clone)]
pub struct CreateInterlocutorsInput {
    pub interlocutors_list: Vec<CreateInterlocutorInput>,
}
