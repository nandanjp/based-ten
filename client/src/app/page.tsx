"use client";
import { LoadingSpinner } from "@/components/animated/Spinner";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";
import {
  Command,
  CommandEmpty,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { useDebounce } from "@/hooks/useDebounce";
import { useQuery } from "@tanstack/react-query";
import { AudioLines, Clapperboard, LucideGamepad, Tv } from "lucide-react";
import { useRouter } from "next/navigation";
import { ReactElement, useState } from "react";
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

  const handleValueChange = (value: string) => {
    setTitle(value);
    setOpen(!!value);
  };
  const onItemSelect = (item: MediaType) => {
    return () => {
      router.push(`/create-list/${item.type}?itemId=${item.id}`);
    };
  };

  const mediaItems = (
    <>
      <CommandEmpty>
        {isFetching ? (
          <span>
            Getting results...
            <LoadingSpinner className="text-blue-300" />
          </span>
        ) : (
          "No results"
        )}
      </CommandEmpty>
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
      <div className="flex flex-col flex-auto justify-center items-center">
        <div className="flex flex-col text-center p-10">
          <h1
            style={{ textShadow: "0 2px 4px rgba(0, 0, 0, 0.2)" }}
            className="mb-5 text-5xl"
          >
            Let's Rank It.
          </h1>
          {isError ? (
            <LoadingSpinner className="text-blue-300" />
          ) : (
            <Command className="w-96 drop-shadow">
              <CommandInput
                placeholder="Search..."
                onValueChange={handleValueChange}
                value={title}
              />
              <CommandList>{open && mediaItems}</CommandList>
            </Command>
          )}
        </div>
      </div>
    </div>
  );
};

export default SearchPage;
