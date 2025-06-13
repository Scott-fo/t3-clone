import { memo } from "react";
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
} from "~/components/ui/sidebar";
import type { Chat } from "~/domain/chat";
import { ChatItem } from "./chat-item";
import { TooltipProvider } from "./ui/tooltip";
import { MAX_PINNED_CHATS } from "./app-sidebar";

interface NavMainProps {
  pinnedChats: Chat[];
  historyChats: Chat[];
  activeId: string;
  onPinChat: (id: string, pinned: boolean) => void;
  onDeleteChat: (id: string) => void;
}

export const NavMain = memo(
  ({
    pinnedChats,
    historyChats,
    activeId,
    onPinChat,
    onDeleteChat,
  }: NavMainProps) => {
    return (
      <div className="overflow-y-scroll no-scrollbar">
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
