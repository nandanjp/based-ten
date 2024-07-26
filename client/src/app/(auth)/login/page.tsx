"use client";

import { UserContext } from "@/app/context";
import LoginForm from "@/components/LoginForm";
import { useRouter } from "next/navigation";
import { useContext, useEffect } from "react";

const LoginPage = () => {
  const router = useRouter();
  const { user } = useContext(UserContext);
  useEffect(() => {
    if (user || localStorage.getItem("token")) {
      router.push("/");
    }
  }, []);

  return <LoginForm />;
};

export default LoginPage;
