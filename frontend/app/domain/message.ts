import type { ReadTransaction, WriteTransaction } from "replicache";

export type Message = {
  readonly id: string;
  readonly chat_id: string;
  readonly user_id: string;
  readonly body: string;
  readonly version: number;
  readonly created_at: number;
  readonly updated_at: number;
};

export const MessageMutators = {
  createMessage: async (tx: WriteTransaction, message: Message) => {
    await tx.set(`message/${message.id}`, message);
  },

  updateMessage: async (
    tx: WriteTransaction,
    {
      id,
      updated_at,
      ...updates
    }: Partial<Message> & { id: string; updated_at: number }
  ) => {
    const prev = await tx.get<Message>(`message/${id}`);
    const next = { ...prev, ...updates, updated_at } as Message;
    await tx.set(`message/${id}`, next);
  },

  deleteMessage: async (tx: WriteTransaction, { id }: { id: string }) => {
    await tx.del(`message/${id}`);
  },
};

export async function listMessagesForChat(
  tx: ReadTransaction,
  chatId: string | null
) {
  if (!chatId) {
    return [];
  }

  const data = await tx
    .scan<Message>({ indexName: "messagesForChat", prefix: chatId })
    .values()
    .toArray();

  return data.sort((a, b) => a.created_at - b.created_at);
}
