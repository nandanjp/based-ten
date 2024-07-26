"use client";
import { Card, CardTitle } from "@/components/animated/GoodCard";
import { CardContent, CardHeader } from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { usePopularItemsByType } from "../../../services/queries";
import { LoadingSpinner } from "@/components/animated/Spinner";
import { ListType, listType } from "../../../services/api.types";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";

export default function PopularPage() {
  const [mediaType, setMediaType] = useState<ListType>("anime");
  const {
    data: popularMovies,
    isError,
    isFetching,
    refetch,
  } = usePopularItemsByType(mediaType);

  useEffect(() => {
    refetch();
  }, [mediaType]);

  return (
    <div className="w-full flex flex-col items-center justify-center text-center p-10 gap-3">
      <h1
        style={{ textShadow: "0 2px 4px rgba(0, 0, 0, 0.2)" }}
        className="mb-5 text-5xl"
      >
        Popular Media
      </h1>
      <Card
        x-chunk="dashboard-06-chunk-0"
        className="max-w-5xl md:max-w-5xl lg:max-w-9xl"
      >
        <CardHeader>
          <CardTitle>{mediaType.toUpperCase()}</CardTitle>
          <div className="flex gap-4 justify-center">
            {Object.keys(listType.Enum).map((lt, index) => (
              <Button
                variant={"outline"}
                className={`border-b-2 ${mediaType == lt && "border-black"} `}
                onClick={() => setMediaType(lt as ListType)}
                key={index}
              >
                {lt}
              </Button>
            ))}
          </div>
        </CardHeader>
        <CardContent>
          {isError || isFetching ? (
            <LoadingSpinner className="h-4 w-4 text-blue-300" />
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="hidden sm:table-cell text-xl">
                    <span className="sr-only">Media Image</span>
                  </TableHead>
                  <TableHead className="text-center">Title</TableHead>
                  <TableHead className="text-center">Type</TableHead>
                  <TableHead className="text-center">Created on</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {popularMovies?.response.map((movie, index) => (
                  <TableRow key={index}>
                    <TableCell className="hidden sm:table-cell">
                      <img
                        alt="Media Image"
                        className="aspect-square rounded-md object-cover"
                        height="64"
                        src={movie.mediaimage}
                        onError={(event) => {
                          event.currentTarget.src = "/eepy.png";
                        }}
                        width="64"
                      />
                    </TableCell>
                    <TableCell className="font-medium">{movie.title}</TableCell>
                    <TableCell className="md:table-cell">
                      {movie.totallikes}
                    </TableCell>
                    <TableCell>{movie.createdon?.toString()}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
