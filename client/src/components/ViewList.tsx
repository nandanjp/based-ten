"use client";
import { getUserList, getUserListType } from "@/app/actions";
import { SparklesCore } from "@/components/animated/Sparkles";
import { LoadingSpinner } from "@/components/animated/Spinner";
import { TypewriterEffect } from "@/components/animated/TypeWriter";
import { ListView } from "@/components/ListView";
import { useQuery } from "@tanstack/react-query";
import Link from "next/link";
import {
  ListAnimeResponseType,
  ListGameResponseType,
  ListMovieResponseType,
  ListSongResponseType,
} from "../../services/api.types";
import { cn } from "@/lib/utils";
import { buttonVariants } from "./ui/button";

interface ViewListProps {
  list_name: string;
  user_name: string;
}

const ViewList = ({ list_name, user_name }: ViewListProps) => {
  const { data, isFetching, isError } = useQuery({
    queryKey: ["list-by-name"],
    queryFn: async () => {
      const type = await getUserListType(user_name, list_name);
      let likes: number | undefined = undefined;

      switch (type.response.listtype) {
        case "anime":
          likes = (
            await getUserList<ListAnimeResponseType>(user_name, list_name)
          ).response[0].likes;
        case "movies":
          likes = (
            await getUserList<ListMovieResponseType>(user_name, list_name)
          ).response[0].likes;
        case "songs":
          likes = (
            await getUserList<ListSongResponseType>(user_name, list_name)
          ).response[0].likes;
        case "videogames":
          likes = (
            await getUserList<ListGameResponseType>(user_name, list_name)
          ).response[0].likes;
      }
      return {
        type: type.response.listtype,
        likes,
      };
    },
  });

  if (isError || isFetching) {
    return (
      <div className="p-8 h-full flex justify-between">
        <LoadingSpinner className="text-blue-300" />
      </div>
    );
  }

  const { type, likes } = data!;

  const words = decodeURIComponent(list_name)
    .split(" ")
    .map((word) => ({
      text: word,
      className: "text-blue-500 dark:text-blue-500",
    }));
  return (
    <div className="p-8 h-full min-w-full flex flex-col gap-12">
      <div className="flex flex-col gap-4 w-full items-center justify-center">
        <div className="w-full absolute inset-0 h-full">
          <SparklesCore
            id="tsparticlesfullpage"
            background="transparent"
            minSize={0.6}
            maxSize={1.4}
            particleDensity={100}
            className="w-full h-full"
            particleColor="#FFFFFF"
          />
        </div>
        <TypewriterEffect className="text-xl" words={words}></TypewriterEffect>
        <Link
          href={`/user/${user_name}`}
          className={cn(buttonVariants({ variant: "secondary" }), "z-10")}
        >
          created by {user_name}
        </Link>
        <h3 className="font-bold italic text-sm pb-4">type: {type}</h3>
        <h3 className="font-bold italic text-sm pb-4">{likes} likes</h3>
        <div className="flex gap-2"></div>
      </div>
      <div className="flex flex-col gap-4">
        {isError || isFetching ? (
          <LoadingSpinner className="text-blue-300" />
        ) : (
          <ListView type={type} user_name={user_name} list_name={list_name} />
        )}
      </div>
    </div>
  );
};

export default ViewList;
