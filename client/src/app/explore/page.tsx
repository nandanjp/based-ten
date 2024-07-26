"use client";
import { TypewriterEffect } from "@/components/animated/TypeWriter";
import { ExploreListItem } from "@/components/ExploreListItem";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@radix-ui/react-tabs";
import { useRouter } from "next/navigation";
import { useContext, useEffect } from "react";
import { listType } from "../../../services/api.types";
import { useAllLists } from "../../../services/queries";
import { UserContext } from "../context";

const ExplorePage = () => {
  const { data: lists, isError, isFetching } = useAllLists();
  const listTypes = Object.keys(listType.Enum);
  const router = useRouter();
  const { user } = useContext(UserContext);

  useEffect(() => {
    if (user) router.push("/explore/me");
  }, []);

  const words = "Explore Lists Page".split(" ").map((word) => ({
    text: word,
    className: "text-blue-500 dark:text-blue-500",
  }));
  return (
    <div className="flex flex-1 flex-col justify-between items-center min-h-full min-w-full">
      <div className="self-start flex justify-center w-full flex-0 py-12 px-8 text-5xl">
        <TypewriterEffect
          className="text-5xl w-full h-full p-6 sm:text-10xl md:text-12xl"
          words={words}
        ></TypewriterEffect>
      </div>
      <div className="flex self-center flex-col p-4 gap-8 min-w-full flex-1 mt-10">
        <Card className="w-full shadow-md py-8 px-3 max-w-3xl lg:max-w-7xl flex self-center flex-col p-4 gap-8 mt-10">
          <CardHeader>
            <div className="w-full flex flex-col gap-y-4 items-center justify-center">
              <h1 className="text-5xl font-semibold text-blue-400">
                Explore Lists
              </h1>
            </div>
          </CardHeader>
          <CardContent>
            <Tabs defaultValue="anime" className="w-full">
              <TabsList className="flex w-full justify-evenly items-center pb-10">
                {listTypes.map((lt, index) => (
                  <TabsTrigger
                    key={`${lt}-${index}_${new Date().getTime()}`}
                    value={lt}
                    className="text-xl md:text-2xl"
                  >
                    {lt}
                  </TabsTrigger>
                ))}
              </TabsList>
              {listTypes.map((lt, index) => (
                <TabsContent
                  value={lt}
                  key={`${lt}-${index}-${new Date().getTime()}`}
                  className="w-full grid md:grid-cols-2 xl:grid-cols-3 gap-4 lg:gap-5 justify-center"
                >
                  {lists
                    ?.filter((list) => list.list_type === lt)
                    .map((list, index) => (
                      <div
                        className="flex justify-center items-center"
                        key={index}
                      >
                        <ExploreListItem
                          alreadyLiked={false}
                          type={list.list_type}
                          author={list.username}
                          title={list.listname}
                          numLikes={list.likes ?? 0}
                        />
                      </div>
                    ))}
                </TabsContent>
              ))}
            </Tabs>
          </CardContent>
          <CardFooter></CardFooter>
        </Card>
      </div>
    </div>
  );
};

export default ExplorePage;
