use axum::response::{IntoResponse, Html};

// Homepage route
pub async fn homepage() -> impl IntoResponse {
    Html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Axium API</title>
            <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 80 80'><text x='0' y='60' font-size='64'>🦖</text></svg>">
            <style>
                :root {
                    --neon-cyan: #00f3ff;
                    --dark-space: #0a0e14;
                    --starry-night: #1a1f2c;
                }
                body {
                    font-family: 'Arial', sans-serif;
                    background: linear-gradient(135deg, var(--dark-space) 0%, var(--starry-night) 100%);
                    color:#ffffff;
                    margin: 0;
                    min-height: 100vh;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    line-height: 1.6;
                }
                a {
                    color:#00ffff;
                    text-decoration: none;
                    font-weight: 500;
                    transition: color 0.3s;
                }
                a:hover {
                    color: #40ffa0;
                }
                .container {
                    background: rgba(25, 28, 36, 0.9);
                    backdrop-filter: blur(12px);
                    border-radius: 16px;
                    padding: 2.5rem;
                    max-width: 800px;
                    margin: 2rem;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
                    border: 1px solid rgba(255, 255, 255, 0.1);
                    text-align: center;
                }
                h1 {
                    font-size: 1.2em;
                    white-space: pre;
                    font-family: monospace;
                    display: inline-block;
                    text-align: left;
                    line-height: normal;
                }
                ul {
                    list-style-type: none;
                    padding: 0;
                    text-align: left;
                    display: inline-block;
                    font-size: 1.1em;
                }
                li {
                    margin: 15px 0;
                }
                .github-link {
                    margin-top: 25px;
                    display: inline-flex;
                    align-items: center;
                    padding: 12px 25px;
                    background-color: #00ffff;
                    color: #0f111a;
                    border-radius: 8px;
                    font-weight: bold;
                    transition: background-color 0.3s;
                }
                .github-link:hover {
                    background-color:#40ffa0;
                    color: #ffffff;
                }
                .github-link svg {
                    margin-right: 8px;
                    position: relative;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>
        db                      88                                 
       d88b                     ""                                 
      d8'`8b                                                       
     d8'  `8b      8b,     ,d8  88  88       88  88,dPYba,,adPYba, 
    d8YaaaaY8b      `Y8, ,8P'   88  88       88  88P'   "88"    "8a
   d8""""""""8b       )888(     88  88       88  88      88      88
  d8'        `8b    ,d8" "8b,   88  "8a,   ,a88  88      88      88
 d8'          `8b  8P'     `Y8  88   `"YbbdP'Y8  88      88      88
                </h1>
                <ul>
                    <li>📖 Explore the API using <a href="/docs">Swagger UI</a> or import the <a href="/openapi.json">OpenAPI spec</a>.</li>
                    <li>🩺 Ensure your Docker setup is reliable, by pointing its healthcheck to <a href="/health">/health</a>.</li>
                </ul>
                <a href="https://github.com/Riktastic/Axium" class="github-link" target="_blank">
                    <svg height="20" aria-hidden="true" viewBox="0 0 16 16" version="1.1" width="20" data-view-component="true" fill="currentColor">
                        <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
                    </svg>
                    View on GitHub
                </a>
            </div>
        </body>
        </html>
    "#)
}
