-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "categories" (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "description" TEXT,
    "server_id" UUID NOT NULL REFERENCES "servers"("id") ON DELETE CASCADE,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "attributes" JSONB DEFAULT '{}'
);
