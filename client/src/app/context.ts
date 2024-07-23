import { createContext, Dispatch } from "react";
import { User } from "../../services/api.types";

type UserState = {
  user: User | undefined;
  setUser: Dispatch<User>;
};

export const UserContext = createContext<UserState>({
  user: undefined,
  setUser: () => {},
});
