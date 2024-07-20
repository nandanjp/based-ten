'use client';
import { ListItem, ListItems } from '@/components/blocks/AddListItem/types';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Trash2, Upload, View } from 'lucide-react';
import { useEffect, useMemo, useState } from 'react';
import { MediaType } from '../../../../services/api.types';
import { useRouter } from 'next/navigation';
import { useMediaByType, useMediaByTypeAndId } from '../../../../services/queries';
import { ViewListItem } from '@/components/blocks/AddListItem/ViewListItem';
import { useParams } from 'next/navigation';
import { useListByName } from '../../../../services/queries';

const placeholderItems: ListItem[] = Array.from({ length: 10 }, (_, index) => ({
  media_image: 'https://via.placeholder.com/150',
  title: 'A Real Item'
}));

const ViewListPage = () => {
  const { list_name } = useParams<{ list_name: string }>();
  // const [listItems, setListItems] = useState<Array<ListItem | undefined>>(
  //   Array(10),
  // );
  // console.log(list_name);
  const list = useListByName(list_name);
  const listItems = placeholderItems;
  
  if (list.isPending) {
    return <span>Loading...</span>;
  }

  if (list.isError) {
    return <span>There was an error in retrieving the list!</span>;
  }

  if (list.isFetching) {
    return <span>Fetching the list...</span>;
  }

  if (!list.data) {
    return <span>There is no list data!</span>;
  }

  console.log(list.data);

  return (
    <div className="p-8 h-full flex justify-between">
      <div className="flex flex-col">
        <h1 className="text-4xl font-bold text-gray-800">List Title</h1>
        <div className="bg-black w-16 h-px" />
        <h2 className="py-1 font-bold">by JustinLin905</h2>
        <h3 className="font-bold italic text-sm pb-4">
          last updated {new Date().toISOString().slice(0, 10)}
        </h3>
        <div className="flex gap-2">
        </div>
      </div>
      <div className="flex flex-col gap-4">
        {listItems.map((item, index) => (
          <div
            key={`${index}=${item?.id}-${item?.type}`}
            className="flex gap-4 items-center justify-between"
          >
            <div className="text-4xl font-semibold text-gray-800">{`${
              index + 1
            }.`}</div>
            <ViewListItem
              listItem={item}
            />
          </div>
        ))}
      </div>
    </div>
  );
};

export default ViewListPage;
