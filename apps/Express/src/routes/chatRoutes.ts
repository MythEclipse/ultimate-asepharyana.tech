import { Application } from 'express';

export function setChatRoutes(app: Application) {
  app.get('/', (_, res) => {
    //     res.send(`<html lang="en">
    // <head>
    //   <meta charset="UTF-8">
    //   <meta name="viewport" content="width=device-width, initial-scale=1.0">
    //   <meta http-equiv="refresh" content="0; url=https://asepharyana.cloud/chat">
    //   <title>Redirecting...</title>
    // </head>
    // <body>
    //   <p>If you are not redirected automatically, <a href="https://asepharyana.cloud/chat">click here</a>.</p>
    // </body>
    // </html>`);
    res.send(`
        <html>
        <body>
          <script>
          const ws = new WebSocket('ws://' + window.location.host);
          ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            const div = document.createElement('div');
            div.textContent = message.userId + ': ' + message.text;
            document.body.appendChild(div);
          };

          document.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
              const input = document.querySelector('input');
              if (input && input.value.trim()) {
                const message = {
                  userId: 'User618', // You can replace this with a dynamic user identifier
                  text: input.value,
                  email: '', // Replace with dynamic email
                  imageProfile: '/profile-circle-svgrepo-com.svg', // Replace with dynamic profile image link
                  imageMessage: '', // Replace with dynamic message image link
                  role: 'user', // Replace with dynamic role
                  timestamp: new Date().toISOString(), // Add timestamp
                  id: '' // Replace with dynamic id
                };
                ws.send(JSON.stringify(message));
                input.value = '';
              }
            }
          });

          document.body.innerHTML = '<input placeholder="Type a message...">';
          </script>
        </body>
        </html>
      `);
  });
}
