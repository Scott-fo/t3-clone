import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { User } from "~/domain/user";

interface State {
  data: User | null;
  loading: boolean;

  setLoading: (l: boolean) => void;
  setData: (u: User | null) => void;

  clear: () => void;
}

export const useUserStore = create<State>()(
  persist(
    (set) => ({
      data: null,
      loading: true,

      setLoading: (loading) => set({ loading }),
      setData: (data) => set({ data, loading: false }),

      clear: () => set({ data: null, loading: false }),
    }),
    {
      name: "user-storage",
    }
  )
);
