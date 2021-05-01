CREATE TABLE public.inventory
(
    "id" SERIAL PRIMARY KEY NOT NULL,
    "user_id" INTEGER NOT NULL,
    "item_id" INTEGER NOT NULL,
    "amount" INTEGER DEFAULT 1 NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.inventory
    OWNER to postgres;