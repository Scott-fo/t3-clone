export type ApiKey = {
  id: number;
  provider: string;
  createdAt: string;
  updatedAt: string;
};

export type CreateApiKeyPayload = {
  provider: string;
  key: string;
};
