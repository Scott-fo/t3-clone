import { type MutatorDefs, Replicache } from "replicache";
import {
  createContext,
  useContext,
  type ReactNode,
  useEffect,
  useMemo,
} from "react";
import { ChatMutators } from "~/domain/chat";
import { MessageMutators } from "~/domain/message";
import { useSSE } from "./SSEContext";
import { ActiveModelMutators } from "~/domain/active-model";

type ReplicacheProviderProps = {
  userId: string;
  children: ReactNode;
};

const ReplicacheContext = createContext<Replicache<any> | null>(null);

const Mutators = {
  ...ChatMutators,
  ...MessageMutators,
  ...ActiveModelMutators,
};

export type ReplicacheType = typeof Mutators & MutatorDefs;

export function useReplicache(): Replicache<ReplicacheType> {
  const context = useContext(ReplicacheContext);
  if (!context) {
    throw new Error("useReplicache must be used within ReplicacheProvider");
  }

  return context as Replicache<ReplicacheType>;
}

export function ReplicacheProvider({
  children,
  userId,
}: ReplicacheProviderProps) {
  const sse = useSSE();

  const rep = useMemo(() => {
    return new Replicache({
      name: `user-${userId}`,
      indexes: {
        messagesForChat: {
          prefix: "message/",
          jsonPointer: "/chat_id",
        },
      },
      licenseKey: import.meta.env.VITE_REPLICACHE_KEY,
      pushURL: "/api/replicache/push",
      pullURL: "/api/replicache/pull",
      mutators: Mutators,
      schemaVersion: "1",
    });
  }, [userId]);

  useEffect(() => {
    const handleReplicachePoke = () => {
      console.log("SSE poke received");
      rep.pull();
    };

    sse.addEventListener("replicache-poke", handleReplicachePoke);
    return () =>
      sse.removeEventListener("replicache-poke", handleReplicachePoke);
  }, [rep, sse]);

  return (
    <ReplicacheContext.Provider value={rep}>
      {children}
    </ReplicacheContext.Provider>
  );
}
