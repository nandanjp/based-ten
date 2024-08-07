"use client";
import { Card, CardTitle } from "@/components/animated/GoodCard";
import { HeroHighlight } from "@/components/animated/HeroHighlight";
import { LoadingSpinner } from "@/components/animated/Spinner";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";
import { CardContent, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useDebounce } from "@/hooks/useDebounce";
import { useQuery } from "@tanstack/react-query";
import {
  AudioLines,
  Clapperboard,
  LucideGamepad,
  SearchIcon,
  Tv,
} from "lucide-react";
import { useRouter } from "next/navigation";
import {
  ChangeEvent,
  ReactElement,
  useContext,
  useEffect,
  useState,
} from "react";
import { MediaType } from "../../services/api.types";
import { getCurrentUser, getMedia } from "./actions";
import { UserContext } from "./context";

const SearchPage = () => {
  const router = useRouter();
  const { user, setUser } = useContext(UserContext);
  useEffect(() => {
    (async () => {
      if (localStorage.getItem("token") && !user) {
        const res = await getCurrentUser();
        if (res.success) setUser(res.response);
      }
    })();
  }, []);

  const [open, setOpen] = useState<boolean>(false);
  const [title, setTitle] = useState<string>("");
  const debouncedSearch = useDebounce(title);
  const { data, isError, isFetching } = useQuery({
    queryKey: [`media?title=${debouncedSearch}`],
    queryFn: async () => {
      return await getMedia({ page: 0, limit: 100, title: debouncedSearch });
    },
  });

  const handleValueChange = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setTitle(e.target.value);
    setOpen(!!e.target.value);
  };
  const onItemSelect = (item: MediaType) => {
    return () => {
      router.push(`/create-list/${item.type}?itemId=${item.id}`);
    };
  };

  return (
    <HeroHighlight className="min-h-screen min-w-screen flex items-center flex-col">
      <div className="w-screen flex flex-col items-center">
        <div className="flex min-w-full justify-between p-4">
          <MainNav isVisible items={dashboardConfig.mainNav} />
          <MainNav items={dashboardConfig.sidebarNav} />
        </div>
      </div>
      <div className="w-full flex flex-col flex-auto justify-center items-center">
        <div className="min-w-full flex flex-col justify-center items-center text-center p-10 gap-3">
          <h1 className="mb-5 text-5xl">Let's Rank It.</h1>
          <span className="w-1/2 max-w-3xl flex justify-center items-center gap-3">
            <Input
              placeholder="Pokemon"
              value={title}
              onChange={handleValueChange}
              className="w-full"
            />
            <SearchIcon className="h-8 w-8" />
          </span>
          {isError || isFetching ? (
            <LoadingSpinner className="h-4 w-4 text-blue-300" />
          ) : (
            <Card
              x-chunk="dashboard-06-chunk-0"
              className="max-w-5xl md:max-w-5xl lg:max-w-9xl dark:bg-gray-900 bg-gray-200"
            >
              <CardHeader>
                <CardTitle>Media</CardTitle>
              </CardHeader>
              <CardContent>
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead className="hidden sm:table-cell text-xl">
                        <span className="sr-only">Media Image</span>
                      </TableHead>
                      <TableHead>Title</TableHead>
                      <TableHead>Type</TableHead>
                      <TableHead>Created on</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {data?.response.map((item, index) => {
                      let icon: ReactElement;
                      switch (item.type) {
                        case "anime":
                          icon = <Tv />;
                          break;
                        case "movies":
                          icon = <Clapperboard />;
                          break;
                        case "songs":
                          icon = <AudioLines />;
                          break;
                        case "videogames":
                          icon = <LucideGamepad />;
                      }
                      return (
                        <TableRow
                          key={index}
                          className="cursor-pointer"
                          onClick={onItemSelect(item)}
                        >
                          <TableCell className="hidden sm:table-cell">
                            <img
                              alt="Media Image"
                              className="aspect-square rounded-md object-cover"
                              height="64"
                              src={item.mediaimage}
                              onError={(event) => {
                                event.currentTarget.src = "/eepy.png";
                              }}
                              width="64"
                            />
                          </TableCell>
                          <TableCell className="font-medium">
                            {item.title}
                          </TableCell>
                          <TableCell className="md:table-cell">
                            {icon}
                          </TableCell>
                          <TableCell>{item.createdon?.toString()}</TableCell>
                        </TableRow>
                      );
                    })}
                  </TableBody>
                </Table>
              </CardContent>
            </Card>
          )}
        </div>
      </div>
    </HeroHighlight>
  );
};

export default SearchPage;
