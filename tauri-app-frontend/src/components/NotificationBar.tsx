import { useState, useEffect } from "react";
import { useTimeout } from "../hooks/use_timeout";
import "./NotificationBar.css"

function Notification({ onExpire, timeout }: { onExpire: () => void; timeout: number }) {
  useTimeout(onExpire, timeout);
  return <div className="notification">Notif</div>;
}

function NotificationBar() {
  const [pings, setPings] = useState<number[]>([]);
  const serverUrl = "http://localhost:7878/ping/listen";

  useEffect(() => {
    const evListener = new EventSource(serverUrl);

    evListener.addEventListener("ping", () => {
      setPings((prev) => {
        const nextData = [...prev, Date.now()];
        console.log("Pinged! " + nextData.length);
        return nextData;
      });
    });

    return () => evListener.close();
  }, []);

  const handleExpire = (id: number) => {
    console.log("expire uwuw");
    setPings((prev) => prev.filter((pingId) => pingId !== id));
  };

  return (
    <div className="notification-bar">
      {pings.map((id) => (
        <Notification
          key={id}
          onExpire={() => handleExpire(id)}
          timeout={2000}
        />
      ))}
    </div>
  );
}

export default NotificationBar;
