use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i64,
    pub channel_id: i64,
    pub user_id: uuid::Uuid,
    pub content: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub edited_at: Option<chrono::NaiveDateTime>,
    pub reference_id: Option<i64>,
    pub attachments: Option<serde_json::Value>,
    pub reactions: Option<serde_json::Value>,
    pub attributes: Option<serde_json::Value>,
}
