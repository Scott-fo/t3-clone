import { create } from "zustand";
import { persist } from "zustand/middleware";

type Preferences = {
  wrapText: boolean;
  showReasoning: boolean;
};

interface State {
  data: Preferences;
  setData: (p: Preferences) => void;
}

export const usePreferencesStore = create<State>()(
  persist(
    (set) => ({
      data: { wrapText: false, showReasoning: false },
      setData: (data) => set({ data }),
    }),
    {
      name: "preferences-storage",
    }
  )
);
