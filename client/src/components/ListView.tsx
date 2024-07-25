import { getUserList } from "@/app/actions";
import { BentoGrid, BentoGridItem } from "@/components/animated/BentoGrid";
import { IconTableColumn } from "@tabler/icons-react";
import { useQuery } from "@tanstack/react-query";
import {
  ListAnimeResponseType,
  ListGameResponseType,
  ListMovieResponseType,
  ListSongResponseType,
  ListType,
} from "../../services/api.types";
import { LoadingSpinner } from "./animated/Spinner";
import { HeroHighlight } from "./animated/HeroHighlight";

interface ListViewProps {
  type: ListType;
  user_name: string;
  list_name: string;
}

export function ListView({ type, user_name, list_name }: ListViewProps) {
  return (
    <HeroHighlight className="min-w-full lg:flex min-h-screen items-center justify-center">
      <List list_name={list_name} user_name={user_name} type={type} />
    </HeroHighlight>
  );
}

function List({ type, user_name, list_name }: ListViewProps) {
  switch (type) {
    case "anime": {
      const { data, isFetching, isError } = useQuery({
        queryKey: ["anime-list"],
        queryFn: async () => {
          return await getUserList<ListAnimeResponseType>(user_name, list_name);
        },
      });
      return (
        <BentoGrid>
          {isFetching || isError ? (
            <div className="p-8 h-full flex justify-between">
              <LoadingSpinner className="text-blue-300" />
            </div>
          ) : (
            data?.response.map((item, i) => (
              <BentoGridItem
                key={i}
                title={item.listname}
                description={
                  <Skeleton
                    message={`The list ${item.listname} has ${item.likes} and has ${item.numepisodes} episodes`}
                  />
                }
                header={item.rankinginlist}
                icon={<IconTableColumn className="h-4 w-4 text-neutral-500" />}
                className={
                  i === 3 || i === 6
                    ? "md:col-span-2"
                    : i === 10
                      ? "md:col-span-3"
                      : ""
                }
                style={{ width: `${100 - (i < 3 ? i : 3) * 5}%` }}
              />
            ))
          )}
        </BentoGrid>
      );
    }
    case "movies": {
      const { data, isFetching, isError } = useQuery({
        queryKey: ["anime-list"],
        queryFn: async () => {
          return await getUserList<ListMovieResponseType>(user_name, list_name);
        },
      });
      return (
        <BentoGrid className="max-w-4xl mx-auto">
          {isFetching || isError ? (
            <div className="p-8 h-full flex justify-between">
              <LoadingSpinner className="text-blue-300" />
            </div>
          ) : (
            data?.response.map((item, i) => (
              <BentoGridItem
                key={i}
                title={item.listname}
                description={
                  <Skeleton
                    message={`The list ${item.listname} has ${item.likes}`}
                  />
                }
                header={item.rankinginlist}
                icon={<IconTableColumn className="h-4 w-4 text-neutral-500" />}
                className={i === 3 || i === 6 || i == 9 ? "md:col-span-2" : ""}
                style={{ width: `${100 - (i < 3 ? i : 3) * 5}%` }}
              />
            ))
          )}
        </BentoGrid>
      );
    }
    case "songs": {
      const { data, isFetching, isError } = useQuery({
        queryKey: ["anime-list"],
        queryFn: async () => {
          return await getUserList<ListSongResponseType>(user_name, list_name);
        },
      });
      return (
        <BentoGrid className="max-w-4xl mx-auto">
          {isFetching || isError ? (
            <div className="p-8 h-full flex justify-between">
              <LoadingSpinner className="text-blue-300" />
            </div>
          ) : (
            data?.response.map((item, i) => (
              <BentoGridItem
                key={i}
                title={item.listname}
                description={
                  <Skeleton
                    message={`Album: ${item.album}, Author: ${item.author}`}
                  />
                }
                header={item.rankinginlist}
                icon={<IconTableColumn className="h-4 w-4 text-neutral-500" />}
                className={i === 3 || i === 6 || i == 9 ? "md:col-span-2" : ""}
                style={{ width: `${100 - (i < 3 ? i : 3) * 5}%` }}
              />
            ))
          )}
        </BentoGrid>
      );
    }
    case "videogames":
      const { data, isFetching, isError } = useQuery({
        queryKey: ["anime-list"],
        queryFn: async () => {
          return await getUserList<ListGameResponseType>(user_name, list_name);
        },
      });
      return (
        <BentoGrid className="max-w-4xl mx-auto">
          {isFetching || isError ? (
            <div className="p-8 h-full flex justify-between">
              <LoadingSpinner className="text-blue-300" />
            </div>
          ) : (
            data?.response.map((item, i) => (
              <BentoGridItem
                key={i}
                title={item.listname}
                description={
                  <Skeleton message={`Playable on: ${item.console}`} />
                }
                header={item.rankinginlist}
                icon={<IconTableColumn className="h-4 w-4 text-neutral-500" />}
                className={i === 3 || i === 6 || i == 9 ? "md:col-span-2" : ""}
                style={{ width: `${100 - (i < 3 ? i : 3) * 5}%` }}
              />
            ))
          )}
        </BentoGrid>
      );
  }
}

const Skeleton = ({ message }: { message: string }) => (
  <div className="flex flex-1 w-full h-full min-h-[6rem] rounded-lg p-4">
    {message}
  </div>
);
