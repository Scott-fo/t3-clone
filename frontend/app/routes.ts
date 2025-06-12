import {
  type RouteConfig,
  route,
  layout,
  index,
} from "@react-router/dev/routes";

export default [
  layout("layouts/protected.tsx", [
    layout("layouts/app.tsx", [
      index("routes/home.tsx"),
      route("chat/:thread_id", "routes/chat.tsx"),
      route("settings", "routes/settings.tsx"),
    ]),
  ]),

  layout("layouts/public.tsx", [
    route("login", "routes/login.tsx"),
    route("register", "routes/register.tsx"),
  ]),
] satisfies RouteConfig;
