import ProviderCard from "~/components/provider-card";
import { SidebarTrigger, useSidebar } from "~/components/ui/sidebar";
import { useApiKeys } from "~/hooks/use-api-keys";
import { cn } from "~/lib/utils";

export default function Page() {
  const sidebar = useSidebar();
  const { data, error } = useApiKeys();

  const openaiKey = data?.find((k) => k.provider === "openai");
  if (error) return <p className="text-red-600">{error.message}</p>;

  return (
    <div className="h-full max-h-screen h-screen w-full mx-auto flex flex-col overflow-hidden">
      <SidebarTrigger
        className={cn(
          `absolute opacity-100 top-3 left-3 transition-opacity duration-200 z-0`,
          sidebar.open && "opacity-0"
        )}
      />
      <div className="flex min-h-screen flex-col items-center gap-8 pt-10">
        <ProviderCard provider="openai" existingKey={openaiKey} />
      </div>
    </div>
  );
}
