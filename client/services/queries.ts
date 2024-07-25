import {
  getAllLists,
  getAllUsers,
  getCurrentUser,
  getGroupById,
  getGroupMemberLists,
  getMedia,
  getRecommendedGroups,
  getRecommendedLists,
  getUserByUsername,
  getUserFollowers,
  getUserFollowing,
  getUserGroups,
  getUserLikes,
  getUserList,
  getUsersLikes,
  getUsersLists,
  getUserListType,
} from "@/app/actions";
import { useQuery } from "@tanstack/react-query";

export const useAllLists = () =>
  useQuery({
    queryKey: ["lists"],
    queryFn: async () => {
      return await getAllLists();
    },
  });

export const useUsersLists = (username: string) =>
  useQuery({
    queryKey: ["user-lists"],
    queryFn: async () => {
      return await getUsersLists(username);
    },
  });

export const useUserList = <T>({
  username,
  list_name,
}: {
  username: string;
  list_name: string;
}) =>
  useQuery({
    queryKey: ["user-list"],
    queryFn: async () => {
      return await getUserList<T>(username, list_name);
    },
  });

export const useAllUsers = async () =>
  useQuery({
    queryKey: ["users"],
    queryFn: async () => {
      return await getAllUsers();
    },
  });

export const useUser = (username: string) =>
  useQuery({
    queryKey: ["user"],
    queryFn: async () => {
      return await getUserByUsername(username);
    },
  });

export const useCurrentUser = () =>
  useQuery({
    queryKey: ["current-user"],
    queryFn: async () => {
      return await getCurrentUser();
    },
  });

export const useAllLikes = async () =>
  useQuery({
    queryKey: ["likes"],
    queryFn: async () => {
      return await getAllLists();
    },
  });

export const useUserLikes = (username: string) =>
  useQuery({
    queryKey: ["user-likes"],
    queryFn: async () => {
      return await getUserLikes(username);
    },
  });

export const useCurrentUsersLikes = (username: string) =>
  useQuery({
    queryKey: [`users-likes-${username}`],
    queryFn: async () => {
      return await getUsersLikes(username);
    },
  });

export const useUserGroups = (username: string) =>
  useQuery({
    queryKey: ["user-groups"],
    queryFn: async () => {
      return await getUserGroups(username);
    },
  });

export const useUserFollowing = (username: string) =>
  useQuery({
    queryKey: ["user-following"],
    queryFn: async () => {
      return await getUserFollowing(username);
    },
  });

export const useGroupById = (gid: string) =>
  useQuery({
    queryKey: ["group-by-id"],
    queryFn: async () => {
      return await getGroupById(gid);
    },
  });

export const useGroupMemberLists = ({
  gid,
  order_by_author,
}: {
  gid: string;
  order_by_author: boolean;
}) =>
  useQuery({
    queryKey: ["group-member-lists"],
    queryFn: async () => {
      return await getGroupMemberLists(gid, { order_by_author });
    },
  });

export const useRecommendedGroups = (gid: string) =>
  useQuery({
    queryKey: ["recommended-groups"],
    queryFn: async () => {
      return await getRecommendedGroups(gid);
    },
  });

export const useUserFollowers = (username: string) =>
  useQuery({
    queryKey: ["user-followers"],
    queryFn: async () => {
      return await getUserFollowers(username);
    },
  });

export const useAllMedia = () =>
  useQuery({
    queryKey: ["get-all-media"],
    queryFn: async () => {
      return await getMedia({ page: 0, limit: 100 });
    },
  });

export const useRecommendedLists = () =>
  useQuery({
    queryKey: ["get-recommended-lists"],
    queryFn: async () => {
      return await getRecommendedLists();
    },
  });

export const useUserListType = (username: string, list_name: string) =>
  useQuery({
    queryKey: ["user-list-type"],
    queryFn: async () => {
      return await getUserListType(username, list_name);
    },
  });