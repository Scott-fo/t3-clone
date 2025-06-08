import { href, Link, useLocation } from "react-router";
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "~/components/ui/sidebar";
import type { Chat } from "~/domain/chat";

export function NavMain({ items }: { items: Chat[] }) {
  const location = useLocation();

  return (
    <SidebarGroup>
      <SidebarGroupLabel>History</SidebarGroupLabel>
      <SidebarMenu>
        {items.map((item) => {
          const isActive = location.pathname.includes(item.id);
          return (
            <SidebarMenuItem key={item.id}>
              <SidebarMenuButton isActive={isActive} asChild>
                <Link to={href("/chat/:thread_id", { thread_id: item.id })}>
                  <span className="truncate">{item.title || "New chat"}</span>
                </Link>
              </SidebarMenuButton>
            </SidebarMenuItem>
          );
        })}
      </SidebarMenu>
    </SidebarGroup>
  );
}
