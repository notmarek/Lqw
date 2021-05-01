CREATE TABLE public.shop
(
    "id" SERIAL PRIMARY KEY NOT NULL,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR NOT NULL,
    "price" INTEGER DEFAULT 0  NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE public.shop
    OWNER to postgres;