"use client";
import { getAllGroupsAndMembers, getUserGroups } from "@/app/actions";
import { UserContext } from "@/app/context";
import { TypewriterEffect } from "@/components/animated/TypeWriter";
import { ExploreGroupItem } from "@/components/ExploreGroupItem";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@radix-ui/react-tabs";
import { useQuery } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { useContext, useEffect } from "react";

const ExplorePage = () => {
  const router = useRouter();
  const { user } = useContext(UserContext);

  useEffect(() => {
    if (!localStorage.getItem("token")) {
      router.push("/login");
    }
  }, []);

  const {
    data: userGroups,
    isError: err,
    isFetching: fetching,
  } = useQuery({
    queryKey: [`user-groups-${user?.username}`],
    queryFn: async () => {
      return await getUserGroups(user?.username ?? "");
    },
  });

  const words = "Explore Groups Page".split(" ").map((word) => ({
    text: word,
    className: "text-blue-500 dark:text-blue-500",
  }));

  const {
    data: groups,
    isError,
    isFetching,
  } = useQuery({
    queryKey: [`all-groups-members-${userGroups?.response.length}`],
    queryFn: async () => {
      return await getAllGroupsAndMembers();
    },
  });

  return (
    <div className="flex flex-1 flex-col justify-between items-center min-h-full min-w-full">
      <div className="self-start flex justify-center w-full flex-0 py-12 px-8 text-5xl">
        <TypewriterEffect
          className="text-7xl w-full h-full p-6 sm:text-12xl md:text-14xl"
          words={words}
        ></TypewriterEffect>
      </div>
      <div className="flex self-center flex-col p-4 gap-8 min-w-full flex-1 mt-10">
        <Card className="min-w-full shadow-md py-8 px-3 max-w-7xl">
          <CardHeader>
            <div className="w-full flex flex-col gap-y-4 items-center justify-center">
              <h1 className="text-5xl font-semibold text-blue-400 text-center">
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
                      alreadyFollows={
                        userGroups?.response
                          .map((g) => g.gid)
                          .includes(group.gid) ?? false
                      }
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
        </Card>
      </div>
    </div>
  );
};

export default ExplorePage;
