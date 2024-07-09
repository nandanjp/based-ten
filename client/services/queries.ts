import { useQuery } from '@tanstack/react-query';
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
  getUserByEmail,
  getAnimeById,
  getGameById,
  getMovieById,
  getSongById,
  getUserList,
  getUsersLists,
  getUserFollowing,
  getUserFollowers,
  getUserLikes,
  getMediaByTypeAndId,
  getAllMediaByType,
} from './api';
import { MediaType } from './api.types';

export const useAllAnime = async () =>
  useQuery({
    queryKey: ['anime'],
    queryFn: getAllAnime,
  });

export const useAnimeById = async () =>
  useQuery({
    queryKey: ['animeid'],
    queryFn: getAnimeById,
  });

export const useAllMovies = async () =>
  useQuery({
    queryKey: ['movies'],
    queryFn: getAllMovies,
  });

export const useMovieById = async () =>
  useQuery({
    queryKey: ['movieid'],
    queryFn: getMovieById,
  });

export const useAllSongs = async () =>
  useQuery({
    queryKey: ['songs'],
    queryFn: getAllSongs,
  });

export const useSongById = async () =>
  useQuery({
    queryKey: ['songid'],
    queryFn: getSongById,
  });

export const useAllGames = async () =>
  useQuery({
    queryKey: ['games'],
    queryFn: getAllGames,
  });

export const useGameById = async () =>
  useQuery({
    queryKey: ['gameid'],
    queryFn: getGameById,
  });

export const useAllLists = async () =>
  useQuery({
    queryKey: ['lists'],
    queryFn: getAllLists,
  });

export const useUsersLists = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ['user-lists'],
    queryFn: getUsersLists({ email }),
  });

export const useUserList = async () =>
  useQuery({
    queryKey: ['user-list'],
    queryFn: getUserList,
  });

export const useAllUsers = async () =>
  useQuery({
    queryKey: ['users'],
    queryFn: getAllUsers,
  });

export const useUser = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ['user'],
    queryFn: getUserByEmail({ email }),
  });

export const useAllLikes = async () =>
  useQuery({
    queryKey: ['likes'],
    queryFn: getAllLists,
  });

export const useUserLikes = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ['user-likes'],
    queryFn: getUserLikes({ email }),
  });

export const useAllFollows = async () =>
  useQuery({
    queryKey: ['follows'],
    queryFn: getAllFollows,
  });

export const useUserFollowing = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ['user-following'],
    queryFn: getUserFollowing({ email }),
  });

export const useAllGroups = async () =>
  useQuery({
    queryKey: ['groups'],
    queryFn: getAllGroups,
  });

export const useUserFollowers = ({ email }: { email: string }) =>
  useQuery({
    queryKey: ['user-followers'],
    queryFn: getUserFollowers({ email }),
  });

export const useAllMedia = () =>
  useQuery({
    queryKey: ['get-all-media'],
    queryFn: getAllMedia,
  });

export const useMediaByTypeAndId = (mediaType: MediaType, id: string) =>
  useQuery({
    queryKey: ['get-all-media-by-type-and-id'],
    queryFn: getMediaByTypeAndId(mediaType, id),
  });

export const useMediaByType = (mediaType: MediaType) =>
  useQuery({
    queryKey: ['get-media-by-type'],
    queryFn: getAllMediaByType(mediaType),
  });
