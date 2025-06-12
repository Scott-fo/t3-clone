import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "../lib/api";
import type { ApiKey, CreateApiKeyPayload } from "~/domain/api-key";

function handle<T>(p: Promise<{ data: T }>): Promise<T> {
  return p
    .then((r) => r.data)
    .catch((e) => {
      /* Normalise axios errors so TanStack receives plain Error */
      const msg =
        e?.response?.data ??
        e?.message ??
        "Unknown error while contacting server";
      throw new Error(msg);
    });
}

export const apiKeys = {
  list: () => handle<ApiKey[]>(api.get("/api/api-keys")),

  create: (payload: CreateApiKeyPayload) =>
    handle<ApiKey>(api.post("/api/api-keys", payload)),

  remove: (id: number) => handle<void>(api.delete(`/api/api-keys/${id}`)),
};

const Keys = {
  all: ["apiKeys"] as const,
};

export function useApiKeys() {
  return useQuery<ApiKey[], Error>({
    queryKey: Keys.all,
    queryFn: apiKeys.list,
    staleTime: Infinity,
  });
}

export function useCreateApiKey() {
  const qc = useQueryClient();
  return useMutation<ApiKey, Error, CreateApiKeyPayload>({
    mutationFn: apiKeys.create,
    onSuccess: () => qc.invalidateQueries({ queryKey: Keys.all }),
  });
}

export function useDeleteApiKey() {
  const qc = useQueryClient();
  return useMutation<void, Error, number>({
    mutationFn: apiKeys.remove,
    onSuccess: () => qc.invalidateQueries({ queryKey: Keys.all }),
  });
}
