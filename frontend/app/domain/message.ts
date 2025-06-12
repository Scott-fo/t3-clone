import type { ReadTransaction, WriteTransaction } from "replicache";

type MessageRole = "user" | "assistant";

export type Message = {
  readonly id: string;
  readonly chat_id: string;
  readonly role: MessageRole;
  readonly body: string;
  readonly reasoning?: string | null;
  readonly created_at: string;
  readonly updated_at: string;
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
    }: Partial<Message> & { id: string; updated_at: string }
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

  return data.sort(
    (a, b) =>
      new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
  );
}
