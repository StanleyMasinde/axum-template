<template>
  <main class="page-shell">
    <section class="hero">
      <p class="eyebrow">404</p>
      <h1>This route does not exist in the frontend app.</h1>
      <p class="lede">
        The server found the SPA entrypoint, but Vue Router could not match this URL
        to a client-side page.
      </p>
    </section>

    <section class="card-stack">
      <article class="card">
        <h2>Try one of the starter routes</h2>
        <div class="actions">
          <RouterLink to="/">Go Home</RouterLink>
          <RouterLink to="/another-page">Open Another Page</RouterLink>
        </div>
      </article>

      <article class="card">
        <h2>Why this page matters</h2>
        <p>
          It demonstrates the difference between server fallback and client routing:
          Axum can still serve <code>index.html</code> while the SPA shows a proper
          not-found state.
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
  margin-bottom: 2rem;
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
  max-width: 11ch;
  margin: 0;
  font-size: clamp(2.25rem, 7vw, 4.75rem);
  line-height: 0.98;
  letter-spacing: -0.04em;
}

.lede {
  max-width: 42rem;
  margin: 1.25rem 0 0;
  font-size: 1.05rem;
  line-height: 1.6;
  color: #314033;
}

.card-stack {
  display: grid;
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

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.actions :deep(a) {
  display: inline-flex;
  align-items: center;
  padding: 0.75rem 1rem;
  border-radius: 999px;
  text-decoration: none;
  color: #f4f6ef;
  background: #1d311f;
}

@media (max-width: 800px) {
  .page-shell {
    padding: 1.5rem 0 3rem;
  }
}
</style>
