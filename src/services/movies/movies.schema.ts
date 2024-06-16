import { z } from 'zod'

export const CreateMoviesSchema = z.object({
  title: z.string(),
  image: z.string().url(),
  creators: z.array(z.string())
})

export const FilterMoviesSchema = z.object({
  creator: z.string().optional()
})

export type CreateMovieType = z.infer<typeof CreateMoviesSchema>
export type FilterMovieType = z.infer<typeof FilterMoviesSchema>
