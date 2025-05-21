import  { useEffect } from 'react'
import sok from './pkg/sok'
import './App.css'

function App() {
  useEffect(() => { 
    sok
  }, [])
  return (
    <div>
      <canvas id="wasm-sok" width={800} height={600}></canvas>
    </div>
  );
}

export default App
