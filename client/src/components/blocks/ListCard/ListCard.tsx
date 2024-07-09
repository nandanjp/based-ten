import * as React from "react"

import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"

export function ListCard() {
  return (
    <Card className="w-[350px]">
      <CardHeader>
        <CardTitle>List name</CardTitle>
        <CardDescription className="italic">list type</CardDescription>
      </CardHeader>
      <CardFooter className="flex justify-between">
        <p className="">Author: _</p>
      </CardFooter>
    </Card>
  )
}
