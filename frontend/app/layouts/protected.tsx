import { Navigate, Outlet, useLocation } from "react-router";
import { DataSync } from "~/components/data-sync";
import { useAuth } from "~/contexts/AuthContext";
import { ReplicacheProvider } from "~/contexts/ReplicacheContext";
import { SSEProvider } from "~/contexts/SSEContext";

const ProtectedRoute = () => {
  const { user, loading } = useAuth();
  const location = useLocation();

  if (loading) return <div>Loading...</div>;

  if (!user) {
    return <Navigate to="/login" state={{ from: location }} replace />;
  }

  return (
    <SSEProvider userId={user.id}>
      <ReplicacheProvider userId={user.id}>
        <DataSync />
        <Outlet />
      </ReplicacheProvider>
    </SSEProvider>
  );
};

export default ProtectedRoute;
