import { useState } from "react";
import { ping_server, PingResponse } from "../api/ping_api";
import "./PingAlarm.css"
import { usePingListener, useTimeout } from "../hooks/hooks";
import alarmUrl from "../assets/alarm_disabled.png"
import alarmActiveUrl from "../assets/alarm.gif"

function PingAlarm() {
  const [showNotif, setNotif] = useState(false);
  const [currImg, setImg] = useState(alarmUrl)

  useTimeout(
    () => {
      setNotif(false);
      setImg(alarmUrl);
      console.log("Test");
    },
    showNotif ? 2000 : null // Pass delay only when showNotif is true
  );

  const pingHandler = () => {
    console.log("Pinged!")
    setNotif(true)
    setImg(alarmActiveUrl);
  };

  usePingListener(pingHandler)

  const [message, setMsg] = useState("Test");
  const handlePing = async () => {
    console.log("Ping");
    let response : PingResponse = await ping_server();
    setMsg(`You have been pingged ${response.ping_count} times`);
  }

  return (
    <div className="ping-div">
          <img className="alarm-img" src={currImg}></img>
          <button onClick={handlePing} className="ping-button"> Ping </button>
    </div>
  )
}


export default PingAlarm;
