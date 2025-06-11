import type { ReadTransaction, WriteTransaction } from "replicache";
import type { Message } from "./message";

// fix this type.
export type Chat = {
  readonly id: string;
  readonly user_id: string;
  readonly title?: string | null;
  readonly pinned?: boolean;
  readonly archived?: boolean;
  readonly forked: boolean;
  readonly version: number;
  readonly created_at: string;
  readonly updated_at: string;
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
    }: Partial<Chat> & { id: string; updated_at: string }
  ) => {
    const prev = await tx.get<Chat>(`chat/${id}`);
    const next = { ...prev, ...updates, updated_at } as Chat;
    await tx.set(`chat/${id}`, next);
  },

  deleteChat: async (tx: WriteTransaction, { id }: { id: string }) => {
    await tx.del(`chat/${id}`);
  },

  forkChat: async (
    tx: WriteTransaction,
    {
      new_id,
      title,
      time,
      msgs,
    }: { new_id: string; title: string; time: string; msgs: Message[] }
  ) => {
    const new_chat = {
      id: new_id,
      title,
      forked: true,
      updated_at: time,
      created_at: time,
    } as Chat;

    await tx.set(`chat/${new_chat.id}`, new_chat);

    for (const msg of msgs) {
      await tx.set(`message/${msg.id}`, msg);
    }
  },
};

export async function listChats(tx: ReadTransaction) {
  const data = await tx.scan<Chat>({ prefix: "chat/" }).values().toArray();
  return data.sort(
    (a, b) =>
      new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
  );
}
