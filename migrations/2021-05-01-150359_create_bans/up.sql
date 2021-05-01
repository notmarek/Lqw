CREATE TABLE public.bans
(
    "id" SERIAL PRIMARY KEY NOT NULL,
    "admin_user_id" INTEGER NOT NULL,
    "banned_user_id" INTEGER NOT NULL,
    "guild_id" BIGINT NOT NULL,
    "reason" VARCHAR NOT NULL,
    "ban_time" BIGINT NOT NULL,
    "end_time" BIGINT NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.bans
    OWNER to postgres;