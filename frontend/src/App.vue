<script setup lang="ts"></script>

<template>
  <div class="app-shell">
    <header class="topbar">
      <div class="brand">
        <p class="eyebrow">Axum + Vue Template</p>
        <p class="brand-copy">Standalone binary, external frontend assets.</p>
      </div>

      <nav class="nav">
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/another-page">Another Page</RouterLink>
      </nav>
    </header>

    <RouterView />
  </div>
</template>

<style scoped>
:global(body) {
  margin: 0;
  font-family:
    "IBM Plex Sans",
    "Segoe UI",
    sans-serif;
  background:
    radial-gradient(circle at top, rgb(213 232 255 / 90%), transparent 45%),
    linear-gradient(180deg, #f4f6ef 0%, #e8ece4 100%);
  color: #142215;
}

:global(code) {
  font-family:
    "IBM Plex Mono",
    "SFMono-Regular",
    monospace;
}

:global(*) {
  box-sizing: border-box;
}

.app-shell {
  min-height: 100vh;
  padding: 1.25rem;
}

.topbar {
  width: min(100%, 1100px);
  margin: 0 auto 1.5rem;
  padding: 1rem 1.25rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  border: 1px solid rgb(20 34 21 / 10%);
  border-radius: 1.25rem;
  background: rgb(255 255 255 / 72%);
  backdrop-filter: blur(10px);
  box-shadow: 0 18px 40px rgb(20 34 21 / 7%);
}

.brand-copy {
  margin: 0.25rem 0 0;
  color: #314033;
}

.nav {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.nav :deep(a) {
  display: inline-flex;
  align-items: center;
  padding: 0.65rem 0.95rem;
  border-radius: 999px;
  text-decoration: none;
  color: #1d311f;
  background: rgb(255 255 255 / 55%);
  border: 1px solid rgb(20 34 21 / 10%);
  transition:
    transform 150ms ease,
    background-color 150ms ease;
}

.nav :deep(a:hover) {
  transform: translateY(-1px);
  background: rgb(255 255 255 / 88%);
}

.nav :deep(a.router-link-exact-active) {
  background: #1d311f;
  color: #f4f6ef;
}

.eyebrow {
  margin: 0;
  font-size: 0.8rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: #385c37;
}

@media (max-width: 800px) {
  .app-shell {
    padding: 1rem;
  }

  .topbar {
    align-items: flex-start;
    flex-direction: column;
  }
}
</style>
