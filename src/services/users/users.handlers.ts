import { Request, Response } from 'express'
import {
  createFollowing,
  createUser,
  createUserGroup,
  createUserList,
  deleteUser,
  deleteUserFollow,
  deleteUserLike,
  deleteUserList,
  getAllFollowersOfUser,
  getAllFollowingOfUser,
  getAllGroupMembersOfUserGroup,
  getAllGroupsOfUser,
  getAllLikedListsOfUser,
  getAllUsers,
  getAllUsersLists,
  getGroupOfUser,
  getUser,
  getUserList
} from './users.service'
import {
  CreateGroupType,
  CreateListType,
  CreateUserType,
  RemoveListLikeType,
  UnfollowUserType,
  UserGroupsParamsType,
  UserListType,
  UserParamsType
} from './users.schema'
import { FollowsFileType } from '../../db/seed/data-schemas'

export const getAllUsersHandler = async (req: Request, res: Response) => {
  const users = await getAllUsers()
  return res.status(200).send(users)
}

export const getUserHandler = async (req: Request<UserParamsType, {}, {}, {}>, res: Response) => {
  const user = await getUser(req.params)
  return res.status(200).send(user)
}

export const getUserListsHandler = async (req: Request<UserParamsType>, res: Response) => {
  const lists = await getAllUsersLists(req.params)
  return res.status(200).send(lists)
}

export const getUserListHandler = async (req: Request<UserListType>, res: Response) => {
  const list = await getUserList(req.params)
  return res.status(200).send(list)
}

export const getAllLikedListsHandler = async (req: Request<UserParamsType>, res: Response) => {
  const liked = await getAllLikedListsOfUser(req.params)
  return res.status(200).send(liked)
}

export const getAllGroupsHandler = async (req: Request<UserParamsType>, res: Response) => {
  const groups = await getAllGroupsOfUser(req.params)
  return res.status(200).send(groups)
}

export const getGroupHandler = async (req: Request<UserGroupsParamsType>, res: Response) => {
  const group = await getGroupOfUser(req.params)
  return res.status(200).send(group)
}

export const getGroupMembersHandler = async (req: Request<UserGroupsParamsType>, res: Response) => {
  const members = await getAllGroupMembersOfUserGroup(req.params)
  return res.status(200).send(members)
}

export const getAllFollowersHandler = async (req: Request<UserParamsType>, res: Response) => {
  const followers = await getAllFollowersOfUser(req.params)
  return res.status(200).send(followers)
}

export const getAllFollowingHandler = async (req: Request<UserParamsType>, res: Response) => {
  const following = await getAllFollowingOfUser(req.params)
  return res.status(200).send(following)
}

export const createUserHandler = async (req: Request<{}, {}, CreateUserType>, res: Response) => {
  const newUser = await createUser(req.body)
  return res.status(201).send(newUser)
}

export const createUserListHandler = async (req: Request<UserParamsType, {}, CreateListType>, res: Response) => {
  const newList = await createUserList(req.params, req.body)
  return res.status(201).send(newList)
}

export const createUserGroupHandler = async (req: Request<UserParamsType, {}, CreateGroupType>, res: Response) => {
  const newGroup = await createUserGroup(req.params, req.body)
  return res.status(201).send(newGroup)
}

export const createFollowingHandler = async (req: Request<UserParamsType, {}, FollowsFileType>, res: Response) => {
  const newFollow = await createFollowing(req.params, req.body)
  return res.status(201).send(newFollow)
}

export const deleteUserHandler = async (req: Request<UserParamsType>, res: Response) => {
  const deleted = await deleteUser(req.params)
  return res.status(204).send(deleted)
}

export const deleteUserFollowHandler = async (req: Request<UnfollowUserType>, res: Response) => {
  const deleted = await deleteUserFollow(req.params)
  return res.status(204).send(deleted)
}

export const deleteUserListHandler = async (req: Request<UserListType>, res: Response) => {
  const deleted = await deleteUserList(req.params)
  return res.status(204).send(deleted)
}

export const deleteUserLikeHandler = async (req: Request<RemoveListLikeType>, res: Response) => {
  const deleted = await deleteUserLike(req.params)
  return res.status(204).send(deleted)
}
