import type { User } from "~/domain/user";
import { api } from "~/lib/api";
import { useQuery } from "@tanstack/react-query";
import { AxiosError } from "axios";

export const useUser = () => {
  return useQuery<User, AxiosError>({
    queryKey: ["user"],
    queryFn: async () => {
      const response = await api.get("/api/me");
      return response.data;
    },

    retry: (_failureCount, error) => {
      if (error?.response?.status) {
        const status = error.response.status;

        if (status === 401 || status === 403) {
          return false;
        }
      }

      return true;
    },
  });
};
