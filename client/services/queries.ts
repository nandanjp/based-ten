import { useQuery } from "@tanstack/react-query";
import {
  getAllLists,
  getAllMedia,
  getAllUsers,
  getUserByEmail,
  getUserList,
  getUsersLists,
  getUserFollowing,
  getUserFollowers,
  getUserLikes,
  getMediaByTypeAndId,
  getAllMediaByType,
  getGroupById,
  getGroupMemberLists,
  getRecommendedGroups,
  getRecommendedLists,
  getList,
} from "./api";
import { MediaType } from "./api.types";

export const useAllLists = () =>
  useQuery({
    queryKey: ["lists"],
    queryFn: getAllLists,
  });

export const useUsersLists = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ["user-lists"],
    queryFn: getUsersLists({ email }),
  });

export const useUserList = async () =>
  useQuery({
    queryKey: ["user-list"],
    queryFn: getUserList,
  });

export const useAllUsers = async () =>
  useQuery({
    queryKey: ["users"],
    queryFn: getAllUsers,
  });

export const useUser = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ["user"],
    queryFn: getUserByEmail({ email }),
  });

export const useAllLikes = async () =>
  useQuery({
    queryKey: ["likes"],
    queryFn: getAllLists,
  });

export const useUserLikes = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ["user-likes"],
    queryFn: getUserLikes({ email }),
  });

export const useUserFollowing = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ["user-following"],
    queryFn: getUserFollowing({ email }),
  });

export const useGroupById = ({ group_id }: { group_id: string }) =>
  useQuery({
    queryKey: ["group-by-id"],
    queryFn: getGroupById({ group_id }),
  });

export const useGroupMemberLists = ({
  group_id,
  orderByAuthor,
}: {
  group_id: string;
  orderByAuthor: boolean;
}) =>
  useQuery({
    queryKey: ["group-member-lists"],
    queryFn: getGroupMemberLists({ group_id, orderByAuthor }),
  });

export const useRecommendedGroups = ({ group_id }: { group_id: string }) =>
  useQuery({
    queryKey: ["recommended-groups"],
    queryFn: getRecommendedGroups({ group_id }),
  });

export const useUserFollowers = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ["user-followers"],
    queryFn: getUserFollowers({ email }),
  });

export const useAllMedia = () =>
  useQuery({
    queryKey: ["get-all-media"],
    queryFn: getAllMedia,
  });

export const useMediaByTypeAndId = (mediaType: MediaType, id: string) =>
  useQuery({
    queryKey: ["get-all-media-by-type-and-id"],
    queryFn: getMediaByTypeAndId(mediaType, id),
  });

export const useMediaByType = (mediaType: MediaType) =>
  useQuery({
    queryKey: ["get-media-by-type"],
    queryFn: getAllMediaByType(mediaType),
  });

export const useRecommendedLists = (userId: string) =>
  useQuery({
    queryKey: ["get-recommended-lists"],
    queryFn: getRecommendedLists(userId),
  });

export const useListByName = (list_name: string, user_name: string) =>
  useQuery({
    queryKey: ["list"],
    queryFn: getList(list_name, user_name),
  });
