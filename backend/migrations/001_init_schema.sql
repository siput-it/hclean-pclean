CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(160) UNIQUE,
    phone VARCHAR(30) UNIQUE,
    password_hash VARCHAR(255),
    role VARCHAR(24) NOT NULL CHECK (role IN ('customer', 'worker', 'admin')),
    display_name VARCHAR(120) NOT NULL DEFAULT '',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS worker_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    bio TEXT,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS service_categories (
    id UUID PRIMARY KEY,
    name VARCHAR(160) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    base_price_idr BIGINT NOT NULL CHECK (base_price_idr >= 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES users(id),
    worker_id UUID REFERENCES users(id),
    service_category_id UUID NOT NULL REFERENCES service_categories(id),
    service_price_idr BIGINT NOT NULL CHECK (service_price_idr >= 0),
    customer_fee_idr BIGINT NOT NULL CHECK (customer_fee_idr >= 0),
    worker_fee_idr BIGINT NOT NULL CHECK (worker_fee_idr >= 0),
    status VARCHAR(24) NOT NULL CHECK (status IN ('pending', 'accepted', 'in_progress', 'completed', 'cancelled')),
    address TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS wallet_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    balance_idr BIGINT NOT NULL DEFAULT 0 CHECK (balance_idr >= 0),
    wallet_type VARCHAR(24) NOT NULL DEFAULT 'simulated' CHECK (wallet_type = 'simulated'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS wallet_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_account_id UUID NOT NULL REFERENCES wallet_accounts(id) ON DELETE CASCADE,
    order_id UUID REFERENCES orders(id),
    transaction_type VARCHAR(24) NOT NULL,
    amount_idr BIGINT NOT NULL CHECK (amount_idr >= 0),
    status VARCHAR(24) NOT NULL DEFAULT 'pending',
    idempotency_key VARCHAR(64) UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_phone ON users(phone);
CREATE INDEX IF NOT EXISTS idx_orders_customer_id ON orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_wallet_transactions_account_id ON wallet_transactions(wallet_account_id);

INSERT INTO service_categories (id, name, description, base_price_idr, is_active)
VALUES
    ('11111111-1111-1111-1111-111111111111', 'Membersihkan toilet', 'Pembersihan toilet dan area sanitasi', 50000, TRUE),
    ('22222222-2222-2222-2222-222222222222', 'Membersihkan kasur', 'Pembersihan kasur dan area tidur', 75000, TRUE),
    ('33333333-3333-3333-3333-333333333333', 'Mencuci pakaian', 'Layanan cuci pakaian', 30000, TRUE),
    ('44444444-4444-4444-4444-444444444444', 'Menyetrika', 'Layanan setrika pakaian', 25000, TRUE),
    ('55555555-5555-5555-5555-555555555555', 'Cuci dan setrika', 'Layanan cuci dan setrika gabungan', 45000, TRUE),
    ('66666666-6666-6666-6666-666666666666', 'Berkebun', 'Perawatan taman dan tanaman', 80000, TRUE),
    ('77777777-7777-7777-7777-777777777777', 'Maintenance/perbaikan rumah', 'Layanan maintenance atau perbaikan rumah', 100000, TRUE)
ON CONFLICT (name) DO NOTHING;
