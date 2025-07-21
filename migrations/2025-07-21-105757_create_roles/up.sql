-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "roles" (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "color" VARCHAR(7) DEFAULT '#000000',
    "permissions" BIGINT DEFAULT 0,
    "attributes" JSONB DEFAULT '{}',
    "server_id" UUID NOT NULL REFERENCES "servers"("id") ON DELETE CASCADE,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
