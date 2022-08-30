use super::context::GraphQLContext;
use crate::models::users::{UserMutation, UserQuery};
use crate::models::clients::{ClientMutation, ClientQuery};
use crate::models::worksites::{CreateNewWorksite, NewWorksite, Worksite, WorksiteContent};
use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext, description = "Query Root")]
impl Query {
    fn users(&self) -> UserQuery {
        UserQuery
    }

    fn clients(&self) -> ClientQuery {
        ClientQuery
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

}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext, description = "Mutation Root")]
impl Mutation {

    fn users(&self) -> UserMutation {
        UserMutation
    }

    fn clients(&self) -> ClientMutation {
        ClientMutation
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
