-- CreateEnum
CREATE TYPE "Platforms" AS ENUM ('NINTENDO_SWITCH', 'NINTENDO_WIIU', 'NINTENDO_WII', 'NINTENDO_GAMECUBE', 'NINTENDO_64', 'PS5', 'PS4', 'PS3', 'PS2', 'XBOX_X', 'XBOX_S', 'XBOX_ONE', 'XBOX');

-- CreateEnum
CREATE TYPE "ListType" AS ENUM ('Anime', 'Movies', 'Songs', 'VideoGames');

-- CreateTable
CREATE TABLE "User" (
    "user_id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "display_name" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "User_pkey" PRIMARY KEY ("user_id")
);

-- CreateTable
CREATE TABLE "Likes" (
    "user_id" TEXT NOT NULL,
    "list_name" TEXT NOT NULL,

    CONSTRAINT "Likes_pkey" PRIMARY KEY ("user_id","list_name")
);

-- CreateTable
CREATE TABLE "Follow" (
    "following_id" TEXT NOT NULL,
    "followed_id" TEXT NOT NULL,

    CONSTRAINT "Follow_pkey" PRIMARY KEY ("followed_id","following_id")
);

-- CreateTable
CREATE TABLE "MemberOf" (
    "user_id" TEXT NOT NULL,
    "group_id" TEXT NOT NULL,

    CONSTRAINT "MemberOf_pkey" PRIMARY KEY ("user_id","group_id")
);

-- CreateTable
CREATE TABLE "Group" (
    "group_id" TEXT NOT NULL,
    "group_name" TEXT NOT NULL,
    "owner_id" TEXT NOT NULL,

    CONSTRAINT "Group_pkey" PRIMARY KEY ("group_id")
);

-- CreateTable
CREATE TABLE "List" (
    "name" TEXT NOT NULL,
    "author_id" TEXT NOT NULL,
    "type" "ListType" NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "List_pkey" PRIMARY KEY ("author_id","name")
);

-- CreateTable
CREATE TABLE "MediaInList" (
    "media_id" TEXT NOT NULL,
    "list_name" TEXT NOT NULL,

    CONSTRAINT "MediaInList_pkey" PRIMARY KEY ("media_id","list_name")
);

-- CreateTable
CREATE TABLE "Media" (
    "id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "image" TEXT NOT NULL,
    "creators" TEXT[],
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Media_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Anime" (
    "anime_id" TEXT NOT NULL,
    "episodes" INTEGER NOT NULL,

    CONSTRAINT "Anime_pkey" PRIMARY KEY ("anime_id")
);

-- CreateTable
CREATE TABLE "Movies" (
    "movie_id" TEXT NOT NULL,

    CONSTRAINT "Movies_pkey" PRIMARY KEY ("movie_id")
);

-- CreateTable
CREATE TABLE "Songs" (
    "song_id" TEXT NOT NULL,
    "album" TEXT NOT NULL,

    CONSTRAINT "Songs_pkey" PRIMARY KEY ("song_id")
);

-- CreateTable
CREATE TABLE "VideoGames" (
    "video_game_id" SERIAL NOT NULL,
    "platforms" "Platforms"[],

    CONSTRAINT "VideoGames_pkey" PRIMARY KEY ("video_game_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "User_email_key" ON "User"("email");

-- CreateIndex
CREATE UNIQUE INDEX "Likes_user_id_list_name_key" ON "Likes"("user_id", "list_name");

-- CreateIndex
CREATE UNIQUE INDEX "Follow_followed_id_following_id_key" ON "Follow"("followed_id", "following_id");

-- CreateIndex
CREATE UNIQUE INDEX "MemberOf_user_id_group_id_key" ON "MemberOf"("user_id", "group_id");

-- CreateIndex
CREATE UNIQUE INDEX "List_name_key" ON "List"("name");

-- CreateIndex
CREATE UNIQUE INDEX "MediaInList_media_id_list_name_key" ON "MediaInList"("media_id", "list_name");

-- AddForeignKey
ALTER TABLE "Likes" ADD CONSTRAINT "Likes_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User"("user_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Likes" ADD CONSTRAINT "Likes_list_name_user_id_fkey" FOREIGN KEY ("list_name", "user_id") REFERENCES "List"("name", "author_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Follow" ADD CONSTRAINT "Follow_following_id_fkey" FOREIGN KEY ("following_id") REFERENCES "User"("user_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Follow" ADD CONSTRAINT "Follow_followed_id_fkey" FOREIGN KEY ("followed_id") REFERENCES "User"("user_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MemberOf" ADD CONSTRAINT "MemberOf_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User"("user_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MemberOf" ADD CONSTRAINT "MemberOf_group_id_fkey" FOREIGN KEY ("group_id") REFERENCES "Group"("group_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Group" ADD CONSTRAINT "Group_owner_id_fkey" FOREIGN KEY ("owner_id") REFERENCES "User"("user_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "List" ADD CONSTRAINT "List_author_id_fkey" FOREIGN KEY ("author_id") REFERENCES "User"("user_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MediaInList" ADD CONSTRAINT "MediaInList_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "Media"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MediaInList" ADD CONSTRAINT "MediaInList_list_name_fkey" FOREIGN KEY ("list_name") REFERENCES "List"("name") ON DELETE RESTRICT ON UPDATE CASCADE;
