"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormMessage,
} from "./ui/form";
import CardWrapper from "./CardWrapper";
import { useContext, useEffect, useState } from "react";
import { useFormStatus } from "react-dom";
import { getCurrentUser, createGroup } from "@/app/actions";
import { UserContext } from "@/app/context";
import { useRouter } from "next/navigation";

const createGroupSchema = z.object({
  groupname: z.string().min(2, {
    message: "Group Name must have at least 2 characters.",
  }),
});

export default function GroupForm() {
  const router = useRouter();
  const { user, setUser } = useContext(UserContext);
  const [loading, setLoading] = useState(false);

  const form = useForm<z.infer<typeof createGroupSchema>>({
    resolver: zodResolver(createGroupSchema),
    defaultValues: {
      groupname: "",
    },
  });
  const onSubmit = async (values: z.infer<typeof createGroupSchema>) => {
    if (user) {
      const group = await createGroup(user.username, values.groupname);
      if (group.success) {
        router.push("/explore/groups");
      } else {
        form.setError("groupname", {
          message: "Invalid group name",
        });
      }
      setLoading(true);
    }
    
  };

  const { pending } = useFormStatus();

  return (
    <CardWrapper
      label="Create a new group"
      title="Create Group"
      backButtonHref={"/"}
      backButtonLabel="Home"
    >
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <FormField
            control={form.control}
            name="groupname"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Group Name</FormLabel>
                <FormControl>
                  <Input {...field} placeholder="naruto" />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit" className="w-full" disabled={pending}>
            {loading ? "Loading..." : "Create"}
          </Button>
        </form>
      </Form>
    </CardWrapper>
  );
}
