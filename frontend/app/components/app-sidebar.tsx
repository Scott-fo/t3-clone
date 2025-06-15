import * as React from "react";
import { useMemo, useState, useEffect } from "react";
import { Search } from "lucide-react";
import { Link, useNavigate, useParams } from "react-router";
import { nanoid } from "nanoid";
import { href } from "react-router";

import { NavMain } from "~/components/nav-main";
import { NavUser } from "~/components/nav-user";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarTrigger,
} from "~/components/ui/sidebar";
import { Button } from "./ui/button";
import { useChatStore } from "~/stores/chat";
import { useUserStore } from "~/stores/user";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { toast } from "sonner";
import { CommandMenu } from "./chat-menu";
import { Input } from "./ui/input";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";
import { useConnectedProviderStore } from "~/stores/connected-provider";

export const MAX_PINNED_CHATS = 9;

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const navigate = useNavigate();
  const rep = useReplicache();
  const activeId = useParams()?.thread_id ?? "";

  const connectedProviders = useConnectedProviderStore((state) => state.data);
  const areConnectedProvidersLoading = useConnectedProviderStore(
    (state) => state.loading
  );

  const allChats = useChatStore((state) => state.data);
  const user = useUserStore((state) => state.data);

  const currentChat = allChats.find((c) => c.id === activeId) ?? null;

  const [searchQuery, setSearchQuery] = useState("");
  const [commandOpen, setCommandOpen] = useState(false);
  const [isApiKeysDialogOpen, setIsApiKeysDialogOpen] = useState(false);

  useEffect(() => {
    if (
      !areConnectedProvidersLoading &&
      (!connectedProviders || connectedProviders.length === 0)
    ) {
      setIsApiKeysDialogOpen(true);
    } else {
      setIsApiKeysDialogOpen(false);
    }
  }, [connectedProviders, areConnectedProvidersLoading]);

  const visibleChats = useMemo(
    () => allChats.filter((chat) => !chat.archived),
    [allChats]
  );

  const filteredChats = useMemo(() => {
    if (!searchQuery) return visibleChats;
    return visibleChats.filter((item) =>
      (item.title || "New chat")
        .toLowerCase()
        .includes(searchQuery.toLowerCase())
    );
  }, [visibleChats, searchQuery]);

  const pinnedChats = useMemo(
    () =>
      filteredChats
        .filter((item) => item.pinned)
        .sort(
          (a, b) =>
            new Date(a.pinned_at!).getTime() - new Date(b.pinned_at!).getTime()
        ),
    [filteredChats]
  );

  const historyChats = useMemo(
    () => filteredChats.filter((item) => !item.pinned),
    [filteredChats]
  );

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (!event.metaKey && !event.ctrlKey) {
        return;
      }

      switch (event.key) {
        case "s": {
          event.preventDefault();
          navigate(href("/chat/:thread_id", { thread_id: nanoid() }));
          break;
        }
        case "h": {
          event.preventDefault();
          if (currentChat) {
            handlePinChat(activeId, !currentChat.pinned);
          }
          break;
        }
        case "r": {
          event.preventDefault();
          setCommandOpen((prev) => !prev);
          break;
        }
        case "1":
        case "2":
        case "3":
        case "4":
        case "5": {
          event.preventDefault();
          const keyNumber = parseInt(event.key, 10);
          const chatToNavigate = pinnedChats[keyNumber - 1];
          if (chatToNavigate) {
            navigate(`/chat/${chatToNavigate.id}`);
          }
          break;
        }
        default:
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [pinnedChats, activeId, navigate]);

  const handlePinChat = (id: string, pinned: boolean) => {
    if (pinned && pinnedChats.length >= MAX_PINNED_CHATS) {
      toast("Hotbar limit reached", {
        description: `You can only add ${MAX_PINNED_CHATS} chats to hotbar at once`,
      });
      return;
    }
    rep.mutate.updateChat({
      id,
      pinned,
      updated_at: new Date().toISOString(),
      pinned_at: new Date().toISOString(),
    });
  };

  const handleDeleteChat = (id: string) => {
    rep.mutate.deleteChat({ id });
  };

  return (
    <Sidebar variant="inset" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem className="flex items-center">
            <SidebarTrigger className="absolute" />
            <SidebarMenuButton asChild>
              <Link to="/">
                <div className="grid flex-1 text-lg leading-tight">
                  <span className="text-center truncate font-mono">
                    Open Chat
                  </span>
                </div>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <Button
          size="lg"
          className="mx-2"
          onClick={() => navigate(`/chat/${nanoid()}`)}
        >
          New Chat
        </Button>
        <div className="relative mt-1 border-b">
          <Search className="absolute left-4 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search chats..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10 border-0 shadow-none focus-visible:ring-0 focus-visible:ring-offset-0"
          />
        </div>
        <NavMain
          pinnedChats={pinnedChats}
          historyChats={historyChats}
          activeId={activeId}
          onPinChat={handlePinChat}
          onDeleteChat={handleDeleteChat}
        />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={user!} />
      </SidebarFooter>
      <CommandMenu
        open={commandOpen}
        setOpen={setCommandOpen}
        chats={visibleChats}
      />
      <Dialog open={isApiKeysDialogOpen} onOpenChange={setIsApiKeysDialogOpen}>
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>API Key Required</DialogTitle>
            <DialogDescription>
              An API key is required to use Open Chat. Please add your API key
              in the settings.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button
              onClick={() => {
                setIsApiKeysDialogOpen(false);
                navigate(href("/settings"));
              }}
            >
              Go to Settings
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Sidebar>
  );
}
