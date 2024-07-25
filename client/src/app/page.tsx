"use client";
import {
  Command,
  CommandEmpty,
  CommandList,
  CommandInput,
  CommandDialog,
  CommandGroup,
  CommandItem,
} from "@/components/ui/command";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Navbar from "@/components/blocks/Navbar/Navbar";
import { useMutation, useQuery } from "@tanstack/react-query";
import { getMedia } from "./actions";
import { LoadingSpinner } from "@/components/animated/Spinner";
import { MediaType } from "../../services/api.types";
import { useDebounce } from "@/hooks/useDebounce";
import { User } from "lucide-react";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";


const SearchPage = () => {
  const router = useRouter();
  const [open, setOpen] = useState<boolean>(false);
  const [title, setTitle] = useState<string>("mario");
  const debouncedSearch = useDebounce(title);
  const { data, isError, isFetching } = useQuery({
    queryKey: [`media?title=${debouncedSearch}`],
    queryFn: async () => {
      return await getMedia({ page: 0, limit: 100, title: debouncedSearch });
    },
  });

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen(!open);
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  const handleValueChange = (value: string) => {
    setTitle(value);
  };
  const onItemSelect = (item: MediaType) => {
    return () => {
      router.push(`/create-list/${item.type}?itemId=${item.id}`);
    };
  };

  console.log(debouncedSearch);

  return (
    <div
      style={{
        background: "linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e)",
        backgroundSize: "200% 200%",
        animation: "gradientFlow 10s ease infinite",
      }}
      className="h-screen w-screen flex items-center flex-col"
    >
      <div className="w-screen flex flex-col items-center">
        <div style={{ width: '100%', display: 'flex', justifyContent: 'space-between' }}>
          <MainNav isVisible items={dashboardConfig.mainNav}/>
          <MainNav items={dashboardConfig.sidebarNav}/>
        </div>
      </div>
      <div className="flex flex-col flex-auto justify-center items-center">
        <div className="flex flex-col text-center p-10">
          <h1
            style={{ textShadow: "0 2px 4px rgba(0, 0, 0, 0.2)" }}
            className="mb-5 text-white text-5xl"
          >
            Let's Rank It.
          </h1>
          {isError ? (
            <LoadingSpinner className="text-blue-300" />
          ) : (
            <>
              <p className="text-sm drop-shadow bg-black rounded-lg py-4 px-6">
                Search for Media{" "}
                <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
                  <span className="text-xs">⌘</span>K
                </kbd>
              </p>
              <CommandDialog open={open} onOpenChange={setOpen}>
                <CommandInput
                  placeholder="Type a command or search..."
                  onValueChange={handleValueChange}
                  value={title}
                />
                <CommandList>
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
                  <CommandGroup heading="Settings">
                    {data?.response.map((item) => {
                      return (
                        <CommandItem
                          className="flex p-4 hover:bg-gray-200 cursor-pointer"
                          onSelect={onItemSelect(item)}
                          key={`${item.id}-${item.type}`}
                        >
                          <User className="mr-2 h-4 w-4" />
                          <span>{item.title}</span>
                        </CommandItem>
                      );
                    })}
                  </CommandGroup>
                </CommandList>
              </CommandDialog>
            </>
          )}
        </div>
      </div>
    </div>
  );
};

export default SearchPage;
