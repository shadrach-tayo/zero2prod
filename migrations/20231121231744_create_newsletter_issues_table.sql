CREATE TABLE newsletter_issues (
    newsletter_issue_id uuid NOT NULL,
    title TEXT NOT NULL,
    text_content TEXT NOT NULL,
    HTML_content TEXT NOT NULL,
    published_content TEXT NOT NULL,
    PRIMARY KEY(newsletter_issue_id)
);
