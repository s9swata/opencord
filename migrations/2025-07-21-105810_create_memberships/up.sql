-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "memberships" (
    "id" UUID PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "server_id" UUID NOT NULL REFERENCES "servers"("id") ON DELETE CASCADE,
    "joined_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "role_ids" UUID[] DEFAULT '{}',
    "nickname" VARCHAR(32),
    "attributes" JSONB DEFAULT '{}',
    UNIQUE ("user_id", "server_id")
);
