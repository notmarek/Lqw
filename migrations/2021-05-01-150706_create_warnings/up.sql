CREATE TABLE public.warnings
(
    "id" SERIAL PRIMARY KEY NOT NULL,
    "admin_user_id" INTEGER NOT NULL,
    "warned_user_id" INTEGER NOT NULL,
    "guild_id" BIGINT NOT NULL,
    "reason" VARCHAR NOT NULL,
    "warning_time" BIGINT NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.warnings
    OWNER to postgres;