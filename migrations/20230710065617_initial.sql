CREATE TABLE IF NOT EXISTS posts (
    url TEXT NOT NULL,
    title TEXT NOT NULL,
    created INTEGER NOT NULL -- unix time seconds UTC
)