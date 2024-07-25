import * as React from "react";
import { MainNav } from "@/components/blocks/Navbar/MainNavCN";
import { dashboardConfig } from "@/components/blocks/Navbar/dashboard";

type HeaderProps = {
  title?: string;
  subtitle?: string;
};

const GradientHeader = (props: HeaderProps) => {
  return (
    <div
      style={{
        height: "10rem",
        background:
          "linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e) dark:linear-gradient(to left bottom, #051937, #00476e, #007b9f, #00b2c3, #12ebd6)",
        backgroundSize: "150% 150%",
        animation: "gradientFlow 10s ease infinite",
      }}
      className="flex items-center justify-center flex-col"
    >
      <div className="w-full flex justify-between">
        <MainNav isVisible items={dashboardConfig.mainNav} />
        <MainNav items={dashboardConfig.sidebarNav} />
      </div>
      <div className="flex items-center  pt-12 pl-6 pb-6">
        <div className="grid gap-1 text-center">
          <div
            className="text-4xl font-bold"
            style={{ textShadow: "0 2px 4px rgba(0, 0, 0, 0.3)" }}
          >
            {props.title}
          </div>
          <div
            className="text-sm"
            style={{ textShadow: "0 2px 4px rgba(0, 0, 0, 0.3)" }}
          >
            {props.subtitle}
          </div>
        </div>
      </div>
    </div>
  );
};

export default GradientHeader;
