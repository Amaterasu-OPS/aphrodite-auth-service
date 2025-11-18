CREATE TABLE IF NOT EXISTS oauth_session
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id TEXT,
    user_id UUID NULL,
    scopes _TEXT NULL,
    redirect_uri TEXT,
    state TEXT,
    response_type TEXT,
    authorization_code TEXT NULL,
    code_challenge TEXT,
    code_challenge_method TEXT,
    status INT DEFAULT 1,
    consent_granted_at TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_oauth_session_user_id ON oauth_session using hash(user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_session_client_id ON oauth_session using hash(client_id);
CREATE INDEX IF NOT EXISTS idx_oauth_session_auth_code ON oauth_session using hash(authorization_code);
CREATE INDEX IF NOT EXISTS idx_oauth_session_status ON oauth_session using hash(status);

CREATE INDEX IF NOT EXISTS idx_oauth_client_status ON oauth_client using hash(status);