import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="w-full flex flex-col justify-between items-center gap-4 min-h-screen">
      <div className="min-w-full justify-between p-4 flex sticky inset-0 top-0 z-50">
        <MainNav isVisible items={dashboardConfig.mainNav} />
        <MainNav items={dashboardConfig.sidebarNav} />
      </div>
      <div className="mt-8 self-center min-w-full flex flex-col items-center justify-center gap-4 p-12">
        {children}
      </div>
    </div>
  );
}
