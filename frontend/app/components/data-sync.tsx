import { useReplicache } from "~/contexts/ReplicacheContext";
import { useChatStore } from "~/stores/chat";
import { useEffect } from "react";
import type { Replicache } from "replicache";
import type { ReplicacheType } from "~/contexts/ReplicacheContext";
import type { User } from "~/domain/user";
import { useAuth } from "~/contexts/AuthContext";
import { useActiveModelStore } from "~/stores/active-model";

type SyncConfig = (rep: Replicache<ReplicacheType>, user: User) => () => void;

const syncConfigs: SyncConfig[] = [
  (rep) => {
    const { sync, cleanup } = useChatStore.getState();
    sync(rep);
    return cleanup;
  },
  (rep) => {
    const { sync, cleanup } = useActiveModelStore.getState();
    sync(rep);
    return cleanup;
  },
];

export const DataSync = () => {
  const rep = useReplicache();
  const { user } = useAuth();

  useEffect(() => {
    if (!rep || !user) return;

    console.log(`Setting up ${syncConfigs.length} store sync processes...`);
    const allCleanups = syncConfigs.map((setup) => setup(rep, user));

    return () => {
      console.log("Cleaning up all store subscriptions...");
      allCleanups.forEach((cleanup) => cleanup());
    };
  }, [rep, user]);

  return null;
};
