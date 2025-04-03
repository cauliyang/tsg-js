# TSG Core JS Demo Page

This simple HTML page demonstrates how to use the TSG Core JS library in a web application.

## Usage Instructions

1. First, build the WebAssembly package and prepare the www directory:

```bash
# If you have npm installed:
npm run dev

# Or manually:
cd ..  # Go to the project root
wasm-pack build --target web
mkdir -p www/pkg
cp -r pkg/* www/pkg/
cd www
python -m http.server 8000
```

2. Open your browser and navigate to `http://localhost:8000`

## Troubleshooting

If you encounter the "Failed to fetch dynamically imported module" error:

1. Make sure you've copied the WebAssembly files to the www/pkg directory
2. Check that the files exist in the right location by navigating to http://localhost:8000/pkg/tsg_core_js.js
3. Check browser console for more detailed error messages
4. Try using an incognito/private window to avoid caching issues
5. If using Firefox and getting CORS errors, try Chrome instead

## Features

- Basic function demonstration (greet, add)
- Graph JSON operations
- Simple user interface for testing

## Notes

- The `load_graph` function as implemented requires file system access, which is restricted in browsers. In a real application, you'd need to modify this to work with browser APIs.
- This demo is intended for development and testing purposes.
