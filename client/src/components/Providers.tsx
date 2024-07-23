"use client";
import { QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { getQueryClient } from "@/lib/get-query-client";
import type * as React from "react";
import { UserContext } from "@/app/context";
import { getCurrentUser } from "../../services/api";
import { User } from "../../services/api.types";
import { useState, useEffect } from "react";

export default function Providers({ children }: { children: React.ReactNode }) {
  const queryClient = getQueryClient();
  const [user, setUser] = useState<User | undefined>(undefined);
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
