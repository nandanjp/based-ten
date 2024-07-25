"use client";
import { Card, CardTitle } from "@/components/animated/GoodCard";
import { LoadingSpinner } from "@/components/animated/Spinner";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";
import { CardContent, CardHeader } from "@/components/ui/card";
import { CommandItem } from "@/components/ui/command";
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
import { ChangeEvent, ReactElement, useState } from "react";
import { MediaType } from "../../services/api.types";
import { getMedia } from "./actions";

const SearchPage = () => {
  const router = useRouter();
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

  const mediaItems = (
    <>
      {data?.response.map((item) => {
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
          <CommandItem
            className="flex gap-4 p-4 hover:bg-gray-200 cursor-pointer"
            onSelect={onItemSelect(item)}
            key={`${item.id}-${item.type}`}
          >
            {icon}
            <span className="whitespace-nowrap overflow-hidden text-ellipsis">
              {item.title}
            </span>
          </CommandItem>
        );
      })}
    </>
  );

  return (
    <div
      style={{
        background:
          "linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e) dark:linear-gradient(to left bottom, #051937, #00476e, #007b9f, #00b2c3, #12ebd6)",
        backgroundSize: "200% 200%",
        animation: "gradientFlow 10s ease infinite",
      }}
      className="h-screen w-screen flex items-center flex-col"
    >
      <div className="w-screen flex flex-col items-center">
        <div className="flex min-w-full justify-between p-4">
          <MainNav isVisible items={dashboardConfig.mainNav} />
          <MainNav items={dashboardConfig.sidebarNav} />
        </div>
      </div>
      <div className="w-full flex flex-col flex-auto justify-center items-center">
        <div className="min-w-full flex flex-col justify-center items-center text-center p-10 gap-3">
          <h1
            style={{ textShadow: "0 2px 4px rgba(0, 0, 0, 0.2)" }}
            className="mb-5 text-5xl"
          >
            Let's Rank It.
          </h1>
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
              className="max-w-5xl md:max-w-5xl lg:max-w-9xl"
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
                      <TableHead>Created On</TableHead>
                      <TableHead>Type</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {data?.response.map((item) => {
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
                          className="cursor-pointer"
                          onClick={onItemSelect(item)}
                        >
                          <TableCell className="hidden sm:table-cell">
                            <img
                              alt="Media Image"
                              className="aspect-square rounded-md object-cover"
                              height="64"
                              src={item.mediaimage}
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
    </div>
  );
};

export default SearchPage;
