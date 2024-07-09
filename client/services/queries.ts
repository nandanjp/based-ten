import { useQuery } from "@tanstack/react-query";
import {
  getAllAnime,
  getAllFollows,
  getAllGames,
  getAllGroups,
  getAllLists,
  getAllMedia,
  getAllMovies,
  getAllSongs,
  getAllUsers,
  getAnimeById,
  getGameById,
  getMovieById,
  getSongById,
  getUserList,
  getUsersLists,
} from "./api";

export const useAllAnime = async () =>
  useQuery({
    queryKey: ["anime"],
    queryFn: getAllAnime,
  });

export const useAnimeById = async () =>
  useQuery({
    queryKey: ["animeid"],
    queryFn: getAnimeById,
  });

export const useAllMovies = async () =>
  useQuery({
    queryKey: ["movies"],
    queryFn: getAllMovies,
  });

export const useMovieById = async () =>
  useQuery({
    queryKey: ["movieid"],
    queryFn: getMovieById,
  });

export const useAllSongs = async () =>
  useQuery({
    queryKey: ["songs"],
    queryFn: getAllSongs,
  });

export const useSongById = async () =>
  useQuery({
    queryKey: ["songid"],
    queryFn: getSongById,
  });

export const useAllGames = async () =>
  useQuery({
    queryKey: ["games"],
    queryFn: getAllGames,
  });

export const useGameById = async () =>
  useQuery({
    queryKey: ["gameid"],
    queryFn: getGameById,
  });

export const useAllMedia = async () =>
  useQuery({
    queryKey: ["media"],
    queryFn: getAllMedia,
  });

export const useAllLists = async () =>
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

export const useAllLikes = async () =>
  useQuery({
    queryKey: ["likes"],
    queryFn: getAllLists,
  });

export const useAllFollows = async () =>
  useQuery({
    queryKey: ["follows"],
    queryFn: getAllFollows,
  });

export const useAllGroups = async () =>
  useQuery({
    queryKey: ["groups"],
    queryFn: getAllGroups,
  });
