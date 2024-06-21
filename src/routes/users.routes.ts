import { Router } from 'express'
import { validate } from '../middleware/validate-schema'

import {
  CreateGroupSchema,
  CreateListSchema,
  CreateUserSchema,
  FollowUserSchema,
  RemoveListLikeSchema,
  RemoveUserFromGroupsParamsSchema,
  UnfollowUserSchema,
  UserGroupsParamsSchema,
  UserListSchema,
  UserParamsSchema
} from '../services/users/users.schema'
import {
  createFollowingHandler,
  createUserGroupHandler,
  createUserHandler,
  createUserListHandler,
  deleteUserFollowHandler,
  deleteUserHandler,
  deleteUserLikeHandler,
  deleteUserListHandler,
  getAllFollowersHandler,
  getAllFollowingHandler,
  getAllGroupsHandler,
  getAllLikedListsHandler,
  getAllUsersHandler,
  getGroupHandler,
  getGroupMembersHandler,
  getUserHandler,
  getUserListHandler,
  getUserListsHandler
} from '../services/users/users.handlers'

const router = Router()

router.get('/', getAllUsersHandler) //get all users
router.get('/:email', validate({ params_schema: UserParamsSchema }), getUserHandler) //get user
router.get('/:email/lists', validate({ params_schema: UserParamsSchema }), getUserListsHandler) //get all lists created by user
router.get('/:email/lists/liked', validate({ params_schema: UserParamsSchema }), getAllLikedListsHandler) //get all liked lists
router.get('/:email/lists/:name', validate({ params_schema: UserListSchema }), getUserListHandler) //get all lists created by user
router.get('/:email/groups', validate({ params_schema: UserParamsSchema }), getAllGroupsHandler) //get all groups owned by user
router.get('/:email/groups/:gid/members', validate({ params_schema: UserGroupsParamsSchema }), getGroupMembersHandler) //get all members of a group
router.get('/:email/groups/:gid', validate({ params_schema: UserGroupsParamsSchema }), getGroupHandler) //get group owned by user
router.get('/:email/groups/:gid/liked', validate({ params_schema: UserGroupsParamsSchema })) //get most liked media from all lists in a group
router.get('/:email/followers', validate({ params_schema: UserParamsSchema }), getAllFollowersHandler) //get all followers of user
router.get('/:email/following', validate({ params_schema: UserParamsSchema }), getAllFollowingHandler) //get all users being followed by user
router.post('/', validate({ body_schema: CreateUserSchema }), createUserHandler) //create a new user
router.post('/:email/lists', validate({ params_schema: UserParamsSchema, body_schema: CreateListSchema }), createUserListHandler) //create a new list
router.post('/:email/groups', validate({ params_schema: UserParamsSchema, body_schema: CreateGroupSchema }), createUserGroupHandler) //create a new group
router.post('/:email/following', validate({ params_schema: UserParamsSchema, body_schema: FollowUserSchema }), createFollowingHandler) //follow a user
router.patch('/:email', validate({ params_schema: UserParamsSchema })) //update user
router.patch('/:email/lists', validate({ params_schema: UserParamsSchema })) //update list items
router.patch('/:email/groups/:gid', validate({ params_schema: UserGroupsParamsSchema })) //update group name
router.delete('/:email', validate({ params_schema: UserParamsSchema }), deleteUserHandler) //delete user
router.delete('/:email/lists/:name', deleteUserListHandler) //delete user's list
router.delete('/:email/groups/:gid', validate({ params_schema: UserParamsSchema })) //delete user's group
router.delete('/:email/groups/:gid/:email', validate({ params_schema: RemoveUserFromGroupsParamsSchema })) //remove a user from a group
router.delete('/:email/following/:email', validate({ params_schema: UnfollowUserSchema }), deleteUserFollowHandler) //unfollow a user being followed
router.delete('/:email/lists/:name/liked', validate({ params_schema: RemoveListLikeSchema }), deleteUserLikeHandler) //remove liked list

export default router
