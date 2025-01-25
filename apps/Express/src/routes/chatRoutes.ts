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
                        const input = document.querySelector('input');
                        if (input && input.value.trim()) {
                            const message = {
                                user: 'Anonymous', // You can replace this with a dynamic user identifier
                                text: input.value,
                                email: 'anonymous@example.com', // Replace with dynamic email
                                imageProfile: 'https://example.com/default-profile.png', // Replace with dynamic profile image link
                                imageMessage: 'https://example.com/default-message.png', // Replace with dynamic message image link
                                role: 'guest' // Replace with dynamic role
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
