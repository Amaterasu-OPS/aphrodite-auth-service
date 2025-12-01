CREATE TABLE IF NOT EXISTS oauth_consent (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    client_id TEXT NOT NULL REFERENCES oauth_client(slug) ON DELETE CASCADE,
    scopes _TEXT NOT NULL,
    status INT DEFAULT 1,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_oauth_consent_client_id ON oauth_consent using hash (client_id);
CREATE INDEX IF NOT EXISTS idx_oauth_consent_user_id ON oauth_consent using hash (user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_consent_status ON oauth_consent using hash (status);