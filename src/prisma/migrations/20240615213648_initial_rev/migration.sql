/*
  Warnings:

  - The primary key for the `VideoGames` table will be changed. If it partially fails, the table could be left without primary key constraint.

*/
-- AlterTable
ALTER TABLE "VideoGames" DROP CONSTRAINT "VideoGames_pkey",
ALTER COLUMN "video_game_id" DROP DEFAULT,
ALTER COLUMN "video_game_id" SET DATA TYPE TEXT,
ADD CONSTRAINT "VideoGames_pkey" PRIMARY KEY ("video_game_id");
DROP SEQUENCE "VideoGames_video_game_id_seq";
