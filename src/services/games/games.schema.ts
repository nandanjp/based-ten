import { z } from 'zod'

const platform = z.union([
  z.literal('NINTENDO_SWITCH'),
  z.literal('NINTENDO_WIIU'),
  z.literal('NINTENDO_WII'),
  z.literal('NINTENDO_GAMECUBE'),
  z.literal('NINTENDO_64'),
  z.literal('PS5'),
  z.literal('PS4'),
  z.literal('PS3'),
  z.literal('PS2'),
  z.literal('XBOX_X'),
  z.literal('XBOX_S'),
  z.literal('XBOX_ONE'),
  z.literal('XBOX')
])

export const GetVideoGameSchema = z.object({
  id: z.number(),
  slug: z.string(),
  name: z.string(),
  released: z.string().date(),
  background_image: z.string(),
  rating: z.number(),
  metacritic: z.number()
})

export const CreateVideoGameSchema = z.object({
  title: z.string(),
  image: z.string().url(),
  creators: z.array(z.string()),
  platform
})

export const FilterVideoGameSchema = z.object({
  creator: z.string().optional(),
  platform: platform.optional()
})

export type CreateVideoGameType = z.infer<typeof CreateVideoGameSchema>
export type FilterVideoGameType = z.infer<typeof FilterVideoGameSchema>
export type Platforms = z.infer<typeof platform>

//https://api.rawg.io/api/games?key=2153728e163f4fb4866e0df8ca39d168
