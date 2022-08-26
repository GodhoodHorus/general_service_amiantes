use crate::schema::worksites;
use chrono::NaiveDateTime;
use diesel_json::Json;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksiteInformation {
    pub folder_number: String,
}

#[juniper::graphql_object]
impl WorksiteInformation {
    pub fn folder_number(&self) -> &str {
        self.folder_number.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Asbestos {
    pub unit: i32,
    pub area: String,
    pub equipments: String,
    pub localization: String,
    pub surveyed_element: String,
    pub materials_description: String,
    pub sampling: String,
    pub date_of_sampling: String,
    pub fcr_result: String,
    pub conservation_state: String,
    pub equipment_volume: String,
    pub material_volume: String,
    pub picture_id: i32,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Lead {
    pub number: i32,
    pub localization: String,
    pub area: String,
    pub number_ud: i32,
    pub diagnostic_unity: String,
    pub substrate: String,
    pub exposed_coating: String,
    pub measure_localization: String,
    pub measure: i32,
    pub incertitude: i32,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksiteContent {
    pub worksite_information: Option<WorksiteInformation>,
    pub leads: Option<Vec<Lead>>,
    pub asbestos: Option<Vec<Asbestos>>,
}

#[juniper::graphql_object]
impl WorksiteContent {
    fn worksite_information(&self) -> Option<&WorksiteInformation> {
        if self.worksite_information.is_none() {
            None
        } else {
            self.worksite_information.as_ref()
        }
    }

    fn leads(&self) -> Option<&Vec<Lead>> {
        if self.leads.is_none() {
            None
        } else {
            self.leads.as_ref()
        }
    }

    fn asbestos(&self) -> Option<&Vec<Asbestos>> {
        if self.asbestos.is_none() {
            None
        } else {
            self.asbestos.as_ref()
        }
    }
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Worksite {
    pub id: i32,
    pub client_id: i32,
    pub worksite: Json<WorksiteContent>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[juniper::graphql_object()]
impl Worksite {
    fn id(&self) -> i32 {
        self.id
    }

    fn client_id(&self) -> i32 {
        self.client_id
    }

    fn worksite(&self) -> &WorksiteContent {
        self.worksite.deref().to_owned()
    }

    fn created_at(&self) -> String {
        self.created_at.format("%d-%m-%Y %M:%S:%f").to_string()
    }

    fn edited_at(&self) -> String {
        self.created_at.format("%d-%m-%Y %M:%S:%f").to_string()
    }

    fn deleted_at(&self) -> Option<String> {
        if !self.deleted_at.is_none() {
            Some(self.created_at.format("%d-%m-%Y %M:%S:%f").to_string())
        } else {
            None
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "worksites"]
pub struct NewWorksite<'a> {
    pub client_id: &'a i32,
    pub worksite: &'a Json<WorksiteContent>,
    pub created_at: &'a NaiveDateTime,
    pub edited_at: &'a NaiveDateTime,
}

#[derive(GraphQLInputObject)]
pub struct CreateNewWorksite {
    pub client_id: i32,
    pub worksite: Option<CreateWorksiteContent>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize)]
pub struct CreateWorksiteContent {
    pub worksite_information: Option<CreateWorksiteInformation>,
    pub leads: Option<Vec<CreateLead>>,
    pub asbestos: Option<Vec<CreateAsbestos>>,
}

impl From<CreateWorksiteContent> for WorksiteContent {
    fn from(f: CreateWorksiteContent) -> Self {
        let serialised = serde_json::to_string(&f).unwrap();
        serde_json::from_str::<WorksiteContent>(&serialised).unwrap()
    }
}

#[derive(GraphQLInputObject, Serialize, Deserialize)]
pub struct CreateWorksiteInformation {
    pub folder_number: String,
}

impl From<CreateWorksiteInformation> for WorksiteInformation {
    fn from(f: CreateWorksiteInformation) -> Self {
        let serialised = serde_json::to_string(&f).unwrap();
        serde_json::from_str::<WorksiteInformation>(&serialised).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct CreateAsbestos {
    pub unit: i32,
    pub area: String,
    pub equipments: String,
    pub localization: String,
    pub surveyed_element: String,
    pub materials_description: String,
    pub sampling: String,
    pub date_of_sampling: String,
    pub fcr_result: String,
    pub conservation_state: String,
    pub equipment_volume: String,
    pub material_volume: String,
    pub picture_id: i32,
}

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct CreateLead {
    pub number: i32,
    pub localization: String,
    pub area: String,
    pub number_ud: i32,
    pub diagnostic_unity: String,
    pub substrate: String,
    pub exposed_coating: String,
    pub measure_localization: String,
    pub measure: i32,
    pub incertitude: i32,
    pub result: String,
}
