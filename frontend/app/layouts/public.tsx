import { Navigate, Outlet } from "react-router";
import { useAuth } from "~/contexts/AuthContext";
import { LoaderCircleIcon } from "lucide-react";

const PublicLayout = () => {
  const { user, loading } = useAuth();

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
