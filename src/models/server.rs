use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: uuid::Uuid,
    pub icon_url: Option<String>,
    pub region: String,
    pub attributes: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
