import { useLocation, useNavigate } from "react-router";
import { memo, useMemo, useState, useEffect } from "react";
import { Search } from "lucide-react";
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
} from "~/components/ui/sidebar";
import { Input } from "~/components/ui/input";
import type { Chat } from "~/domain/chat";
import { ChatItem } from "./chat-item";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { TooltipProvider } from "./ui/tooltip";
import { href } from "react-router";
import { nanoid } from "nanoid";

const MAX_PINNED_CHATS = 5;

export const NavMain = memo(({ items }: { items: Chat[] }) => {
  const rep = useReplicache();
  const location = useLocation();
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");

  const filteredChats = useMemo(() => {
    if (!searchQuery) return items;
    return items.filter((item) =>
      (item.title || "New chat")
        .toLowerCase()
        .includes(searchQuery.toLowerCase())
    );
  }, [items, searchQuery]);

  const pinnedChats = filteredChats
    .filter((item) => item.pinned)
    .sort(
      (a, b) =>
        new Date(a.pinned_at!).getTime() - new Date(b.pinned_at!).getTime()
    );
  const historyChats = filteredChats.filter((item) => !item.pinned);

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
  }, [pinnedChats, navigate]);

  const handlePinChat = (id: string, pinned: boolean) => {
    if (pinned && pinnedChats.length >= MAX_PINNED_CHATS) {
      // add sonner
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

  const renderChatList = (chatList: Chat[]) => {
    return chatList.map((item) => {
      const isActive = location.pathname.includes(item.id);
      return (
        <ChatItem
          key={item.id}
          item={item}
          isActive={isActive}
          onPin={handlePinChat}
          onDelete={handleDeleteChat}
        />
      );
    });
  };

  return (
    <div className="gap-y-2">
      <div className="relative mt-1 border-b">
        <Search className="absolute left-4 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder="Search chats..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="pl-10 border-0 shadow-none focus-visible:ring-0 focus-visible:ring-offset-0"
        />
      </div>

      <TooltipProvider delayDuration={1000} skipDelayDuration={200}>
        {pinnedChats.length > 0 && (
          <SidebarGroup>
            <SidebarGroupLabel>Hotbar</SidebarGroupLabel>
            <SidebarMenu>
              {pinnedChats.slice(0, MAX_PINNED_CHATS).map((item, index) => {
                const isActive = location.pathname.includes(item.id);
                return (
                  <ChatItem
                    key={item.id}
                    item={item}
                    isActive={isActive}
                    onPin={handlePinChat}
                    onDelete={handleDeleteChat}
                    pinIndex={index + 1}
                  />
                );
              })}
            </SidebarMenu>
          </SidebarGroup>
        )}

        {historyChats.length > 0 && (
          <SidebarGroup>
            <SidebarGroupLabel>History</SidebarGroupLabel>
            <SidebarMenu>{renderChatList(historyChats)}</SidebarMenu>
          </SidebarGroup>
        )}
      </TooltipProvider>
    </div>
  );
});
