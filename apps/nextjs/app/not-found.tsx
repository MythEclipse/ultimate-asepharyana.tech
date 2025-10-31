export default function NotFound() {
  return (
    <html lang="en">
      <body>
        <div style={{ display: 'flex', minHeight: '100vh', alignItems: 'center', justifyContent: 'center' }}>
          <div style={{ textAlign: 'center' }}>
            <h1 style={{ marginBottom: '1rem', fontSize: '4rem', fontWeight: 'bold' }}>404</h1>
            <p style={{ marginBottom: '1rem', fontSize: '1.25rem' }}>Page Not Found</p>
            <a href="/" style={{ color: 'blue', textDecoration: 'underline' }}>
              Go back home
            </a>
          </div>
        </div>
      </body>
    </html>
  );
}
