import * as React from "react";
import { useMemo, useState, useEffect } from "react";
import { Command } from "lucide-react";
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
} from "~/components/ui/sidebar";
import { Button } from "./ui/button";
import { useChatStore } from "~/stores/chat";
import { useUserStore } from "~/stores/user";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { toast } from "sonner";
import { CommandMenu } from "./chat-menu";

export const MAX_PINNED_CHATS = 9;

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const navigate = useNavigate();
  const rep = useReplicache();
  const activeId = useParams()?.thread_id ?? "";

  const allChats = useChatStore((state) => state.data);
  const user = useUserStore((state) => state.data);

  const currentChat = allChats.find((c) => c.id === activeId) ?? null;

  const [searchQuery, setSearchQuery] = useState("");
  const [commandOpen, setCommandOpen] = useState(false);

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
        case "c": {
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
        case "s": {
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
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <Link to="/">
                <div className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                  <Command className="size-4" />
                </div>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-medium">Acme Inc</span>
                  <span className="truncate text-xs">Enterprise</span>
                </div>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <Button size="lg" onClick={() => navigate(`/chat/${nanoid()}`)}>
          New Chat
        </Button>
        <NavMain
          pinnedChats={pinnedChats}
          historyChats={historyChats}
          searchQuery={searchQuery}
          activeId={activeId}
          onSearchQueryChange={setSearchQuery}
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
    </Sidebar>
  );
}
