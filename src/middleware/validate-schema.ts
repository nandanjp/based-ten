import { NextFunction, Request, Response } from 'express'
import { ZodSchema } from 'zod'

type Schemas = {
  query_schema?: ZodSchema
  params_schema?: ZodSchema
  body_schema?: ZodSchema
}

export const validate = (schemas: Schemas) => async (req: Request, res: Response, next: NextFunction) => {
  if (schemas.query_schema && !schemas.query_schema.safeParse(req.query).success) {
    return next(new Error(schemas.query_schema.safeParse(req.query).error?.toString()))
  }
  if (schemas.params_schema && !schemas.params_schema.safeParse(req.params).success) {
    return next(new Error(schemas.params_schema.safeParse(req.params).error?.toString()))
  }
  if (schemas.body_schema && !schemas.body_schema.safeParse(req.body).success) {
    return next(new Error(schemas.body_schema.safeParse(req.body).error?.toString()))
  }
  next()
}
