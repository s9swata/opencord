use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::direct_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DirectMessage {
    pub id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub receiver_id: uuid::Uuid,
    pub content: String,
    pub attachments: Option<serde_json::Value>,
    pub reactions: Option<serde_json::Value>,
    pub edited_at: Option<chrono::NaiveDateTime>,
    pub reference_id: Option<uuid::Uuid>,
    pub attributes: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
