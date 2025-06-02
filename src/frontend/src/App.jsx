import './App.css'
import {Route, Routes, useNavigate } from "react-router-dom";
import Home from "./pages/Home"


function App() {

  return (
    <div>
       <div className="w-screen min-h-screen bg-richblack-900 flex flex-col font-inter">
        <Routes>
          <Route path="/" element={<Home/>}/>
        </Routes>
       </div>
    </div>
  )
}

export default App
