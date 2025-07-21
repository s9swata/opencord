-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "servers" (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL,
    "description" TEXT,
    "owner_id" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "icon_url" VARCHAR(256),
    "region" VARCHAR(10) NOT NULL,
    "attributes" JSONB DEFAULT '{}',
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
