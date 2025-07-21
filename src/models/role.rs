use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: Option<String>,
    pub permissions: Option<i64>,
    pub attributes: Option<serde_json::Value>,
    pub server_id: uuid::Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
