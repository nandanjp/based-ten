"use client";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  CommandDialog,
  CommandEmpty,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { useState } from "react";
import { AddListItemProps } from "./types";

export const AddListItem = (props: AddListItemProps) => {
  const { onClick, listItem, list } = props;
  const [open, setOpen] = useState<boolean>(false);
  return (
    <div
      onClick={() => {
        setOpen(!open);
      }}
      className="max-w-9xl flexcol p-6"
    >
      <Card className="w-96">
        <CardHeader>
          <CardTitle>{listItem?.title ?? "Select an item..."}</CardTitle>
        </CardHeader>
        <CardContent>
          <img
            src={listItem?.mediaimage ?? "/eepy.png"}
            alt="media image"
            height={96}
            width={96}
            onError={(event) => (event.currentTarget.src = "/eepy.png")}
          />
        </CardContent>
      </Card>
      <CommandDialog open={open} onOpenChange={setOpen}>
        <CommandInput placeholder="Search" />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          {list?.map((item) => (
            <CommandItem
              onSelect={() => {
                onClick?.(item);
                setOpen(false);
              }}
              key={`${item.id}-${item.type}`}
            >
              {item.title}
            </CommandItem>
          ))}
        </CommandList>
      </CommandDialog>
    </div>
  );
};
