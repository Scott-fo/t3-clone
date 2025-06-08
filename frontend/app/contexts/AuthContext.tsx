import { type User } from "~/domain/user";
import { useUser } from "~/hooks/user";
import { createContext, useContext, type ReactNode } from "react";

interface AuthContextType {
  user: User | null;
  loading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const { data: user, isLoading: loading } = useUser();

  return (
    <AuthContext.Provider value={{ user: user ?? null, loading }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
