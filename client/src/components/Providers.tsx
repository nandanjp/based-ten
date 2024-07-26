"use client";
import { getCurrentUser } from "@/app/actions";
import { UserContext } from "@/app/context";
import { getQueryClient } from "@/lib/get-query-client";
import { QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import type * as React from "react";
import { useEffect, useState } from "react";
import { UserType } from "../../services/api.types";

export default function Providers({ children }: { children: React.ReactNode }) {
  const queryClient = getQueryClient();
  const [user, setUser] = useState<UserType | undefined>(undefined);
  useEffect(() => {
    getCurrentUser().then((response) => {
      if (response.success) {
        setUser(response.response);
      }
    });
  }, []);

  return (
    <UserContext.Provider value={{ user, setUser }}>
      <QueryClientProvider client={queryClient}>
        {children}
        <ReactQueryDevtools />
      </QueryClientProvider>
    </UserContext.Provider>
  );
}
