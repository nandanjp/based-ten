import { z } from 'zod'

export const RetrieveAnimeSchema = z.object({
  mal_id: z.number(),
  url: z.string(),
  images: z.array(z.string()),
  title: z.string(),
  title_english: z.string(),
  title_japanese: z.string(),
  type: z.string(),
  episodes: z.number(),
  status: z.string(),
  rating: z.string(),
  score: z.number(),
  rank: z.number(),
  synopsis: z.string(),
  year: z.number()
})

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

export type RetrieveAnimeType = z.infer<typeof RetrieveAnimeSchema>
export type CreateAnimeType = z.infer<typeof CreateAnimeSchema>
export type FilterAnimeType = z.infer<typeof FilterAnimeSchema>

//https://api.jikan.moe/v4/anime/55791/full
