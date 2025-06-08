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

type ReplicacheProviderProps = {
  userId: string;
  children: ReactNode;
};

const ReplicacheContext = createContext<Replicache<any> | null>(null);

const Mutators = {
  ...ChatMutators,
  ...MessageMutators,
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
    const eventSource = new EventSource("/api/sse");

    eventSource.addEventListener("replicache/poke", (event) => {
      try {
        const data = JSON.parse(event.data);
        console.log("Received poke event: ", data);
        rep.pull();
      } catch (error) {
        console.error("Error parsing SSE event data: ", error);
      }
    });

    eventSource.onopen = () => {
      console.log("SSE connection established");
    };

    eventSource.onerror = (error) => {
      console.error("SSE connection error: ", error);
    };

    return () => {
      eventSource.close();
    };
  }, [rep]);

  return (
    <ReplicacheContext.Provider value={rep}>
      {children}
    </ReplicacheContext.Provider>
  );
}
