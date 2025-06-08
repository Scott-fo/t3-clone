import type { ReadTransaction, WriteTransaction } from "replicache";

export type Chat = {
  readonly id: string;
  readonly user_id: string;
  readonly tite: string | null;
  readonly archived: boolean;
  readonly version: number;
  readonly created_at: number;
  readonly updated_at: number;
};

export const ChatMutators = {
  createChat: async (tx: WriteTransaction, chat: Chat) => {
    await tx.set(`chat/${chat.id}`, chat);
  },

  updateChat: async (
    tx: WriteTransaction,
    {
      id,
      updated_at,
      ...updates
    }: Partial<Chat> & { id: string; updated_at: number }
  ) => {
    const prev = await tx.get<Chat>(`chat/${id}`);
    const next = { ...prev, ...updates, updated_at } as Chat;
    await tx.set(`chat/${id}`, next);
  },

  deleteChat: async (tx: WriteTransaction, { id }: { id: string }) => {
    await tx.del(`chat/${id}`);
  },
};

export async function listChats(tx: ReadTransaction) {
  const data = await tx.scan<Chat>({ prefix: "chat/" }).values().toArray();
  return data.sort((a, b) => a.updated_at! - b.updated_at!);
}
