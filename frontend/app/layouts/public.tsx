import { Navigate, Outlet } from "react-router";
import { LoaderCircleIcon } from "lucide-react";
import { useUserStore } from "~/stores/user";

const PublicLayout = () => {
  const user = useUserStore((state) => state.data);
  const loading = useUserStore((state) => state.loading);

  if (loading) {
    return (
      <div className="flex bg-background flex-1 items-center justify-center p-6 min-h-screen">
        <div className="text-center">
          <LoaderCircleIcon className="mx-auto mb-4 h-8 w-8 animate-spin text-primary-500" />
        </div>
      </div>
    );
  }

  if (user) {
    return <Navigate to="/" replace />;
  }

  return <Outlet />;
};

export default PublicLayout;
