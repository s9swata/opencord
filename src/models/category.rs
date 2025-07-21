use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub server_id: uuid::Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub attributes: Option<serde_json::Value>,
}
