'use client';

export default function Error() {
  return (
    <div style={{ padding: '20px', textAlign: 'center', minHeight: '100vh', display: 'flex', flexDirection: 'column', justifyContent: 'center', alignItems: 'center' }}>
      <h2 style={{ fontSize: '2rem', marginBottom: '1rem' }}>Something went wrong!</h2>
      <a href="/" style={{ padding: '10px 20px', fontSize: '1rem', textDecoration: 'underline', color: 'blue' }}>Go home</a>
    </div>
  );
}
