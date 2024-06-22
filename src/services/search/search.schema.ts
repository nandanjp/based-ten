import { z } from 'zod'

export const AnimeIdSearchSchema = z.object({
  id: z.string().refine((s) => !isNaN(parseInt(s)))
})

export const TopAnimeFilterSchema = z.object({
  type: z.enum(['tv', 'movie', 'ova', 'special', 'ona', 'music', 'cm', 'pv', 'tv_special']),
  filter: z.enum(['airing', 'upcoming', 'bypopularity', 'favorite']).optional(),
  rating: z.enum(['g', 'pg', 'pg13', 'r17', 'r']).optional(),
  limit: z.string().refine((s) => !isNaN(parseInt(s)))
})

export const GetAnimeFilterSchema = z.object({
  limit: z
    .string()
    .refine((s) => !isNaN(parseInt(s)))
    .refine((s) => parseInt(s) > 0 && parseInt(s) < 500)
    .default('20'),
  page: z
    .string()
    .refine((s) => !isNaN(parseInt(s)))
    .refine((s) => parseInt(s) > 0 && parseInt(s) < 20)
    .default('1'),
  type: z.enum(['tv', 'movie', 'ova', 'special', 'music', 'cm', 'pv', 'tv_special']).optional(),
  score: z
    .string()
    .refine((s) => !isNaN(parseInt(s)))
    .optional(),
  min_score: z
    .string()
    .refine((s) => !isNaN(parseInt(s)))
    .optional(),
  max_score: z
    .string()
    .refine((s) => !isNaN(parseInt(s)))
    .optional(),
  status: z.enum(['airing', 'complete', 'upcoming']),
  rating: z.enum(['g', 'pg', 'r17', 'r']),
  order_by: z
    .enum(['mal_id', 'title', 'start_date', 'end_date', 'episodes', 'score', 'scored_by', 'rank', 'popularity', 'members', 'favorites'])
    .default('score'),
  sort: z.enum(['desc', 'asc']).default('asc')
})

export type AnimeIdSearchType = z.infer<typeof AnimeIdSearchSchema>
export type GetTopAnimeType = z.infer<typeof TopAnimeFilterSchema>
export type GetAnimeType = z.infer<typeof GetAnimeFilterSchema>
