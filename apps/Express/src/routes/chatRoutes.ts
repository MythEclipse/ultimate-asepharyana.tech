import { Application } from 'express';

export function setChatRoutes(app: Application) {
  app.get('/', (_, res) => {
    res.send(`
            <html>
            <body>
                <script>
                const ws = new WebSocket('ws://' + window.location.host);
                ws.onmessage = (event) => {
                    const message = JSON.parse(event.data);
                    const div = document.createElement('div');
                    div.textContent = message.user + ': ' + message.text;
                    document.body.appendChild(div);
                };

                document.addEventListener('keydown', (e) => {
                    if (e.key === 'Enter') {
                    const input = document.querySelector('input[type="text"]');
                    const userIdInput = document.querySelector('input[type="text"].user-id');
                    if (input && input.value.trim() && userIdInput && userIdInput.value.trim()) {
                        const message = {
                        text: input.value,
                        userId: userIdInput.value
                        };
                        ws.send(JSON.stringify(message));
                        input.value = '';
                    }
                    }
                });

                document.body.innerHTML = '<input type="text" class="user-id" placeholder="Enter your user ID"><input type="text" placeholder="Type a message...">';
                </script>
            </body>
            </html>
        `);
  });
}
