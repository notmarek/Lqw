CREATE TABLE public.users
(
    id SERIAL PRIMARY KEY NOT NULL,
    discord_id BIGINT NOT NULL,
    messages INTEGER DEFAULT 0 NOT NULL,
    money INTEGER DEFAULT 0  NOT NULL,
    daily_claimed BIGINT DEFAULT 0 NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.users
    OWNER to postgres;