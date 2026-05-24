CREATE TABLE IF NOT EXISTS "users" (
	"id" bigserial NOT NULL UNIQUE,
	"name" varchar(255) NOT NULL,
	"email" varchar(255) NOT NULL,
	"login" varchar(255) NOT NULL UNIQUE,
	"password" varchar(255) NOT NULL,
	"cell_number" varchar(255),
	"is_admin" boolean NOT NULL DEFAULT FALSE,
	"is_activated" boolean NOT NULL DEFAULT TRUE,
	"created_at" timestamp with time zone NOT NULL DEFAULT NOW(),
	"updated_at" timestamp with time zone NOT NULL DEFAULT NOW(),
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "books" (
	"id" bigserial NOT NULL UNIQUE,
	"title" varchar(255) NOT NULL,
	"author" varchar(255) NOT NULL,
	"description" varchar(255),
	"published_at" varchar(255),
	"isbn_10_code" varchar(255) UNIQUE,
	"isbn_13_code" varchar(255) UNIQUE,
	"cover_type" varchar(255),
	"edition" varchar(255)
	"language" varchar(255),
	"genre" varchar(255),
	"publisher" varchar(255),
	"pages" bigint,
	"dimensions" varchar(255),
	"created_at" timestamp with time zone NOT NULL DEFAULT NOW(),
	"updated_at" timestamp with time zone NOT NULL DEFAULT NOW(),
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "stores" (
	"id" bigserial NOT NULL UNIQUE,
	"name" varchar(255) NOT NULL,
	"cnpj" varchar(255) NOT NULL,
	"street" varchar(255) NOT NULL,
	"number" bigint NOT NULL,
	"city" varchar(255) NOT NULL,
	"state" varchar(255) NOT NULL,
	"city_block" varchar(255) NOT NULL,
	"cep" varchar(255) NOT NULL,
	"created_at" timestamp with time zone NOT NULL DEFAULT NOW(),
	"updated_at" timestamp with time zone NOT NULL DEFAULT NOW(),
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "user_store" (
	"role" varchar(255) NOT NULL,
	"user_id" bigint NOT NULL,
	"store_id" bigint NOT NULL,
	PRIMARY KEY ("user_id", "store_id")
);

CREATE TABLE IF NOT EXISTS "catalog" (
	"store_id" bigserial NOT NULL,
	"book_id" bigserial NOT NULL,
	"price" real NOT NULL,
	"quantity" bigint NOT NULL,
	"description" varchar(255) NOT NULL,
	PRIMARY KEY ("store_id", "book_id")
);

CREATE
OR REPLACE FUNCTION update_updated_at_column() RETURNS TRIGGER AS $ $ BEGIN NEW.updated_at = NOW();

RETURN NEW;

END;

$ $ language 'plpgsql';

DO $ $ DECLARE t text;

BEGIN FOR t IN
SELECT
	unnest(ARRAY ['users', 'books', 'stores']) LOOP EXECUTE format(
		'CREATE TRIGGER update_%I_updated_at
             BEFORE UPDATE ON %I
             FOR EACH ROW
             EXECUTE FUNCTION update_updated_at_column();',
		t,
		t
	);

END LOOP;

END;

$ $;

ALTER TABLE
	"user_store"
ADD
	CONSTRAINT "users_fk0" FOREIGN KEY ("user_id") REFERENCES "users"("id");

ALTER TABLE
	"user_store"
ADD
	CONSTRAINT "store_fk0" FOREIGN KEY ("store_id") REFERENCES "stores"("id");

ALTER TABLE
	"catalog"
ADD
	CONSTRAINT "catalog_fk0" FOREIGN KEY ("store_id") REFERENCES "stores"("id");

ALTER TABLE
	"catalog"
ADD
	CONSTRAINT "catalog_fk1" FOREIGN KEY ("book_id") REFERENCES "books"("id");