"use client";

import * as React from "react";
import Link from "next/link";
import { useSelectedLayoutSegment } from "next/navigation";

import { MainNavItem } from "@/components/blocks/Navbar/types/index";
import { cn } from "@/lib/utils";
import { MobileNav } from "@/components/blocks/Navbar/MobileNavCN";
import { UserContext } from "@/app/context";
import { useContext } from "react";
import Logo from "@/components/Logo";
import { ModeToggle } from "@/components/ModeToggleButton";

interface MainNavProps {
  isVisible?: boolean;
  items?: MainNavItem[];
  children?: React.ReactNode;
  classname?: string;
}

export function MainNav({
  isVisible,
  items,
  children,
  classname,
}: MainNavProps) {
  const segment = useSelectedLayoutSegment();
  const [showMobileMenu, setShowMobileMenu] = React.useState<boolean>(false);
  const userContext = useContext(UserContext);
  const user = userContext?.user;

  return (
    <div className={cn("flex items-center gap-6 md:gap-10", classname)}>
      {isVisible && ( // Use isVisible for conditional rendering
        <Link href="/" className="hidden items-center space-x-2 md:flex">
          <span className="hidden font-bold sm:inline-block">
            <span className="flex items-center justify-center">
              <Logo classname="w-12 h-12" />
              Based Ten
            </span>
          </span>
        </Link>
      )}
      {!user || isVisible ? (
        items?.length ? (
          <nav className="hidden gap-6 md:flex">
            {items?.map((item, index) => (
              <Link
                key={index}
                href={item.disabled ? "#" : item.href}
                className={cn(
                  "flex items-center text-lg font-medium transition-colors hover:text-foreground/80 sm:text-sm",
                  item.href.startsWith(`/${segment}`)
                    ? "text-foreground"
                    : "text-foreground/60",
                  item.disabled && "cursor-not-allowed opacity-80"
                )}
              >
                {item.title}
              </Link>
            ))}
          </nav>
        ) : null
      ) : null}
      {user && !isVisible ? (
        <div className="flex items-center justify-center gap-6 ">
          <Link
            href={`/user/${user.username}/me`}
            className={cn(
              "flex items-center text-lg font-medium transition-colors hover:text-foreground/80 sm:text-sm",
              "text-foreground"
            )}
          >
            {user.username}
          </Link>
        
          <ModeToggle />
          <div></div>
        </div>
      ) : null}
      <button
        className="flex items-center space-x-2 md:hidden"
        onClick={() => setShowMobileMenu(!showMobileMenu)}
      >
        <span className="font-bold">Menu</span>
      </button>
      {showMobileMenu && items && (
        <MobileNav items={items}>{children}</MobileNav>
      )}
    </div>
  );
}
