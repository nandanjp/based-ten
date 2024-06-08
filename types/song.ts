import { Type, type Static } from "@sinclair/typebox";
import { Media } from "./media";

export const Song = Type.Intersect([
  Media,
  Type.Object({
    album: Type.String(),
  }),
]);
export type SongType = Static<typeof Song>;
