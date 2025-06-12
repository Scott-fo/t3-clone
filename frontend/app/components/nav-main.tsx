import { memo } from "react";
import { Search } from "lucide-react";
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
} from "~/components/ui/sidebar";
import { Input } from "~/components/ui/input";
import type { Chat } from "~/domain/chat";
import { ChatItem } from "./chat-item";
import { TooltipProvider } from "./ui/tooltip";
import { MAX_PINNED_CHATS } from "./app-sidebar";

interface NavMainProps {
  pinnedChats: Chat[];
  historyChats: Chat[];
  searchQuery: string;
  activeId: string;
  onSearchQueryChange: (query: string) => void;
  onPinChat: (id: string, pinned: boolean) => void;
  onDeleteChat: (id: string) => void;
}

export const NavMain = memo(
  ({
    pinnedChats,
    historyChats,
    searchQuery,
    activeId,
    onSearchQueryChange,
    onPinChat,
    onDeleteChat,
  }: NavMainProps) => {
    return (
      <div className="gap-y-2">
        <div className="relative mt-1 border-b">
          <Search className="absolute left-4 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search chats..."
            value={searchQuery}
            onChange={(e) => onSearchQueryChange(e.target.value)}
            className="pl-10 border-0 shadow-none focus-visible:ring-0 focus-visible:ring-offset-0"
          />
        </div>

        <TooltipProvider delayDuration={500} skipDelayDuration={200}>
          {pinnedChats.length > 0 && (
            <SidebarGroup>
              <SidebarGroupLabel>Hotbar</SidebarGroupLabel>
              <SidebarMenu>
                {pinnedChats.slice(0, MAX_PINNED_CHATS).map((item, index) => {
                  return (
                    <ChatItem
                      key={item.id}
                      item={item}
                      isActive={item.id === activeId}
                      onPin={onPinChat}
                      onDelete={onDeleteChat}
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
              <SidebarMenu>
                {historyChats.map((item) => {
                  return (
                    <ChatItem
                      key={item.id}
                      item={item}
                      isActive={item.id === activeId}
                      onPin={onPinChat}
                      onDelete={onDeleteChat}
                    />
                  );
                })}
              </SidebarMenu>
            </SidebarGroup>
          )}
        </TooltipProvider>
      </div>
    );
  }
);
