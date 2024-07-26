import { HeroHighlight } from "@/components/animated/HeroHighlight";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="w-screen flex flex-col items-center">
      <div
        style={{
          width: "100%",
          display: "flex",
          justifyContent: "space-between",
        }}
        className="p-4"
      >
        <MainNav isVisible items={dashboardConfig.mainNav} />
        <MainNav items={dashboardConfig.sidebarNav} />
      </div>
      <HeroHighlight className="w-full grid min-h-screen">
        {children}
      </HeroHighlight>
    </div>
  );
}
