"use client";

import { cn } from "@/lib/utils";
import { useTheme } from "next-themes";
import { useEffect, useState } from "react";

interface LogoProps {
  classname?: string;
}

const Logo = ({ classname }: LogoProps) => {
  const [mounted, setMounted] = useState(false);
  const { theme } = useTheme();

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return null;
  }

  return (
    <svg
      height="100%"
      strokeMiterlimit="10"
      className={cn(
        "fill-rule:nonzero;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;",
        classname
      )}
      version="1.1"
      viewBox="0 0 1024 1024"
      width="100%"
      xmlns="http://www.w3.org/2000/svg"
    >
      <defs>
        <linearGradient
          gradientTransform="matrix(-148.419 179.761 -80.5744 -161.715 770.377 541.651)"
          gradientUnits="userSpaceOnUse"
          id="LinearGradient"
          x1="0"
          x2="1"
          y1="0"
          y2="0"
        >
          <stop
            offset="0"
            stopColor={theme === "light" ? "#353535" : "#ffffff"}
          />
          <stop
            offset="1"
            stopColor={theme === "light" ? "#000000" : "#e2e2e2"}
          />
        </linearGradient>
        <linearGradient
          gradientTransform="matrix(176.278 261.299 -73.3546 192.07 222.255 459.983)"
          gradientUnits="userSpaceOnUse"
          id="LinearGradient_2"
          x1="0"
          x2="1"
          y1="0"
          y2="0"
        >
          <stop
            offset="0"
            stopColor={theme === "dark" ? "#ffffff" : "#353535"}
          />
          <stop
            offset="1"
            stopColor={theme === "light" ? "#000000" : "#e2e2e2"}
          />
        </linearGradient>
      </defs>
      <g id="Layer-1">
        <path
          d="M452.874 371.652C424.818 371.652 402.074 394.395 402.074 422.451L402.074 655.869L402.074 669.431L402.074 721.072L447.365 721.072L576.635 721.072L621.926 721.072L621.926 669.431L621.926 655.869L621.926 422.451C621.926 394.395 599.182 371.652 571.126 371.652L452.874 371.652Z"
          fill={theme === "light" ? "#353535" : "#ffffff"}
          fillRule="nonzero"
          opacity="1"
          stroke="#007aff"
          strokeLinecap="butt"
          strokeLinejoin="round"
          strokeWidth="0.01"
        />
        <g opacity="1">
          <path
            d="M750.283 510.696C778.339 510.696 801.082 533.44 801.082 561.495L801.082 670.273C801.082 698.328 778.339 721.072 750.283 721.072L673.233 721.072L667.216 721.072L621.926 721.072L621.926 680.203L621.926 670.702L621.926 551.565L621.926 510.696L664.459 510.696L664.459 510.822C665.371 510.772 666.29 510.696 667.216 510.696L750.283 510.696Z"
            fill="url(#LinearGradient)"
            fillRule="nonzero"
            opacity="1"
            stroke="#007aff"
            strokeLinecap="butt"
            strokeLinejoin="round"
            strokeWidth="0.01"
          />
        </g>
        <path
          d="M273.717 455.243C245.661 455.243 222.918 477.987 222.918 506.042L222.918 670.273C222.918 698.328 245.661 721.072 273.717 721.072L350.767 721.072L356.784 721.072L402.074 721.072L402.074 669.431L402.074 657.425L402.074 506.885L402.074 455.243L359.541 455.243L359.541 455.402C358.629 455.34 357.71 455.243 356.784 455.243L273.717 455.243Z"
          fill="url(#LinearGradient_2)"
          fillRule="nonzero"
          opacity="1"
          stroke="#007aff"
          strokeLinecap="butt"
          strokeLinejoin="round"
          strokeWidth="0.01"
        />
      </g>
    </svg>
  );
};

export default Logo;
