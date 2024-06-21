import { z } from 'zod'

export const CreateSongsSchema = z.object({
  title: z.string(),
  image: z.string().url(),
  creators: z.array(z.string()),
  album: z.string()
})

export const FilterSongsSchema = z.object({
  album: z.string().optional(),
  creator: z.string().optional()
})

export type CreateSongType = z.infer<typeof CreateSongsSchema>
export type FilterSongType = z.infer<typeof FilterSongsSchema>
