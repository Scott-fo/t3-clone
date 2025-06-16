import ProviderCard from "~/components/provider-card";
import { SidebarTrigger, useSidebar } from "~/components/ui/sidebar";
import { cn } from "~/lib/utils";
import { useConnectedProviderStore } from "~/stores/connected-provider";

export default function Page() {
  const sidebar = useSidebar();
  const connectedProviders = useConnectedProviderStore((state) => state.data);

  const openaiKey = connectedProviders?.find((k) => k.provider === "openai");
  const googleKey = connectedProviders?.find((k) => k.provider === "google");
  const anthropicKey = connectedProviders?.find(
    (k) => k.provider === "anthropic"
  );
  const openRouterKey = connectedProviders?.find(
    (k) => k.provider === "openrouter"
  );

  return (
    <div className="max-w-3xl w-full mx-auto flex flex-col px-6 pb-6">
      <SidebarTrigger
        className={cn(
          `absolute opacity-100 top-3 left-3 transition-opacity duration-200 z-0`,
          sidebar.open && "opacity-0"
        )}
      />
      <div className="flex flex-col items-center gap-8 pt-10">
        <div className="flex w-full flex-col items-start">
          <h1 className="text-2xl font-bold">API Keys</h1>
          <p className="text-muted-foreground">
            Add API keys for each provider. These are encrypted and stored
            server side only.
          </p>
        </div>
        <ProviderCard
          provider="openai"
          existingKey={openaiKey}
          placeholder="sk-..."
        />
        <ProviderCard
          provider="google"
          existingKey={googleKey}
          placeholder="AI..."
        />
        <ProviderCard
          provider="anthropic"
          existingKey={anthropicKey}
          placeholder="sk-..."
        />
        <ProviderCard
          provider="openrouter"
          existingKey={openRouterKey}
          placeholder="sk-..."
        />
      </div>
    </div>
  );
}
