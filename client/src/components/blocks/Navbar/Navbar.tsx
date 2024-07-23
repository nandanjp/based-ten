import Link from "next/link";
import { NavbarProps } from "./types";
import { cn } from "@/lib/utils";
import { UserContext } from "@/app/context";
import { useContext } from "react";
import { ModeToggle } from "@/components/ModeToggleButton";

const Navbar = ({ className }: NavbarProps) => {
  const { user } = useContext(UserContext);
  return (
    <div
      className={cn([
        className,
        "flex justify-between w-full backdrop-filter backdrop-blur-xl text-xl",
      ])}
    >
      <div className="flex gap-2">
        <Link href="/" className="hover:bg-secondary p-3">
          Home
        </Link>
        <Link href="/explore" className="hover:bg-secondary p-3">
          Explore
        </Link>
      </div>
      <div className="flex gap-2 items-center">
        {user ? (
          <Link
            href={`/user/${user.username}`}
            className="hover:bg-secondary p-3"
          >
            {user.username}
          </Link>
        ) : (
          <>
            <Link href="/login" className="hover:bg-secondary p-3">
              Log In
            </Link>
            <Link href="/signup" className="hover:bg-secondary p-3">
              Sign Up
            </Link>
          </>
        )}
        <ModeToggle className="bg-transparent outline-none" />
      </div>
    </div>
  );
};

export default Navbar;
