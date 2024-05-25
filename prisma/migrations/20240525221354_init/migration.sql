-- CreateTable
CREATE TABLE "VideoGame" (
    "id" INTEGER NOT NULL,
    "slug" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "metacritic" DOUBLE PRECISION NOT NULL,
    "released" TIMESTAMP(3) NOT NULL,
    "tba" BOOLEAN NOT NULL,
    "updated" TIMESTAMP(3) NOT NULL,
    "website" TEXT NOT NULL,
    "rating" DOUBLE PRECISION NOT NULL,
    "rating_top" DOUBLE PRECISION NOT NULL,

    CONSTRAINT "VideoGame_pkey" PRIMARY KEY ("id")
);
