-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "direct_messages" (
    "id" UUID PRIMARY KEY,
    "sender_id" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "receiver_id" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "content" TEXT NOT NULL,
    "attachments" JSONB DEFAULT '[]',
    "reactions" JSONB DEFAULT '[]',
    "edited_at" TIMESTAMPTZ DEFAULT NULL,
    "reference_id" UUID REFERENCES "direct_messages"("id") ON DELETE SET NULL,
    "attributes" JSONB DEFAULT '{}',
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
