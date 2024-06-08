import { Type, type Static } from "@sinclair/typebox";
import { Media } from "./media";

export const Game = Type.Intersect([
  Media,
  Type.Object({
    console: Type.Union([
      Type.Literal("Atari 5200"),
      Type.Literal("Atari 7800"),
      Type.Literal("DS"),
      Type.Literal("Dreamcast"),
      Type.Literal("Famicon"),
      Type.Literal("Game Boy Advanced"),
      Type.Literal("Game Cube"),
      Type.Literal("Nintendo 64"),
      Type.Literal("NES"),
      Type.Literal("SNES"),
      Type.Literal("PS1"),
      Type.Literal("PS2"),
      Type.Literal("PS3"),
      Type.Literal("PS4"),
      Type.Literal("PS5"),
      Type.Literal("Sega Genesis"),
      Type.Literal("Steam"),
      Type.Literal("Nintendo Switch"),
      Type.Literal("Virtual Boy"),
      Type.Literal("Nintendo Wii"),
      Type.Literal("Nintendo WiiU"),
      Type.Literal("Xbox 360"),
      Type.Literal("Xbox Series X"),
    ]),
  }),
]);

export type GameType = Static<typeof Game>;
