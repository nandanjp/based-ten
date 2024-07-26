"use client";
import ViewList from "@/components/ViewList";
import { useParams } from "next/navigation";

const ViewListPage = () => {
  const { list_name, user_name } = useParams<{
    list_name: string;
    user_name: string;
  }>();

  return <ViewList list_name={list_name} user_name={user_name} />;
};

export default ViewListPage;
