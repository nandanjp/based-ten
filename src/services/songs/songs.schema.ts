import { z } from 'zod'

export const CreateSongsSchema = z.object({
  title: z.string(),
  image: z.string().url(),
  creators: z.array(z.string()),
  album: z.string()
})
