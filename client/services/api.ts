import axios from "axios";
import { CreateUserType, UserResponse } from "./api.types";

const BASE_URL = `http://localhost:5000`;
const axiosInstance = axios.create({ baseURL: BASE_URL });

export const getAllAnime = async () => {};

export const getAnimeById = async () => {};

export const createAnimeById = async () => {};

export const updateAnimeById = async () => [];

export const deleteAnimeById = async () => {
  "/anime";
};

export const getAllMovies = async () => {};

export const getMovieById = async () => {};

export const createMovie = async () => {};

export const updateMovie = async () => {};

export const deleteMovie = async () => {};

export const getAllSongs = async () => {};

export const getSongById = async () => {};

export const createSong = async () => {};

export const updateSong = async () => {};

export const deleteSong = async () => {};

export const getAllGames = async () => {};

export const getGameById = async () => {};

export const createGame = async () => {};

export const updateGame = async () => {};

export const deleteGame = async () => {};

export const getAllMedia = async () => {};

export const getAllLists = async () => {};

export const createList = async () => {};

export const getUsersLists = async () => {};

export const getUserList = async () =>
  (await axios.get<UserResponse[]>("users")).data;

export const getUserListItems = async ({ id }: { id: string }) =>
  (await axios.get<UserResponse>(`users/{id}`)).data;

export const updateUserList = async () => {};

export const deleteUserList = async () => {};

export const getUserListItem = async () => {};

export const createUserListItem = async () => {};

export const updateUserListItem = async () => {};

export const deleteUserListItem = async () => {};

export const getAllUsers = async () =>
  (await axios.get<UserResponse[]>("users")).data;

export const getUserByEmail = async ({ email }: { email: string }) =>
  (await axios.get<UserResponse[]>(`users/${email}`)).data;

export const createUser = async (user: CreateUserType) =>
  (await axios.post<UserResponse[]>(`users`, user)).data;

export const updateUser = async () => {};

export const deleteUser = async ({ email }: { email: string }) =>
  (await axios.delete<UserResponse>(`users/${email}`)).data;

export const getAllLikes = async () => {};

export const getLikeByEmail = async () => {};

export const createLike = async () => [];

export const deleteLike = async () => {};

export const getAllFollows = async () => {};

export const createFollows = async () => {};

export const getUsersFollowers = async () => {};

export const deleteUserFollower = async () => {};

export const getAllGroups = async () => {};

export const getGroupById = async () => {};

export const createGroup = async () => {};

export const deleteGroup = async () => {};
