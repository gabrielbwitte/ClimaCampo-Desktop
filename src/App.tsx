import { useState } from "react";
import { Home } from "./components/Home";
import { Login } from "./components/Login";
import "./app.css";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

function App() {
 
  const [isLogin, setIsLogin] = useState(false);
  const [err, setErr] = useState("");
  
  const handleForm = (data: any) => {
        setErr("");
    invoke("login_ipc", {username: data.username, password: data.password})
      .then((msg) => {
        if (msg === 200) {
          setIsLogin(true)
        } else if (msg === 401) {
          setErr("Usuário ou senha incorretos.")
        }
      })
      .catch((err) => setErr(err))
  }

  listen<number>('logoff_command_ipc', (event) => {
    if(event.payload === 401) {
      setIsLogin(false);
    }
  })

  return (
    <main>
      {
        isLogin ? <Home /> : <Login LoginForm={handleForm} error={err}/>
      }
    </main>
  );
}

export default App;
