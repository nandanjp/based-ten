import { DashboardConfig } from "@/components/blocks/Navbar/types/index";

export const dashboardConfig: DashboardConfig = {
  mainNav: [
    {
      title: "Home",
      href: "/",
    },
    {
      title: "Explore Lists",
      href: "/explore/lists",
    },
    {
      title: "Explore Groups",
      href: "/explore/groups",
    },
    {
      title: "Popular",
      href: "/popular",
    },
  ],
  sidebarNav: [
    {
      title: "Log In",
      href: "/login",
    },
    {
      title: "Sign Up",
      href: "/signup",
    },
  ],
};
