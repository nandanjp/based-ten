'use client';
import { Media } from '../../services/api.types';
import {
  Command,
  CommandEmpty,
  CommandList,
  CommandInput,
} from '@/components/ui/command';
import { CommandItem } from 'cmdk';
import { useRouter } from 'next/navigation';
import { useState } from 'react';
import { useAllMedia } from '../../services/queries';
import Link from 'next/link';
import { Button } from '@/components/ui/button';
import Navbar from '@/components/blocks/Navbar/Navbar';

const SearchPage = () => {
  const router = useRouter();
  const [open, setOpen] = useState<boolean>(false);
  const media = useAllMedia();
  const handleValueChange = (value: string) => {
    setOpen(!!value);
  };
  const onItemSelect = (item: Media) => {
    return () => {
      router.push(`/create-list/?itemId=${item.id}&mediaType=${item.type}`);
    };
  };
  return (
    <div
      style={{
        background: 'linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e)',
        backgroundSize: '200% 200%',
        animation: 'gradientFlow 10s ease infinite',
      }}
      className="h-screen w-screen flex items-center flex-col"
    >
      <Navbar className="bg-opacity-25 bg-gray-900" />
      <div className="flex flex-col flex-auto justify-center items-center">
        <div className="flex flex-col text-center p-10">
          <h1
            style={{ textShadow: '0 2px 4px rgba(0, 0, 0, 0.2)' }}
            className="mb-5 text-white text-5xl"
          >
            Let's Rank It.
          </h1>
          <Command className="w-96 drop-shadow">
            <CommandInput
              placeholder="Search"
              onValueChange={handleValueChange}
            />
            <CommandList>
              {open && <CommandEmpty>No results found.</CommandEmpty>}
              {open &&
                media?.data?.map((item: Media) => {
                  return (
                    <CommandItem
                      className="flex p-4 hover:bg-gray-200 cursor-pointer"
                      onSelect={onItemSelect(item)}
                      key={`${item.id}-${item.type}`}
                    >
                      {item.title}
                    </CommandItem>
                  );
                })}
            </CommandList>
          </Command>
        </div>
      </div>
    </div>
  );
};

export default SearchPage;
