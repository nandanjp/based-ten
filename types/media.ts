import { Type, type Static } from "@sinclair/typebox";

export const Media = Type.Object({
  id: Type.String(),
  title: Type.String(),
  image: Type.String(),
  creators: Type.Array(Type.String()),
  createdAt: Type.Date(),
});

export type MediaType = Static<typeof Media>;
