"use client";

import { createGroup, joinGroup } from "@/app/actions";
import { UserContext } from "@/app/context";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { zodResolver } from "@hookform/resolvers/zod";
import { useRouter } from "next/navigation";
import { useContext, useState } from "react";
import { useFormStatus } from "react-dom";
import { useForm } from "react-hook-form";
import { z } from "zod";
import CardWrapper from "./CardWrapper";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "./ui/form";

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
        const joinGroupResponse = await joinGroup(group.response.gid);
        if (joinGroupResponse?.success) {
          router.push("/explore/groups");
        }
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
