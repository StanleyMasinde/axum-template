<template>
  <main class="page-shell">
    <section class="hero">
      <p class="eyebrow">Home</p>
      <h1>Ship a standalone binary without bundling the frontend into it.</h1>
      <p class="lede">
        Build the UI as static assets, publish them separately, and let the Rust app
        serve or download them when needed.
      </p>
    </section>

    <section class="grid">
      <article class="card">
        <h2>Development</h2>
        <p>
          Run <code>cargo run</code> and the app proxies frontend requests to Vite
          while Axum continues handling your API routes.
        </p>
      </article>

      <article class="card">
        <h2>Release</h2>
        <p>
          Production builds look for assets on disk first, then fetch
          <code>frontend.tar.gz</code> from the matching GitHub Release when needed.
        </p>
      </article>

      <article class="card">
        <h2>Why this shape</h2>
        <p>
          You keep the binary lean, avoid embedding HTML and JS, and can still ship a
          self-contained app experience.
        </p>
      </article>
    </section>
  </main>
</template>

<style scoped>
.page-shell {
  width: min(100%, 1100px);
  margin: 0 auto;
  padding: 2.5rem 0 4rem;
}

.hero {
  margin-bottom: 2.5rem;
}

.eyebrow {
  margin: 0 0 0.75rem;
  font-size: 0.8rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: #385c37;
}

h1 {
  max-width: 12ch;
  margin: 0;
  font-size: clamp(2.5rem, 8vw, 5.5rem);
  line-height: 0.95;
  letter-spacing: -0.04em;
}

.lede {
  max-width: 40rem;
  margin: 1.25rem 0 0;
  font-size: 1.1rem;
  line-height: 1.6;
  color: #314033;
}

.grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 1rem;
}

.card {
  padding: 1.25rem;
  border: 1px solid rgb(20 34 21 / 10%);
  border-radius: 1.25rem;
  background: rgb(255 255 255 / 72%);
  backdrop-filter: blur(10px);
  box-shadow: 0 18px 40px rgb(20 34 21 / 7%);
}

h2 {
  margin: 0 0 0.75rem;
  font-size: 1rem;
}

.card p {
  margin: 0;
  line-height: 1.6;
  color: #314033;
}

@media (max-width: 800px) {
  .page-shell {
    padding: 1.5rem 0 3rem;
  }

  .grid {
    grid-template-columns: 1fr;
  }
}
</style>
