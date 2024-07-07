'use client';
import { getAllMedia } from '@/app/api/media/get-media';
import { Media } from '@/app/api/media/types';
import {
  Command,
  CommandEmpty,
  CommandList,
  CommandInput,
} from '@/components/ui/command';
import { CommandItem } from 'cmdk';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';

const SearchPage = () => {
  const router = useRouter();
  const [open, setOpen] = useState<boolean>(false);
  const [media, setMedia] = useState<Array<Media>>([]);
  const handleValueChange = (value: string) => {
    setOpen(!!value);
  };
  const onItemSelect = (item: Media) => {
    return () => {
      console.log('create list');
      router.push(`/create-list/?itemId=${item.id}&mediaType=${item.type}`);
    };
  };
  useEffect(() => {
    const getMedia = async () => {
      const result = await getAllMedia();
      if (result.success) {
        setMedia([...result.media]);
      }
    };
    getMedia();
  }, []);
  return (
    <div
      style={{
        background: 'linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e)',
      }}
      className="h-screen w-screen flex flex-col justify-center items-center"
    >
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
              media.map((item: Media) => {
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
  );
};

export default SearchPage;
