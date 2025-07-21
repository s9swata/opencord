use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::memberships)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Membership {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub server_id: uuid::Uuid,
    pub joined_at: Option<chrono::NaiveDateTime>,
    pub role_ids: Option<Vec<Option<uuid::Uuid>>>,
    pub nickname: Option<String>,
    pub attributes: Option<serde_json::Value>,
}
