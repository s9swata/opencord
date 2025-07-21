-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users" (
    "id" UUID PRIMARY KEY,
    "username" VARCHAR(32) NOT NULL,
    "email" VARCHAR(64) UNIQUE NOT NULL,
    "avatar_url" VARCHAR(256),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "last_active" TIMESTAMPTZ,
    "attributes" JSONB DEFAULT '{}'
);
