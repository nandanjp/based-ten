import { z } from 'zod'

const ParamsSchema = z.object({
  id: z.string()
})

export type ParamsType = z.infer<typeof ParamsSchema>
