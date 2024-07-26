"use client";

import { UserContext } from "@/app/context";
import SignupForm from "@/components/SignupForm";
import { useRouter } from "next/navigation";
import { useContext, useEffect } from "react";

const SignUpPage = () => {
  const router = useRouter();
  const { user } = useContext(UserContext);
  useEffect(() => {
    if (user || localStorage.getItem("token")) {
      router.push("/");
    }
  }, []);

  return <SignupForm />;
};

export default SignUpPage;
