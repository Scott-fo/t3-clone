export type SharedChat = {
  id: string;
  title?: string | null;
  createdAt: string;
};

export type SharedMessage = {
  id: string;
  role: string;
  body: string;
  reasoning?: string | null;
  createdAt: string;
};

export interface SharedChatWithMessages extends SharedChat {
  messages: SharedMessage[];
}
