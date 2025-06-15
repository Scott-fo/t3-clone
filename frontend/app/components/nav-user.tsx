import { KeyIcon } from "@heroicons/react/24/outline";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { ChevronsUpDown, LogOut, Settings } from "lucide-react";
import { href, useNavigate } from "react-router";
import { dropAllDatabases } from "replicache";

import { Avatar, AvatarFallback } from "~/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "~/components/ui/sidebar";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { api } from "~/lib/api";
import { useUserStore } from "~/stores/user";

export function NavUser({
  user,
}: {
  user: {
    email: string;
  };
}) {
  const { isMobile } = useSidebar();
  const queryClient = useQueryClient();
  const rep = useReplicache();
  const navigate = useNavigate();

  const clear = useUserStore((state) => state.clear);

  const logoutMutation = useMutation({
    mutationFn: () => api.post("/api/logout"),
    onSuccess: () => {
      queryClient.clear();
      rep.close();
      dropAllDatabases();
      clear();
      navigate("/login", { replace: true });
    },
  });

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <Avatar className="h-8 w-8 rounded-lg">
                <AvatarFallback className="rounded-lg">
                  {user?.email?.slice(0, 2).toUpperCase()}
                </AvatarFallback>
              </Avatar>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate text-xs">{user.email}</span>
              </div>
              <ChevronsUpDown className="ml-auto size-4" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
            side={isMobile ? "bottom" : "right"}
            align="end"
            sideOffset={4}
          >
            <DropdownMenuGroup>
              <DropdownMenuItem onClick={() => navigate(href("/settings"))}>
                <KeyIcon />
                API Keys
              </DropdownMenuItem>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => logoutMutation.mutate()}>
              <LogOut />
              Log out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
