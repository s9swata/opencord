-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "channels" (
    "id" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "server_id" UUID NOT NULL REFERENCES "servers"("id") ON DELETE CASCADE,
    "category_id" UUID REFERENCES "categories"("id") ON DELETE SET NULL,
    "type" VARCHAR(12) NOT NULL CHECK (type IN ('text', 'voice', 'announcement')),
    "order_in_ui" INT NOT NULL DEFAULT 0,
    "attributes" JSONB DEFAULT '{}',
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
