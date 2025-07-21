-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "messages" (
    "id" BIGSERIAL PRIMARY KEY,
    "channel_id" BIGSERIAL NOT NULL REFERENCES "channels"("id") ON DELETE CASCADE,
    "user_id" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "edited_at" TIMESTAMPTZ DEFAULT NULL,
    "reference_id" BIGINT REFERENCES "messages"("id") ON DELETE SET NULL,
    "attachments" JSONB DEFAULT '[]',
    "reactions" JSONB DEFAULT '[]',
    "attributes" JSONB DEFAULT '{}'
);
