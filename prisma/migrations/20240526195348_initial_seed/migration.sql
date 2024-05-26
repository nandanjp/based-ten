-- CreateEnum
CREATE TYPE "Genre" AS ENUM ('action', 'adventure', 'comedy', 'drama', 'scifi', 'space', 'shounen', 'police', 'mystery', 'supernatural', 'magic', 'josei', 'slice_of_life', 'sports', 'cars', 'seinen', 'horror', 'psychological', 'thriller', 'marital_arts', 'super_power', 'fantasy', 'school', 'vampire', 'historical', 'dementia', 'mecha', 'demons', 'romance', 'samurai', 'military', 'game', 'music', 'shoujo', 'harem');

-- CreateTable
CREATE TABLE "Anime" (
    "id" TEXT NOT NULL,
    "anime_id" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "score" DOUBLE PRECISION NOT NULL,
    "genres" "Genre"[],
    "english_name" TEXT NOT NULL,
    "japanese_name" TEXT NOT NULL,
    "synopsis" TEXT NOT NULL,
    "type" TEXT NOT NULL,
    "episodes" INTEGER NOT NULL,
    "aired" TEXT NOT NULL,
    "premiered" TEXT NOT NULL,
    "producers" TEXT NOT NULL,
    "licensors" TEXT[],
    "studios" TEXT NOT NULL,
    "source" TEXT NOT NULL,
    "duration" TEXT NOT NULL,
    "rating" TEXT NOT NULL,
    "ranked" INTEGER NOT NULL,
    "popularity" INTEGER NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Anime_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Manga" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "date_released" TEXT NOT NULL,
    "depth" INTEGER NOT NULL,
    "download_timeout" DOUBLE PRECISION NOT NULL,
    "download_slot" TEXT NOT NULL,
    "download_latency" DOUBLE PRECISION NOT NULL,
    "link" TEXT NOT NULL,
    "genres" "Genre"[],
    "status" TEXT NOT NULL,
    "rating" DOUBLE PRECISION NOT NULL,
    "img_link" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Manga_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Anime_anime_id_key" ON "Anime"("anime_id");

-- CreateIndex
CREATE UNIQUE INDEX "Anime_name_key" ON "Anime"("name");

-- CreateIndex
CREATE UNIQUE INDEX "Anime_english_name_key" ON "Anime"("english_name");

-- CreateIndex
CREATE UNIQUE INDEX "Anime_japanese_name_key" ON "Anime"("japanese_name");
