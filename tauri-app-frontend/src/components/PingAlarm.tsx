import { useState, useEffect } from "react";
import { ping_server, PingResponse } from "../api/ping_api";
import "./PingAlarm.css"
import { useTimeout } from "../hooks/use_timeout";



function PingAnimation() {
  return (
    <div className="ping-notification">PINGGED!!!</div>
  )
}

function PingAlarm() {
  const [showNotif, setNotif] = useState(false);
  const [serverUrl, setServerUrl] = useState('http://localhost:7878/ping/listen');


  useTimeout(() => {
    setNotif(false);
    console.log("Test");
  }, showNotif ? 2000 : null);

  useEffect(() => {
    const evListener = new EventSource(serverUrl)
    evListener.addEventListener("ping", () => {
      console.log("Pinged!")
      setNotif(true)
    })
  	return () => {
     evListener.close()
  	};
  }, [serverUrl]);

  const [message, setMsg] = useState("Test");
  const handlePing = async () => {
    console.log("Ping");
    let response : PingResponse = await ping_server();
    setMsg(`Ping ${response.status} count ${response.ping_count}`);
  }

  return (
    <div className="ping-div">
      <header id="notifications">
        {showNotif && <PingAnimation />}
      </header>
      <div className="alarm-content">
        <div>
          {message}
        </div>
        <div>
          <button onClick={handlePing} className="ping-button"> Ping </button>
        </div>
        </div>
    </div>
  )
}


export default PingAlarm;
