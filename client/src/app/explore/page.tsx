'use client';
import Navbar from '@/components/blocks/Navbar/Navbar';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@radix-ui/react-collapsible';
import { ChevronsUpDown, Heart } from 'lucide-react';
import { useAllLists, useRecommendedLists } from '../../../services/queries';
import { listTypes } from '../../../services/api.types';
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from '@/components/ui/carousel';
import { Card, CardContent } from '@/components/ui/card';

const ExplorePage = () => {
  const lists = useAllLists();
  const recommendedLists = useRecommendedLists('ethan.ramirez');
  return (
    <div className="w-screen h-screen text-gray-600">
      <Navbar />
      <div className="flex flex-col p-4 gap-8">
        <h1 className="text-4xl font-bold">Explore Lists</h1>
        <div className="flex flex-col w-full items-center gap-4">
          <h2 className="text-lg font-semibold">Recommended Lists for You</h2>
          <Carousel className="w-1/2">
            <CarouselContent className="-ml-1">
              {recommendedLists.data?.map((list, index) => (
                <CarouselItem
                  key={index}
                  className="lg:basis-1/4 md:basis-1/3 sm:basis-1/2"
                >
                  <Card>
                    <CardContent className="flex aspect-square items-center justify-center p-6">
                      <span>
                        {list.list_name} by {list.user_name}
                      </span>
                    </CardContent>
                  </Card>
                </CarouselItem>
              ))}
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
        {listTypes.map((listType) => (
          <Collapsible>
            <CollapsibleTrigger className="flex gap-2">
              <ChevronsUpDown />
              <h2 className="text-2xl">{listType.toUpperCase()}</h2>
            </CollapsibleTrigger>
            <CollapsibleContent className="m-4">
              {lists.data
                ?.filter((list) => list.list_type === listType)
                .map((list, index) => (
                  <div
                    className="flex justify-between border-b-2 p-2 items-center"
                    key={index}
                  >
                    <span>
                      {list.list_name} by {list.user_name}
                    </span>
                    <div className="flex flex-col items-center">
                      <Heart
                        className="cursor-pointer"
                        onClick={(event) =>
                          event.currentTarget.setAttribute(
                            'fill',
                            event.currentTarget.getAttribute('fill') === 'pink'
                              ? 'none'
                              : 'pink',
                          )
                        }
                      />
                      {list.likes}
                    </div>
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
