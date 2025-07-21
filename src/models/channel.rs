use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub server_id: uuid::Uuid,
    pub category_id: Option<uuid::Uuid>,
    pub type_: String,
    pub order_in_ui: i32,
    pub attributes: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
