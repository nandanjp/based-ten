import {
  CommandDialog,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
} from "@/components/ui/command";
import Link from "next/link";
import { useEffect, useState } from "react";
import { useAllMedia } from "../../services/queries";
import { undefined } from "zod";

export function CommandMenu() {
  const [open, setOpen] = useState(false);
  const { data, isLoading, isPending, isFetching } = useAllMedia(null);

  // if (isLoading || isPending || isFetching) {
  //   return (
  //     <span>Waiting for your media list creation journey to begin....</span>
  //   );
  // }

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((open) => !open);
      }
    };
    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <CommandInput placeholder="Type a command or search..." />
      <CommandList className="p-5">
        <CommandEmpty>No results found.</CommandEmpty>
        <CommandGroup heading="Suggestions">
          {data?.map((item, i) => (
            <CommandItem key={i} className="p-3">
              <Link
                href={`/create-list/?itemId=${item.id}&mediaType=${item.listtype}`}
              >
                {item.title}
              </Link>
            </CommandItem>
          ))}
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
}
