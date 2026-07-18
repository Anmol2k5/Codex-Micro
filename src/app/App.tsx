import { useEffect } from "react";
import { FloatingController } from "../components/controller/FloatingController";
import { Dashboard } from "../components/dashboard/Dashboard";
import { useMicroDeckStore } from "../state/useMicroDeckStore";

export function App() {
  const refresh = useMicroDeckStore((state) => state.refresh);
  const settings = useMicroDeckStore((state) => state.settings);
  const controllerOnly = new URLSearchParams(window.location.search).get("view") === "controller";

  useEffect(() => {
    void refresh();
    const timer = window.setInterval(() => void refresh(), 5000);
    return () => window.clearInterval(timer);
  }, [refresh]);

  useEffect(() => {
    document.documentElement.dataset.reducedMotion = settings.reducedMotion ? "true" : "false";
  }, [settings.reducedMotion]);

  if (controllerOnly) {
    return (
      <div className="controller-window">
        <FloatingController />
      </div>
    );
  }

  return (
    <div className="app-frame app-frame-main">
      <Dashboard />
    </div>
  );
}
