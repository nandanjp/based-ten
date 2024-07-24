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
import { useContext, useState } from "react";
import { useFormStatus } from "react-dom";
import { getCurrentUser, loginUser } from "@/app/actions";
import { UserContext } from "@/app/context";
import { useRouter } from "next/navigation";

const loginFormSchema = z.object({
  username: z.string().min(2, {
    message: "Username must be at least 2 characters.",
  }),
  password: z.string().min(2, {
    message: "Password must be at least 2 characters.",
  }),
});

export default function LoginForm() {
  const router = useRouter();
  const { user, setUser } = useContext(UserContext);
  const [loading, setLoading] = useState(false);

  const form = useForm<z.infer<typeof loginFormSchema>>({
    resolver: zodResolver(loginFormSchema),
    defaultValues: {
      username: "",
      password: "",
    },
  });
  const onSubmit = async (values: z.infer<typeof loginFormSchema>) => {
    const loginResult = await loginUser(values.username, values.password);
    console.log(loginResult);
    if (loginResult.success) {
      localStorage.setItem("token", loginResult.response.token);
      const userResponse = await getCurrentUser();
      if (userResponse.response) {
        setUser(userResponse.response);
      }
      router.push("/");
    } else {
      form.setError("password", {
        message: "Invalid username or password",
      });
    }
    setLoading(true);
    console.log("user logged in");
  };

  const { pending } = useFormStatus();

  return (
    <CardWrapper
      label="Login into account"
      title="Login"
      backButtonHref={"/signup"}
      backButtonLabel="Don't have an account? Sign up here"
    >
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Email</FormLabel>
                <FormControl>
                  <Input {...field} placeholder="johndoe@gmail.com" />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input {...field} type="password" placeholder="******" />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit" className="w-full" disabled={pending}>
            {loading ? "Loading..." : "Login"}
          </Button>
        </form>
      </Form>
    </CardWrapper>
  );
}
