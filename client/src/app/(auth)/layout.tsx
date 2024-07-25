import { HeroHighlight } from "@/components/animated/HeroHighlight";
import Image from "next/image";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div>
      <HeroHighlight className="w-full lg:grid lg:grid-cols-2 min-h-screen">
        <div className="flex flex-col gap-3">
          <div className="p-4">
            <MainNav isVisible items={dashboardConfig.mainNav}/>
          </div>
          <div className="mx-auto w-[350px] gap-6 flex-1 flex items-center">
            {children}
            </div>
        </div>
        <div className="hidden bg-primary lg:block">
          <Image
            src="/howls-1.jpeg"
            alt="Image"
            width="1920"
            height="1080"
            className="h-full w-full object-cover object-right dark:brightness-[0.9]"
          />
        </div>
      </HeroHighlight>
    </div>
  );
}
