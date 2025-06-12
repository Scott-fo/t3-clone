import type { ReadTransaction, WriteTransaction } from "replicache";
import type { Reasoning } from "./ai";

export type ActiveModel = {
  readonly id: string;
  readonly provider: string;
  readonly model: string;
  readonly reasoning: Reasoning | null;
  readonly created_at: string;
  readonly updated_at: string;
};

export const ActiveModelMutators = {
  createActiveModel: async (tx: WriteTransaction, activeModel: ActiveModel) => {
    await tx.set(`activeModel/${activeModel.id}`, activeModel);
  },

  updateActiveModel: async (
    tx: WriteTransaction,
    {
      id,
      updated_at,
      ...updates
    }: Partial<ActiveModel> & { id: string; updated_at: string }
  ) => {
    const prev = await tx.get<ActiveModel>(`activeModel/${id}`);
    const next = { ...prev, ...updates, updated_at } as ActiveModel;
    await tx.set(`activeModel/${id}`, next);
  },

  deleteActiveModel: async (tx: WriteTransaction, { id }: { id: string }) => {
    await tx.del(`activeModel/${id}`);
  },
};

export async function getActiveModel(tx: ReadTransaction) {
  const data = await tx
    .scan<ActiveModel>({ prefix: "activeModel/" })
    .values()
    .toArray();

  if (data.length > 0) {
    return data[0];
  }

  return null;
}
