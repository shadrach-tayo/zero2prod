-- Add migration script here
ALTER TABLE newsletter_issues
RENAME COLUMN published_content TO published_at;