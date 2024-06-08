import "dotenv/config";
import { z } from "zod";

const envSchema = z.object({
  PORT: z.number().int().positive().default(3000),
  POSTGRES_URL: z.string(),
  POSTGRES_PORT: z.number().int().positive().default(5432),
  POSTGRES_URL_NO_SSL: z.string(),
  POSTGRES_URL_NON_POOLING: z.string(),
  POSTGRES_USER: z.string().default("default"),
  POSTGRES_HOST: z.string(),
  POSTGRES_PASSWORD: z.string(),
  POSTGRES_DATABASE: z.string(),
});

const parsedEnv = envSchema.safeParse(process.env);
if (!parsedEnv.success) {
  console.error(
    `Invalid environment variables passed: ${JSON.stringify(
      parsedEnv.error.format(),
      null,
      4
    )}`
  );
}

export default parsedEnv.data;
