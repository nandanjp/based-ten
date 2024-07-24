import { HeroHighlight } from "@/components/animated/HeroHighlight";
import Image from "next/image";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <HeroHighlight className="w-full grid min-h-screen">
      {children}
    </HeroHighlight>
  );
}
