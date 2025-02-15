use axum::response::{IntoResponse, Html};
use tracing::instrument; // For logging

#[instrument]
pub async fn homepage() -> impl IntoResponse {
    Html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Welcome to Axium!</title>
            <style>
                body {
                    font-family: 'Arial', sans-serif;
                    background-color: #1e1e2e;
                    color: #ffffff;
                    text-align: center;
                    padding: 40px;
                }
                a {
                    color: #00bcd4;
                    text-decoration: none;
                    font-weight: bold;
                }
                a:hover {
                    text-decoration: underline;
                }
                .container {
                    max-width: 800px;
                    margin: auto;
                    padding: 20px;
                    background: #282a36;
                    border-radius: 8px;
                    box-shadow: 0 0 15px rgba(0, 0, 0, 0.2);
                    text-align: center;
                }
                h1 {
                    font-size: 1.2em;
                    white-space: pre;
                    font-family: monospace;
                }
                .motto {
                    margin-top: 10px;
                    font-size: 1em;
                    font-style: italic;
                }
                ul {
                    list-style-type: none;
                    padding: 0;
                    text-align: left;
                    display: inline-block;
                }
                li {
                    margin: 10px 0;
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
                <p class="motto">An example API built with Rust, Axum, SQLx, and PostgreSQL</p>
                <ul>
                    <li>🚀 Check out all endpoints by visiting <a href="/swagger-ui">Swagger</a>, or import the <a href="/openapi.json">OpenAPI</a> file.</li>
                    <li>⚡ Do not forget to update your Docker Compose configuration with a health check. Just point it to: <a href="/health">/health</a></li>
                </ul>
            </div>
        </body>
        </html>
    "#)
}
