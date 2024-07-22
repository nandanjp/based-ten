"use client";

import React, { useState } from "react";
import { HoveredLink, Menu, MenuItem } from "@/components/ui/navbar-menu";
import { cn } from "@/lib/utils";

function Navbar({ className }: { className?: string }) {
  const [active, setActive] = useState<string | null>(null);
  return (
    <div className={cn("fixed inset-x-0 w-full mx-auto z-50", className)}>
      <Menu setActive={setActive}>
        <div className="flex justify-between items-center w-full">
          <MenuItem setActive={setActive} active={active} item="Services">
            <div className="flex flex-col space-y-4 text-sm">
              <HoveredLink href="/">Home</HoveredLink>
              <HoveredLink href="/explore">Explore</HoveredLink>
              <HoveredLink href="/login">Login</HoveredLink>
              <HoveredLink href="/signup">Signup</HoveredLink>
            </div>
          </MenuItem>
          <div className="flex gap-8 justify-center items-center">
            <MenuItem setActive={setActive} active={active} item="Menu">
              <div className="flex flex-col space-y-4 text-sm">
                <HoveredLink href="/">Home</HoveredLink>
                <HoveredLink href="/explore">Explore</HoveredLink>
                <HoveredLink href="/login">Login</HoveredLink>
                <HoveredLink href="/signup">Signup</HoveredLink>
              </div>
            </MenuItem>
          </div>
        </div>
      </Menu>
    </div>
  );
}

export default Navbar;
