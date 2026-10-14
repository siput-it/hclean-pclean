CREATE TABLE IF NOT EXISTS user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    access_token_hash VARCHAR(255) NOT NULL,
    refresh_token_hash VARCHAR(255) NOT NULL,
    session_status VARCHAR(16) NOT NULL DEFAULT 'active' CHECK (session_status IN ('active', 'revoked')),
    expires_at TIMESTAMPTZ NOT NULL,
    refresh_expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_status ON user_sessions(session_status, expires_at);
CREATE UNIQUE INDEX IF NOT EXISTS uq_user_sessions_refresh_hash ON user_sessions(refresh_token_hash);
CREATE UNIQUE INDEX IF NOT EXISTS uq_user_sessions_access_hash ON user_sessions(access_token_hash);
