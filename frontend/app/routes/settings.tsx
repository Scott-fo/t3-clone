import ProviderCard from "~/components/provider-card";
import { SidebarTrigger, useSidebar } from "~/components/ui/sidebar";
import { useApiKeys } from "~/hooks/use-api-keys";
import { cn } from "~/lib/utils";

export default function Page() {
  const sidebar = useSidebar();
  const { data, error } = useApiKeys();

  const openaiKey = data?.find((k) => k.provider === "openai");
  const googleKey = data?.find((k) => k.provider === "google");

  if (error) return <p className="text-red-600">{error.message}</p>;

  return (
    <div className="h-full max-h-screen max-w-3xl h-screen w-full mx-auto flex flex-col overflow-hidden">
      <SidebarTrigger
        className={cn(
          `absolute opacity-100 top-3 left-3 transition-opacity duration-200 z-0`,
          sidebar.open && "opacity-0"
        )}
      />
      <div className="flex min-h-screen flex-col items-center gap-8 pt-10">
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
      </div>
    </div>
  );
}
