import { Navigate, Outlet, useLocation } from "react-router";
import { LoaderCircleIcon } from "lucide-react";
import { useUserStore } from "~/stores/user";

const PublicLayout = () => {
  const user = useUserStore((state) => state.data);
  const loading = useUserStore((state) => state.loading);
  const location = useLocation();

  if (loading) {
    return (
      <div className="flex min-h-screen flex-1 items-center justify-center bg-background p-6">
        <div className="text-center">
          <LoaderCircleIcon className="mx-auto mb-4 h-8 w-8 animate-spin text-primary-500" />
        </div>
      </div>
    );
  }

  if (
    user &&
    (location.pathname === "/login" || location.pathname === "/register")
  ) {
    return <Navigate to="/" replace />;
  }

  const allowedPaths = ["/login", "/register", "/shared"];

  const isPathAllowed = allowedPaths.some((path) =>
    location.pathname.startsWith(path)
  );

  if (!isPathAllowed) {
    return <Navigate to="/login" replace />;
  }

  return <Outlet />;
};

export default PublicLayout;
