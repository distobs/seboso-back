CREATE TABLE IF NOT EXISTS "users" (
	"id" serial NOT NULL UNIQUE,
	"name" varchar(255) NOT NULL,
	"email" varchar(255) NOT NULL,
	"login" varchar(255) NOT NULL,
	"password" varchar(255) NOT NULL,
	"cell_number" varchar(255) NOT NULL,
	"role" bigint NOT NULL,
	"created_at" timestamp with time zone NOT NULL,
	"updated_at" timestamp with time zone NOT NULL,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "books" (
	"id" serial NOT NULL UNIQUE,
	"title" varchar(255) NOT NULL,
	"description" varchar(255) NOT NULL,
	"launched_at" timestamp with time zone NOT NULL,
	"cover_type" varchar(255) NOT NULL,
	"author" varchar(255) NOT NULL,
	"edition" varchar(255) NOT NULL,
	"language" varchar(255) NOT NULL,
	"genre" varchar(255) NOT NULL,
	"isbn_10_code" bigint NOT NULL,
	"isbn_13_code" varchar(255),
	"publisher" varchar(255) NOT NULL,
	"pages" bigint NOT NULL,
	"dimentions" varchar(255) NOT NULL,
	"created_at" timestamp with time zone NOT NULL,
	"updated_at" timestamp with time zone NOT NULL,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "store" (
	"id" serial NOT NULL UNIQUE,
	"name" varchar(255) NOT NULL,
	"cnpj" varchar(255) NOT NULL,
	"street" varchar(255) NOT NULL,
	"number" bigint NOT NULL,
	"city" varchar(255) NOT NULL,
	"estate" varchar(255) NOT NULL,
	"city_block" varchar(255) NOT NULL,
	"cep" varchar(255) NOT NULL,
	"created_at" timestamp with time zone NOT NULL,
	"updated_at" timestamp with time zone NOT NULL,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "user_store" (
	"role" bigint NOT NULL,
	"id_user" bigint NOT NULL,
	"id_store" bigint NOT NULL,
	PRIMARY KEY ("id_user", "id_store")
);

CREATE TABLE IF NOT EXISTS "catalog" (
	"id_store" bigint NOT NULL,
	"id_book" bigint NOT NULL,
	"price" bigint NOT NULL,
	"quantity" bigint NOT NULL,
	"description" bigint NOT NULL,
	PRIMARY KEY ("id_store", "id_book")
);


ALTER TABLE "user_store" ADD CONSTRAINT "users_fk0" FOREIGN KEY ("id_user") REFERENCES "users"("id");

ALTER TABLE "user_store" ADD CONSTRAINT "store_fk0" FOREIGN KEY ("id_store") REFERENCES "store"("id");

ALTER TABLE "catalog" ADD CONSTRAINT "catalog_fk0" FOREIGN KEY ("id_store") REFERENCES "store"("id");

ALTER TABLE "catalog" ADD CONSTRAINT "catalog_fk1" FOREIGN KEY ("id_book") REFERENCES "books"("id");