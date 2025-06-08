import {
  type RouteConfig,
  index,
  route,
  layout,
} from "@react-router/dev/routes";

export default [
  layout("layouts/protected.tsx", [index("routes/home.tsx")]),

  layout("layouts/public.tsx", [
    route("login", "routes/login.tsx"),
    route("register", "routes/register.tsx"),
  ]),
] satisfies RouteConfig;
