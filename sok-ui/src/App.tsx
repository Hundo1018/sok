import { useEffect, useRef } from 'react'
import sok from './pkg/sok';
function App() {
  const initialized = useRef(false);

  useEffect(() => {
    // Prevent multiple initializations
    if (!initialized.current) {
      initialized.current = true;
      try {
        sok();
      } catch (error) {
        console.error('Error initializing WASM module:', error);
        initialized.current = false; // Allow retry on error
      }
    }

    // Cleanup function to handle component unmount
    return () => {
      // If your WASM module has a cleanup function, call it here
      // Example: if (window.cleanup_sok) window.cleanup_sok();
    };
  }, []) // Empty dependency array ensures this runs only once

  return (
    <div id="wasm-container">
    </div>
  );
}

export default App