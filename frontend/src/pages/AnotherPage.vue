<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const pills = computed(() => [
  { label: "Current path", value: route.path },
  { label: "Route name", value: String(route.name ?? "unnamed") },
  { label: "Mode", value: "Vue Router history" },
]);
</script>

<template>
  <main class="page-shell">
    <section class="hero">
      <p class="eyebrow">SPA Demo</p>
      <h1>Client-side routing stays intact when Axum falls back to <code>index.html</code>.</h1>
      <p class="lede">
        This page exists to prove that the frontend can navigate to another URL and
        still render correctly after a full refresh in production.
      </p>
    </section>

    <section class="status-grid">
      <article
        v-for="pill in pills"
        :key="pill.label"
        class="status-card"
      >
        <p class="status-label">{{ pill.label }}</p>
        <strong>{{ pill.value }}</strong>
      </article>
    </section>

    <section class="notes card">
      <h2>What to test</h2>
      <p>Open this page directly at <code>/another-page</code>.</p>
      <p>Refresh it in development and in a release build.</p>
      <p>
        If the app still loads, your static asset serving and SPA fallback are wired
        correctly.
      </p>
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
  max-width: 14ch;
  margin: 0;
  font-size: clamp(2rem, 7vw, 4.5rem);
  line-height: 1;
  letter-spacing: -0.04em;
}

.lede {
  max-width: 42rem;
  margin: 1.25rem 0 0;
  font-size: 1.05rem;
  line-height: 1.6;
  color: #314033;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 1rem;
  margin-bottom: 1rem;
}

.card,
.status-card {
  padding: 1.25rem;
  border: 1px solid rgb(20 34 21 / 10%);
  border-radius: 1.25rem;
  background: rgb(255 255 255 / 72%);
  backdrop-filter: blur(10px);
  box-shadow: 0 18px 40px rgb(20 34 21 / 7%);
}

.status-label {
  margin: 0 0 0.5rem;
  font-size: 0.78rem;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: #5f7360;
}

strong {
  font-size: 1.1rem;
}

.notes p {
  margin: 0.75rem 0 0;
  color: #314033;
  line-height: 1.6;
}

.notes p:first-of-type {
  margin-top: 0;
}

@media (max-width: 800px) {
  .page-shell {
    padding: 1.5rem 0 3rem;
  }

  .status-grid {
    grid-template-columns: 1fr;
  }
}
</style>
