import { z } from 'zod'

export const UserParamsSchema = z.object({
  email: z.string().email()
})
export const UserGroupsParamsSchema = z.object({
  email: z.string().email(),
  gid: z.string().refine((g) => !isNaN(parseInt(g)))
})
export const RemoveUserFromGroupsParamsSchema = z.object({
  email: z.string().email(),
  gid: z
    .string()
    .refine((g) => !isNaN(parseInt(g)))
    .transform((g) => parseInt(g)),
  following_email: z.string().email()
})
export const UserListSchema = z.object({
  email: z.string().email(),
  list_name: z.string()
})

// Creation Schemas
export const CreateUserSchema = z.object({
  email: z.string().email(),
  display_name: z.string().min(4).max(20),
  user_password: z.string().min(6).max(15)
})
export const CreateListSchema = z.object({
  list_name: z.string().min(4).max(30),
  list_type: z.enum(['anime', 'movies', 'songs', 'videogames']),
  items: z
    .array(
      z.object({
        ranking_in_list: z
          .string()
          .refine((r) => !isNaN(parseInt(r)))
          .transform((r) => parseInt(r))
          .refine((r) => r > 0 && r < 11),
        item_id: z.string()
      })
    )
    .length(10)
})
export const CreateGroupSchema = z.object({
  group_name: z.string().min(4).max(30)
})
export const FollowUserSchema = z.object({
  following_email: z.string().email()
})

// Updating Schemas
export const UpdateUserSchema = z.object({
  display_name: z.string().min(4).max(20).optional(),
  user_password: z.string().min(6).max(15).optional()
})
export const UpdateListSchema = z.object({
  list_name: z.string().min(4).max(30).optional(),
  items: z
    .array(
      z.object({
        ranking_in_list: z
          .string()
          .refine((r) => !isNaN(parseInt(r)))
          .refine((r) => parseInt(r) > 0 && parseInt(r) < 11),
        item_id: z.string()
      })
    )
    .optional()
})
export const UpdateGroupSchema = z.object({
  group_name: z.string().min(4).max(30)
})

export const UnfollowUserSchema = z.object({
  follower_email: z.string().email(),
  following_email: z.string().email()
})
export const RemoveListLikeSchema = z.object({
  liker_email: z.string().email(),
  liking_email: z.string().email(),
  list_name: z.string()
})

export type UserParamsType = z.infer<typeof UserParamsSchema>
export type UserGroupsParamsType = z.infer<typeof UserGroupsParamsSchema>
export type RemoveUserFromGroupParamsType = z.infer<typeof RemoveUserFromGroupsParamsSchema>
export type UserListType = z.infer<typeof UserListSchema>
export type CreateUserType = z.infer<typeof CreateUserSchema>
export type CreateListType = z.infer<typeof CreateListSchema>
export type CreateGroupType = z.infer<typeof CreateGroupSchema>
export type FollowUserSchema = z.infer<typeof FollowUserSchema>
export type UpdateUserType = z.infer<typeof UpdateUserSchema>
export type UpdateListType = z.infer<typeof UpdateListSchema>
export type UnfollowUserType = z.infer<typeof UnfollowUserSchema>
export type RemoveListLikeType = z.infer<typeof RemoveListLikeSchema>
