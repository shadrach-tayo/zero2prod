-- Add migration script here
ALTER TABLE subscription_tokens ADD COLUMN expired BOOLEAN NOT NULL DEFAULT false
