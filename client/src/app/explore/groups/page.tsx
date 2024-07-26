"use client";
import { TypewriterEffect } from "@/components/animated/TypeWriter";
import { ExploreGroupItem } from "@/components/ExploreGroupItem";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@radix-ui/react-tabs";
import { useAllGroupsMembers } from "../../../../services/queries";
import { UserContext } from "@/app/context";
import { useContext } from "react";
import { useRouter } from "next/navigation";

const ExplorePage = () => {
  const router = useRouter();
  const { data: groups, isError, isFetching } = useAllGroupsMembers();
  const { user } = useContext(UserContext);
  if (user) {
    router.push("/explore/groups/me");
  }

  const words = "Explore Groups Page".split(" ").map((word) => ({
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
      <div className="flex self-center flex-col p-4 gap-8 max-w-7xl flex-1 mt-10">
        <Card className="shadow-md py-8 px-3">
          <CardHeader>
            <div className="w-full flex flex-col gap-y-4 items-center justify-center">
              <h1 className="text-5xl font-semibold text-blue-400">
                Explore Groups
              </h1>
            </div>
          </CardHeader>
          <CardContent>
            <Tabs defaultValue="Groups" className="w-full">
              <TabsList className="flex w-full justify-evenly items-center pb-10">
                <TabsTrigger value={"Groups"} className="text-xl md:text-2xl">
                  Groups
                </TabsTrigger>
              </TabsList>
              <TabsContent
                value={"Groups"}
                className="w-full grid md:grid-cols-2 xl:grid-cols-3 gap-4 lg:gap-5 justify-center"
              >
                {groups?.response.map((group, index) => (
                  <div className="flex justify-center items-center" key={index}>
                    <ExploreGroupItem
                      alreadyFollows={false}
                      gid={group.gid}
                      groupname={group.groupname}
                      numMembers={group.nummembers ?? 0}
                      owner={group.ownedby}
                    />
                  </div>
                ))}
              </TabsContent>
            </Tabs>
          </CardContent>
          <CardFooter></CardFooter>
        </Card>
      </div>
    </div>
  );
};

export default ExplorePage;
