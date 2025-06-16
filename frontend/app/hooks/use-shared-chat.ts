import { api } from "~/lib/api";
import { type SharedChatWithMessages } from "~/domain/shared_chat";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

export async function fetchSharedChat(
  id: string
): Promise<SharedChatWithMessages> {
  const { data } = await api.get<SharedChatWithMessages>(`/api/shared/${id}`);
  return data;
}

export async function createShare(
  chatId: string
): Promise<SharedChatWithMessages> {
  const { data } = await api.post<SharedChatWithMessages>(
    `/api/chats/${chatId}/share`
  );
  return data;
}

export async function deleteShare(id: string): Promise<void> {
  await api.delete(`/api/shared/${id}`);
}

const queryKeys = {
  sharedChat: (id: string) => ["shared-chat", id] as const,
};

// Public read – no auth required
export function useSharedChat(id: string) {
  return useQuery<SharedChatWithMessages>({
    queryKey: queryKeys.sharedChat(id),
    queryFn: () => fetchSharedChat(id),
    enabled: !!id,
  });
}

// POST /api/chats/{chatId}/share
export function useCreateShare() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (chatId: string) => createShare(chatId),
    onSuccess: (snapshot) => {
      qc.setQueryData(queryKeys.sharedChat(snapshot.id), snapshot);
    },
  });
}

// DELETE /api/shared/{id}
export function useDeleteShare() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => deleteShare(id),
    onSuccess: (_void, id) => {
      qc.removeQueries({ queryKey: queryKeys.sharedChat(id) });
    },
  });
}
