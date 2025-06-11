import type { User } from "~/domain/user";
import { api } from "~/lib/api";
import { useQuery } from "@tanstack/react-query";
import { AxiosError } from "axios";
import { useUserStore } from "~/stores/user";

export const useUser = () => {
  const setLoading = useUserStore((state) => state.setLoading);
  const setUser = useUserStore((state) => state.setData);
  const clear = useUserStore((state) => state.clear);

  return useQuery<User, AxiosError>({
    queryKey: ["user"],
    queryFn: async () => {
      const response = await api.get("/api/me");

      setUser(response.data);
      setLoading(false);

      return response.data;
    },

    retry: (_failureCount, error) => {
      if (error?.response?.status) {
        const status = error.response.status;

        if (status === 401 || status === 403) {
          clear();
          setLoading(false);
        }
      }

      return true;
    },
  });
};
