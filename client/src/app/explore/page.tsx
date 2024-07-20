'use client';
import Navbar from '@/components/blocks/Navbar/Navbar';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@radix-ui/react-collapsible';
import { ChevronsUpDown } from 'lucide-react';
import { useAllLists } from '../../../services/queries';
import { listTypes } from '../../../services/api.types';

const ExplorePage = () => {
  const lists = useAllLists();
  return (
    <div className="w-screen h-screen text-gray-600">
      <Navbar />
      <div className="flex flex-col p-4 gap-8">
        <h1 className="text-4xl font-bold">Explore Lists</h1>
        {listTypes.map((listType) => (
          <Collapsible>
            <CollapsibleTrigger className="flex gap-2">
              <ChevronsUpDown />
              <h2 className="text-2xl">{listType.toUpperCase()}</h2>
            </CollapsibleTrigger>
            <CollapsibleContent className="m-4">
              {lists.data
                ?.filter((list) => list.list_type === listType)
                .map((list) => (
                  <div className="flex justify-between border-b-2 p-2">
                    <span>
                      {list.list_name} by {list.user_name}
                    </span>
                  </div>
                ))}
            </CollapsibleContent>
          </Collapsible>
        ))}
      </div>
    </div>
  );
};

export default ExplorePage;
