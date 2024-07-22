"use client";

import { useRouter } from "next/navigation";
import { useUserStore } from "@/store";
import { CommandMenu } from "@/components/SearchBar";
import { useEffect } from "react";

const SearchPage = () => {
  const router = useRouter();
  const hasToken = useUserStore((store) => store.has_token);

  useEffect(() => {
    if (!hasToken()) {
      router.push("/login");
    }
  }, []);

  return (
    <div
      style={{
        background: "linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e)",
        backgroundSize: "200% 200%",
        animation: "gradientFlow 10s ease infinite",
      }}
      className="h-screen w-screen flex items-center flex-col"
    >
      <div className="flex flex-col flex-auto justify-center items-center">
        <div className="flex flex-col text-center p-10">
          <h1
            style={{ textShadow: "0 2px 4px rgba(0, 0, 0, 0.2)" }}
            className="mb-5 text-white text-5xl"
          >
            Let's Rank It.
          </h1>
          <p className="text-sm text-muted-foreground bg-black p-3 rounded-lg">
            Press{" "}
            <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
              <span className="text-xs">⌘</span>K
            </kbd>
          </p>
          <CommandMenu />
        </div>
      </div>
    </div>
  );
};

export default SearchPage;
