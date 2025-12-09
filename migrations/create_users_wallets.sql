CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY,
    uid             UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    username        VARCHAR(64),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE wallets (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT REFERENCES users(id) ON DELETE CASCADE,
    encrypted_key   BYTEA NOT NULL,           -- AES-GCM 加密后的 64 字节 secret_key
    nonce           BYTEA NOT NULL,           -- 12 字节 nonce
    pubkey          VARCHAR(44) UNIQUE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);