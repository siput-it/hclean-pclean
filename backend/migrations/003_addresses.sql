CREATE TABLE IF NOT EXISTS addresses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    label VARCHAR(80) NOT NULL DEFAULT 'Rumah',
    recipient_name VARCHAR(120) NOT NULL,
    recipient_phone VARCHAR(30) NOT NULL,
    full_address TEXT NOT NULL,
    additional_details TEXT,
    latitude NUMERIC(9,6),
    longitude NUMERIC(9,6),
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_latitude CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    CONSTRAINT chk_longitude CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180)
);

CREATE INDEX IF NOT EXISTS idx_addresses_user_id ON addresses(user_id);
CREATE INDEX IF NOT EXISTS idx_addresses_primary ON addresses(user_id, is_primary);
CREATE UNIQUE INDEX IF NOT EXISTS uq_addresses_primary_per_user
    ON addresses(user_id)
    WHERE is_primary = TRUE;
