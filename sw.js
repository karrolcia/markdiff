const CACHE = 'markdiff-v3';
const ASSETS = [
  './',
  './index.html',
  'https://cdn.jsdelivr.net/npm/marked/marked.min.js',
  'https://cdn.jsdelivr.net/npm/diff/dist/diff.min.js',
  'https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap'
];

self.addEventListener('install', (e) => {
  e.waitUntil(caches.open(CACHE).then(c => c.addAll(ASSETS)));
  self.skipWaiting();
});

self.addEventListener('activate', (e) => {
  e.waitUntil(
    caches.keys().then(keys =>
      Promise.all(keys.filter(k => k !== CACHE).map(k => caches.delete(k)))
    )
  );
  self.clients.claim();
});

self.addEventListener('fetch', (e) => {
  const url = new URL(e.request.url);

  // Handle Web Share Target: files shared to the app on mobile
  if (e.request.method === 'POST' && url.pathname.endsWith('/')) {
    e.respondWith((async () => {
      const formData = await e.request.formData();
      const file = formData.get('file');
      // Store the shared file so the page can pick it up
      const cache = await caches.open('markdiff-shared');
      if (file) {
        const text = await file.text();
        await cache.put('/_shared_file', new Response(JSON.stringify({
          name: file.name,
          content: text
        }), { headers: { 'Content-Type': 'application/json' } }));
      }
      // Redirect to the app
      return Response.redirect('./?shared=1', 303);
    })());
    return;
  }

  if (url.origin === location.origin) {
    // Network first for local files
    e.respondWith(
      fetch(e.request).then(r => {
        const clone = r.clone();
        caches.open(CACHE).then(c => c.put(e.request, clone));
        return r;
      }).catch(() => caches.match(e.request))
    );
  } else {
    // Cache first for CDN
    e.respondWith(
      caches.match(e.request).then(r => r || fetch(e.request))
    );
  }
});
