import { useLocation } from "react-router";
import { memo, useMemo, useState } from "react";
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

export const NavMain = memo(({ items }: { items: Chat[] }) => {
  const rep = useReplicache();

  const location = useLocation();
  const [searchQuery, setSearchQuery] = useState("");

  const filteredChats = useMemo(() => {
    if (!searchQuery) return items;
    return items.filter((item) =>
      (item.title || "New chat")
        .toLowerCase()
        .includes(searchQuery.toLowerCase())
    );
  }, [items, searchQuery]);

  const pinnedChats = filteredChats.filter((item) => item.pinned);
  const historyChats = filteredChats.filter((item) => !item.pinned);

  const handlePinChat = (id: string, pinned: boolean) => {
    rep.mutate.updateChat({ id, pinned, updated_at: new Date().toISOString() });
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

      {pinnedChats.length > 0 && (
        <SidebarGroup>
          <SidebarGroupLabel>Pinned</SidebarGroupLabel>
          <SidebarMenu>{renderChatList(pinnedChats)}</SidebarMenu>
        </SidebarGroup>
      )}

      {historyChats.length > 0 && (
        <SidebarGroup>
          <SidebarGroupLabel>History</SidebarGroupLabel>
          <SidebarMenu>{renderChatList(historyChats)}</SidebarMenu>
        </SidebarGroup>
      )}
    </div>
  );
});
