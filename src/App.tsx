import { useState } from "react";
import { Home } from "./components/Home";
import { Login } from "./components/Login";
import "./app.css";


function App() {
  const [isLogin, setIsLogin] = useState(false);
  const handleDataLogin = (status: any) => {
    console.log(status)
    if (status === 200) {
      setIsLogin(true)
    }
  }

  return (
    <main>
      {
        isLogin ? <Home /> : <Login onStatus={handleDataLogin}/>
      }
    </main>
  );
}

export default App;
