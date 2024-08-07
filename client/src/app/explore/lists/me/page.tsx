"use client";
import { getAllLists, getUsersLikes } from "@/app/actions";
import { Card as AppleCard, Carousel } from "@/components/animated/AppleCards";
import { TypewriterEffect } from "@/components/animated/TypeWriter";
import { ExploreListItem } from "@/components/ExploreListItem";
import { buttonVariants } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import ViewList from "@/components/ViewList";
import { cn } from "@/lib/utils";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@radix-ui/react-tabs";
import { useQuery } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { useContext, useEffect } from "react";
import { listType } from "../../../../../services/api.types";
import { useRecommendedLists } from "../../../../../services/queries";
import { UserContext } from "../../../context";

const ExplorePage = () => {
  const router = useRouter();
  const { user } = useContext(UserContext);

  useEffect(() => {
    if (!localStorage.getItem("token")) {
      router.push("/login");
    }
  }, []);

  const recommendedLists = useRecommendedLists();
  const listTypes = Object.keys(listType.Enum);
  const {
    data: usersLikes,
    error,
    isLoading,
  } = useQuery({
    queryKey: [`users-likes-${user?.username}`],
    queryFn: async () => {
      return await getUsersLikes(user?.username ?? "");
    },
  });

  if (error) {
    console.error("Error fetching user likes:", error);
  }

  if (!isLoading && !error && !usersLikes) {
    console.warn("User likes data is undefined");
  }

  const words = "Explore Lists Page".split(" ").map((word) => ({
    text: word,
    className: "text-blue-500 dark:text-blue-500",
  }));

  const { data: lists } = useQuery({
    queryKey: [`lists-${usersLikes?.response.length}`],
    queryFn: async () => {
      return await getAllLists();
    },
  });

  return (
    <div className="flex flex-1 flex-col justify-between items-center min-h-full min-w-full">
      <div className="self-start flex justify-center w-full flex-0 py-12 px-8 text-5xl">
        <TypewriterEffect
          className="w-full h-full p-6"
          words={words}
        ></TypewriterEffect>
      </div>
      <div className="flex flex-col w-full items-center gap-4 py-12">
        <h2 className="text-3xl font-semibold">Recommended Lists for You</h2>
      </div>
      <Carousel
        items={
          recommendedLists.data?.map((list, index) => (
            <AppleCard
              key={index}
              card={{
                category: list.list_type,
                title: list.listname,
                src: "/howls-1.jpeg",
                content: (
                  <ViewList
                    user_name={list.username}
                    list_name={list.listname}
                  />
                ),
              }}
              index={index}
            />
          )) ?? []
        }
      />
      <Card className="w-full shadow-md py-8 px-3 max-w-3xl lg:max-w-7xl flex self-center flex-col p-4 gap-8 mt-10">
        <CardHeader>
          <div className="w-full flex flex-col gap-y-4 items-center justify-center">
            <h1 className="text-5xl font-semibold text-blue-700">
              Explore Lists
            </h1>
          </div>
        </CardHeader>
        <CardContent>
          <Tabs
            defaultValue="anime"
            className="w-full flex flex-col items-center justify-center"
          >
            <TabsList className="flex max-w-xl gap-2 justify-evenly items-center pb-10">
              {listTypes.map((lt, index) => (
                <TabsTrigger
                  key={`${lt}-${index}_${new Date().getTime()}`}
                  value={lt}
                  className={cn(
                    buttonVariants({ variant: "outline" }),
                    "text-md md:text-2xl p-4 md:py-6 md:px-8"
                  )}
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
                        alreadyLiked={
                          usersLikes?.response
                            .map((l) => l.likingname)
                            .includes(list.username) ?? false
                        }
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
  );
};

export default ExplorePage;
