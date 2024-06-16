import { z } from 'zod'

export const CreateAnimeSchema = z.object({
  title: z.string(),
  image: z.string().url(),
  creators: z.array(z.string()),
  episodes: z.number().min(1)
})

export const FilterAnimeSchema = z.object({
  // episodes: z.preprocess((val) => Number(val), z.number()).optional(),
  episodes: z.string().optional(),
  creator: z.string().optional()
})

export type CreateAnimeType = z.infer<typeof CreateAnimeSchema>
export type FilterAnimeType = z.infer<typeof FilterAnimeSchema>
