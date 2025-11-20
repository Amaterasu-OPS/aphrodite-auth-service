ALTER TABLE oauth_session DROP COLUMN IF EXISTS authorization_code;

CREATE TABLE IF NOT EXISTS oauth_token (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    status INT DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_oauth_token_session FOREIGN KEY (session_id) REFERENCES oauth_session(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_oauth_token_session_id ON oauth_token using hash(session_id);
CREATE INDEX IF NOT EXISTS idx_oauth_token_refresh_token ON oauth_token using hash(refresh_token);
CREATE INDEX IF NOT EXISTS idx_oauth_token_status ON oauth_token using hash(status);

ALTER TABLE oauth_client ADD CONSTRAINT uq_oauth_client_slug UNIQUE (slug);
ALTER TABLE oauth_session ADD CONSTRAINT fk_oauth_session_client_slug FOREIGN KEY (client_id) REFERENCES oauth_client(slug) ON DELETE CASCADE;