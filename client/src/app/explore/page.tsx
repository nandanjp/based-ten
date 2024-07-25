"use client";
import { Spotlight } from "@/components/animated/SpotLight";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { Badge, ChevronsUpDown, Heart } from "lucide-react";
import Link from "next/link";
import { useContext } from "react";
import { listType } from "../../../services/api.types";
import { useAllLists, useRecommendedLists } from "../../../services/queries";
import { createLike } from "../actions";
import { UserContext } from "../context";
import { HeroHighlight } from "@/components/animated/HeroHighlight";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import Image from "next/image";
import { TypewriterEffect } from "@/components/animated/TypeWriter";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@radix-ui/react-tabs";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { ExploreListItem } from "@/components/ExploreListItem";
import CardWrapper from "@/components/CardWrapper";
import { IconClick } from "@tabler/icons-react";

const ExplorePage = () => {
  const { data: lists, isError, isFetching } = useAllLists();
  const { user } = useContext(UserContext);
  const recommendedLists = useRecommendedLists();
  const listTypes = Object.keys(listType.Enum);
  const onLikeClick = async (
    list_name: string,
    user_name: string,
    event: React.MouseEvent<SVGSVGElement, MouseEvent>
  ) => {
    event.currentTarget.setAttribute(
      "fill",
      event.currentTarget.getAttribute("fill") === "pink" ? "none" : "pink"
    );
    if (user) {
      const response = await createLike({
        list_name,
        liking_name: user_name,
      });
      if (response?.error) {
        const message = `An error has occurred: ${response.error}`;
        throw new Error(message);
      }
      console.log("like response");
      console.log(response);
    } else {
      console.log("no user??");
    }
  };

  const words = "Explore Page".split(" ").map((word) => ({
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
        {user ? (
          <div className="flex flex-col w-full items-center gap-4 py-16">
            <h2 className="text-lg font-semibold">Recommended Lists for You</h2>
            <Carousel className="w-1/2">
              <CarouselContent className="-ml-1">
                {recommendedLists.data?.map((list, index) => (
                  <CarouselItem
                    key={index}
                    className="lg:basis-1/4 md:basis-1/3 sm:basis-1/2"
                  >
                    <Card>
                      <CardContent className="flex flex-col gap-2 justify-between aspect-square p-6">
                        <div className="flex flex-col">
                          <span>{list.listname}</span>
                          <span className="italic text-xs">
                            {list.username}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <Badge className="w-fit">
                            {list && list.list_type
                              ? list.list_type.toUpperCase()
                              : ""}
                          </Badge>
                        </div>
                      </CardContent>
                    </Card>
                  </CarouselItem>
                ))}
              </CarouselContent>
              <CarouselPrevious />
              <CarouselNext />
            </Carousel>
          </div>
        ) : (
          <div className="flex flex-col w-full items-center gap-4 py-16">
            <h2 className="text-lg font-semibold">Recommended Lists for You</h2>
            <Carousel className="w-1/2">
              <CarouselContent className="-ml-1">
                <CarouselItem className="lg:basis-1/4 md:basis-1/3 sm:basis-1/2">
                  <Card>
                    <CardContent className="flex flex-col gap-2 justify-between aspect-square p-6">
                      <div className="flex flex-col">
                        <span>Looks like you've got none yet!</span>
                      </div>
                      <div className="flex justify-between">
                        <IconClick>Explore a List!</IconClick>
                      </div>
                    </CardContent>
                  </Card>
                </CarouselItem>
              </CarouselContent>
              <CarouselPrevious />
              <CarouselNext />
            </Carousel>
          </div>
        )}

        <Card className="min-w-full shadow-md py-8 px-3 max-w-7xl">
          <CardHeader>
            <div className="w-full flex flex-col gap-y-4 items-center justify-center">
              <h1 className="text-5xl font-semibold text-blue-200">
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
